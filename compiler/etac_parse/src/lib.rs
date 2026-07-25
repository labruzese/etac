use etac_source::SourceMap;

use etac_errors::{etac_error, Diag, DiagCtxt};
use etac_intern::{Symbol, InternReader};
use etac_lexer::{ILexer, RawToken, SourceToken};
use lalrpop_util::{lalrpop_mod, ErrorRecovery, ParseError};

lalrpop_mod!(grammar);

/// Outcome of a parse. Every diagnostic has already been emitted through the
/// [`DiagCtxt`] by the time this is returned — the caller never receives a `Vec` of
/// diagnostics to drain. 
#[derive(Debug)]
pub enum Parsed<Out> {
    /// Parsed cleanly, no errors.
    Ok {
        ast: Out,
        symbols: InternReader,
    },
    /// lalrpop recovered from one or more errors but still produced a full tree.
    Recovered {
        ast: Out,
        symbols: InternReader,
    },
    /// Parsing hit a fatal error and produced no tree.
    Failed,
}

impl<Out> Parsed<Out> {
    /// The parsed tree, if one was produced ([`Ok`](Parsed::Ok) or
    /// [`Recovered`](Parsed::Recovered)).
    pub fn output(&self) -> Option<&Out> {
        match self {
            Parsed::Ok{ ast, ..} | Parsed::Recovered{ ast, ..} => Some(ast),
            Parsed::Failed => None,
        }
    }
}

/// Mutable state threaded through every grammar action.
pub(crate) struct ParseState<'dcx, 'src> {
    pub diagc: &'dcx DiagCtxt<'dcx>,
    pub cache: &'dcx EtaCache,
    pub lalrpop_errs: Vec<ErrorRecovery<u32, SourceToken, Diag<'dcx>>>,
    pub etac_errs: Vec<Diag<'dcx>>,
}

impl<'dcx, 'src> ParseState<'dcx, 'src> {
    #[must_use]
    pub fn new(diagnostic_context: &'dcx DiagCtxt<'dcx>) -> Self {
        ParseState {
            diagc: diagnostic_context,
            cache: diagnostic_context.cache(),
            lalrpop_errs: Vec::new(),
            etac_errs: Vec::new(),
        }
    }
}

pub use grammar::__ToTriple;

pub trait IParser<'dcx, 'src> {
    type Out;

    fn parse(&mut self, lexer: impl ILexer<'src, 'dcx>) -> Parsed<Self::Out>;

    fn errors_mut(&mut self) -> &mut [Diag<'dcx>];

    fn into_errors(self) -> Vec<Diag<'dcx>>;

    fn diagnostic_context(&self) -> &'dcx DiagCtxt<'dcx>;
}

/// Creates a new struct shadowing the name of the passed on (you must qualify a path ex:
/// [`grammar::ProgramParser`]) and implements [`IParser`] for it.
macro_rules! impl_iparser {
    ($($seg:ident)::+, $out:ty) => {
        impl_iparser!(@inner ($($seg)::+) ($($seg)::+) $out);
    };
    // strip the leading segment and keep going.
    (@inner ($full:path) ($head:ident :: $($rest:ident)::+) $out:ty) => {
        impl_iparser!(@inner ($full) ($($rest)::+) $out);
    };
    (@inner ($full:path) ($name:ident) $out:ty) => {
        pub struct $name<'dcx, 'src> {
            state: ParseState<'dcx, 'src>
        }
        impl<'dcx, 'src> $name<'dcx, 'src> {
            #[must_use]
            pub fn new(diagc: &'dcx DiagCtxt<'dcx>) -> Self {
                $name { state: ParseState::new(diagc) }
            }
        }
        impl<'dcx, 'src> IParser<'dcx, 'src> for $name<'dcx, 'src> {
            type Out = $out;

            fn parse(&mut self, lexer: &mut impl ILexer<'src, 'dcx>) -> Parsed<Self::Out> {
                let parse = <$full>::parse(&<$full>::new(), &mut self.state, &mut lexer);
                let mut recovered = false;
                for e in std::mem::take(&mut self.state.lalrpop_errs) {
                    let diag = to_diag(self.diagnostic_context(), e.error);
                    if diag.level == etac_errors::Level::Error {
                        recovered = true
                    }
                    self.state.etac_errs.push(diag)
                }
                let symbols = lexer.symbols();
                match (parse, recovered) {
                    (Ok(ast),false) => Parsed::Ok { ast, symbols },
                    (Ok(ast),true) => Parsed::Recovered { ast, symbols },
                    (Err(fatal), _) => {
                        self.state.etac_errs.push(to_diag(self.diagnostic_context(), fatal));
                        Parsed::Failed
                    }
                }
            }

            fn errors_mut(&mut self) -> &mut [Diag<'dcx>] {
                &mut self.state.etac_errs
            }

            fn into_errors(self) -> Vec<Diag<'dcx>> {
                self.state.etac_errs
            }

            fn diagnostic_context(&self) -> &'dcx DiagCtxt<'dcx> {
                self.state.diagc
            }
        }
    };
}

impl_iparser! {grammar::ProgramParser, etac_ast::Program}
impl_iparser! {grammar::InterfaceParser, etac_ast::Interface}

pub(crate) fn lvalue_to_expr(lv: LValue, cache: &EtaCache) -> Expr {
    let kind = match lv.kind {
        LvalueKind::Id(id) => ExprKind::Id(id),
        LvalueKind::ProcCall(pc) => ExprKind::Call(pc),
        LvalueKind::Index { array, index } => ExprKind::Index { array, index },
    };
    Expr::new(cache.dup_span(lv.node_id), kind)
}

/// LALRPOP error to [`Diag`]
fn to_diag<'dcx, 'src>(
    diagc: &'dcx DiagCtxt,
    err: ParseError<u32, RawToken, Diag<'dcx>>,
) -> Diag<'dcx> {
    use ParseError::*;
    match err {
        User { error } => error,

        UnrecognizedToken {
            token: (s, t, e),
            expected,
        } => {
            etac_error! {
                diagc, Span::new(s, e), "Unexpected token {}", t;
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
                diagc, Span::new(s, e), "Extra token {} after program", t;
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
