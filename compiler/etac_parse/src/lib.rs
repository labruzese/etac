use etac_ast::{Expr, ExprKind, LValue, LValueKind, NodeIdGen};
use etac_errors::{etac_error, DiagCtxt, Diag, ErrorGuaranteed};
use etac_lexer::Token;
use etac_span::Span;
use lalrpop_util::{lalrpop_mod, ErrorRecovery, ParseError};

lalrpop_mod!(grammar);

/// Mutable state threaded through every grammar action.
///
/// Bundles the [`NodeIdGen`] that hands out node ids with the buffer lalrpop fills
/// on error recovery, so the grammar carries a single parameter instead of two.
pub struct ParseState<'dcx, 'src> {
    pub diagc: &'dcx DiagCtxt<'src>,
    pub ids: NodeIdGen,
    pub errors: Vec<ErrorRecovery<u32, Token, Diag<'dcx, 'src>>>,
}

impl<'dcx, 'src> ParseState<'dcx, 'src> {
    #[must_use]
    pub fn new(diagnostic_context: &'dcx DiagCtx<'src>) -> Self {
        ParseState {
            diagc: diagnostic_context,
            ids: NodeIdGen::default(),
            errors: Vec::new(),
        }
    }
}

pub trait IParser<ParseOut> {
    fn new() -> Self;

    /// # Errors
    /// Error produced by the lalrpop parser
    fn parse<__TOKEN, __TOKENS, 'dcx, 'src>(
        &self,
        state: &mut ParseState,
        __tokens0: __TOKENS,
    ) -> Result<ParseOut, lalrpop_util::ParseError<u32, Token, Diag<'dcx, 'src>>>
    where
        __TOKEN: grammar::__ToTriple,
        __TOKENS: IntoIterator<Item = __TOKEN>;
}

macro_rules! impl_iparser {
    ($parser:ty, $out:ty) => {
        impl IParser<$out> for $parser {
            fn new() -> Self { <$parser>::new() }

            fn parse<__TOKEN, __TOKENS, 'dcx, 'src>(
                &self,
                state: &mut ParseState,
                __tokens0: __TOKENS,
            ) -> Result<$out, lalrpop_util::ParseError<u32, Token, Diag<'dcx, 'src>>>
            where
                __TOKEN: grammar::__ToTriple,
                __TOKENS: IntoIterator<Item = __TOKEN> {
                <$parser>::parse(self, state, __tokens0)
            }
        }
    };
}

pub use grammar::ProgramParser;
pub use grammar::InterfaceParser;
impl_iparser!{ProgramParser, etac_ast::Program}
impl_iparser!{InterfaceParser, etac_ast::Interface}

/// Outcome of a parse. Every diagnostic has already been emitted through the
/// [`DiagCtxt`] by the time this is returned — the caller never receives a `Vec` of
/// diagnostics to drain. The retained `first_error` exists only so the `.parsed` log
/// can record the first syntactic error for a file, and the [`ErrorGuaranteed`] is the
/// proof that the failure was reported.
#[derive(Debug)]
pub enum Parsed<Out> {
    /// Parsed cleanly, no errors.
    Ok(Out),
    /// lalrpop recovered from one or more errors but still produced a full tree.
    Recovered(Out),
    /// Parsing hit a fatal error and produced no tree.
    Failed(ErrorGuarenteed),
}

impl<Out> Parsed<Out> {
    /// The parsed tree, if one was produced ([`Ok`](Parsed::Ok) or
    /// [`Recovered`](Parsed::Recovered)).
    pub fn output(&self) -> Option<&Out> {
        match self {
            Parsed::Ok(out) | Parsed::Recovered(out) => Some(out),
            Parsed::Failed { .. } => None,
        }
    }
}

/// Parse `lexer`'s tokens with `Parser`, routing every diagnostic through `dcx`.
///
/// lalrpop's recovered errors are emitted in source order, then any fatal error. The
/// first error is cloned and retained in the result purely for `.parsed` logging; the
/// caller inspects the [`Parsed`] variant (not a diagnostic list) to decide what to do.
pub fn parse<Out, Lexer, Parser, 'dcx, 'src>(dcx: &'dcx DiagCtxt<'src>, lexer: &mut Lexer) -> Parsed<Out>
where
    Lexer: Iterator<Item = Result<(u32, Token, u32), Diag<'dcx, 'src>>>,
    Parser: IParser<Out>,
{
    let mut state = ParseState::new(dcx);
    let result = Parser::new().parse(&mut state, lexer);

    let mut recovered = false;
    for r in state.errors {
        let diag = to_diag(dcx, r.error);
        if diag.level == etac_errors::Level::Error {
            recovered = true;
        }
        // This guar matches the ones claimed in lalrpop
        let _guar = dcx.emit(diag);
    }

    match result {
        Ok(out) => match recovered {
            false => Parsed::Ok(out),
            true  => Parsed::Recovered(out),
        },
        Err(fatal) => {
            let diag = to_diag(dcx, fatal);
            let guar = diag.emit();
            Parsed::Failed(guar)
        }
    }
}

/// Reinterpret a parsed [`LValue`] as the equivalent [`Expr`], minting fresh ids for
/// the rebuilt carrier. The AST models the array operand of an indexed lvalue
/// (`a[i]`) as an `Expr`, so the grammar funnels the accumulated base through here
/// when folding postfix `[..]` groups.
pub(crate) fn lvalue_to_expr(lv: LValue, ids: &mut NodeIdGen) -> Expr {
    let kind = match lv.kind {
        LValueKind::Id(id) => ExprKind::Id(id),
        LValueKind::ProcCall(pc) => ExprKind::Call(pc),
        LValueKind::Index { array, index } => ExprKind::Index { array, index },
    };
    Expr::new(ids.fresh(), lv.span, kind)
}

fn to_diag<'dcx, 'src>(diagc: &'dcx DiagCtxt, err: ParseError<u32, Token, Diag<'dcx, 'src>>) -> Diag<'dcx, 'src> {
    use ParseError::*;
    match err {
        User { error } => error,

        UnrecognizedToken {
            token: (s, t, e),
            expected,
        } => {
            etac_error! {
                diagc, Span::new(s, e), "Unexpected token {t}";
                primary: "{}", format_expected(&expected)
            }
        }
        UnrecognizedEof { location, expected } => {
            etac_error! {
                diagc, Span::new(location, location), "Unexpected end of file";
                primary: "{}", format_expected(&expected)
            }
        }
        ExtraToken { token: (s, t, e) } => {
            etac_error! {
                diagc, Span::new(s, e), "Extra token {t} after program";
                primary: "unexpected"
            }
        }

        InvalidToken { location: _ } => {
            unreachable!("external lexer; lalrpop can not recieve an invalid token")
        }
    }
}

fn format_expected(expected: &[String]) -> String {
    match expected.len() {
        0 => String::from("expected nothing"),
        1 => format!("expected {}", expected[0]),
        _ => {
            let (last, rest) = expected.split_last().unwrap();
            format!("expected one of {}, or {}", rest.join(", "), last)
        }
    }
}
