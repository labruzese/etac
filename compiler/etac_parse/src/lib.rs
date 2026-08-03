use lalrpop_util::{lalrpop_mod, ParseError};
use etac_source::Span;
use etac_errors::{
    etac_error,
    dcx::{Diag, DiagCtx}
}; 
use etac_lexer::{
    EtaLexer, 
    token::{RawToken, TokenDisplay}
};

use crate::{
    result::{ParseResult, Parsed},
    state::ParseState
};

mod state;
pub mod result;

// -- import grammar --

lalrpop_mod!(grammar);
type LalrpopFailure<'dcx> = ParseError<u32, RawToken, Diag<'dcx>>;

// -- parser interface --

pub trait IParser<'src, 'dcx> {
    type Out;

    fn parse(self, lexer: EtaLexer<'src, 'dcx>) -> ParseResult<'src, 'dcx, Self::Out>;

    fn diagnostic_context(&self) -> &'dcx DiagCtx;
}

/// easily implement IParser from lalrpop parse fn
fn parse_generic<'src, 'dcx, LalrpopParser, LalrpopParseFn, Out> (
    parser: LalrpopParser,
    parse_inner: LalrpopParseFn,
    mut pstate: ParseState<'dcx>,
    mut lexer: EtaLexer<'src, 'dcx>
) -> ParseResult<'src, 'dcx, Out> 
where 
    LalrpopParseFn: Fn(&LalrpopParser, &mut ParseState<'dcx>, &mut EtaLexer<'src, 'dcx>) -> Result<Out, LalrpopFailure<'dcx>> 
{
    let parse = parse_inner(&parser, &mut pstate, &mut lexer);
    let symbols = lexer.into_interner_mut();
    let mut recovered = false;
    for e in pstate.lalrpop_errs {
        let diag = to_diag(pstate.dcx, symbols, e.error);
        if diag.level() == etac_errors::Level::Error {
            recovered = true
        }
        pstate.etac_errs.push(diag)
    }
    let parsed = match (parse, recovered) {
        (Ok(ast),false) => Parsed::Ok(ast),
        (Ok(ast),true) => Parsed::Recovered(ast),
        (Err(fatal), _) => {
            pstate.etac_errs.push(to_diag(pstate.dcx, symbols, fatal));
            Parsed::Failed
        }
    };
    let diags = pstate.etac_errs;
    ParseResult {
        parse: parsed,
        diags,
        symbols,
    }
}

/// syntax:
/// `
/// impl_iparser! { 
///     parser: grammar::Parser,
///     output: OutputTy,
/// }
/// `
///
/// *note*: parser needs to be qualified (ex: `path::Parser`)
///
/// writes the following:
///     * `struct $parser`
///     * `fn new(&'dcx DiagCtx) -> $parser`
///     * `impl IParser for $name`
macro_rules! impl_iparser {
    (parser: $($seg:ident)::+, output: $out:ty $(,)?) => {
        impl_iparser!(@inner ($($seg)::+) ($($seg)::+) $out);
    };
    // strip the leading segment and keep going.
    (@inner ($full:path) ($head:ident :: $($rest:ident)::+) $out:ty) => {
        impl_iparser!(@inner ($full) ($($rest)::+) $out);
    };
    (@inner ($full:path) ($name:ident) $out:ty) => {
        pub struct $name<'dcx> {
            state: ParseState<'dcx>
        }
        impl<'src, 'dcx> $name<'dcx> {
            pub fn new(diagc: &'dcx DiagCtx) -> Self {
                $name { state: ParseState::new(diagc) }
            }
        }
        impl<'src, 'dcx> IParser<'src, 'dcx> for $name<'dcx> {
            type Out = $out;

            /// basically just defer to parse_generic by extracting lalrpop parse fn
            fn parse(self, lexer: EtaLexer<'src, 'dcx>) -> ParseResult<'src, 'dcx, Self::Out> {
                let parser = <$full>::new();
                parse_generic(
                    parser,
                    |p, s, l| <$full>::parse(p, s, l),
                    self.state,
                    lexer,
                )
            }

            fn diagnostic_context(&self) -> &'dcx DiagCtx {
                self.state.dcx
            }
        }
    };
}

impl_iparser! {
    parser: grammar::ProgramParser, 
    output: etac_ast::ast::Program
}
impl_iparser! {
    parser: grammar::InterfaceParser,
    output: etac_ast::ast::Interface
}

// -- helpers --

/// LALRPOP error to [`Diag`]
fn to_diag<'dcx>(
    diagc: &'dcx DiagCtx,
    res: &impl etac_intern::Resolver,
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
                diagc, Span::new(s, e), "unexpected token `{}`", TokenDisplay::new(t, res);
                primary: "{}", format_expected(&expected)
            }
        }
        UnrecognizedEof { location, expected } => {
            etac_error! {
                diagc, Span::new(location, location), "unexpected end of file";
                primary: "{}", format_expected(&expected)
            }
        }
        ExtraToken { token: (s, t, e) } => {
            etac_error! {
                diagc, Span::new(s, e), "extra token `{}`", TokenDisplay::new(t, res);
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
