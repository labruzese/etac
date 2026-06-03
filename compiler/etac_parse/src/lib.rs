use etac_errors::{error, Diagnostic};
use etac_lexer::{Token};
use etac_span::Span;
use lalrpop_util::{lalrpop_mod, ParseError};

lalrpop_mod!(grammar);

mod tests;

pub trait IParser<ParseOut> {
    fn new() -> Self;

    fn parse<__TOKEN, __TOKENS>(
        &self,
        errors: &mut Vec<lalrpop_util::ErrorRecovery<usize, Token, Diagnostic>>,
        __tokens0: __TOKENS,
    ) -> Result<ParseOut, lalrpop_util::ParseError<usize, Token, Diagnostic>>
    where
        __TOKEN: grammar::__ToTriple,
        __TOKENS: IntoIterator<Item = __TOKEN>;
}

macro_rules! impl_iparser {
    ($parser:ty, $out:ty) => {
        impl IParser<$out> for $parser {
            fn new() -> Self { <$parser>::new() }

            fn parse<__TOKEN, __TOKENS>(
                &self,
                errors: &mut Vec<lalrpop_util::ErrorRecovery<usize, Token, Diagnostic>>,
                __tokens0: __TOKENS,
            ) -> Result<$out, lalrpop_util::ParseError<usize, Token, Diagnostic>>
            where
                __TOKEN: grammar::__ToTriple,
                __TOKENS: IntoIterator<Item = __TOKEN> {
                <$parser>::parse(self, errors, __tokens0)
            }
        }
    };
}

pub use grammar::ProgramParser;
pub use grammar::InterfaceParser;
impl_iparser!{ProgramParser, etac_ast::Program}
impl_iparser!{InterfaceParser, etac_ast::Interface}

#[derive(Debug)]
pub enum ParseResult<Out> {
    Clean(Out),
    WithDiags { out: Out, diags: Vec<Diagnostic>},
    FatalError(Vec<Diagnostic>),
}

pub fn parse<
    Out,
    Lexer,
    Parser,
>(
    lexer: &mut Lexer,
) -> ParseResult<Out> 
where
    Lexer: Iterator<Item = Result<(usize, Token, usize), Diagnostic>>,
    Parser: IParser<Out>,
{
    let mut recovered = Vec::new();
    let result = Parser::new()
                    .parse(&mut recovered, lexer)
                    .map_err(|e| to_diag(e));

    let recovered_iter = recovered.into_iter().map(|r| to_diag(r.error));
    match result {
        Ok(out) => {
            let errors: Vec<_> = recovered_iter.collect();
            if errors.is_empty() {
                ParseResult::Clean(out)
            } else {
                ParseResult::WithDiags { out, diags: errors }
            }
        }
        Err(e)  => ParseResult::FatalError(recovered_iter.chain(std::iter::once(e)).collect())
    }
}

fn to_diag(err: ParseError<usize, Token, Diagnostic>) -> Diagnostic {
    use ParseError::*;
    match err {
        User { error } => error,

        UnrecognizedToken {
            token: (s, t, e),
            expected,
        } => error!(Span::new(s, e); "Unexpected token {t}")
            .with_primary_label(format_expected(&expected)),

        UnrecognizedEof { location, expected } => {
            error!(Span::new(location, location); "Unexpected end of file")
                .with_primary_label(format_expected(&expected))
        }

        ExtraToken { token: (s, t, e) } => {
            error!(Span::new(s, e); "Extra token {} after program", t).with_primary_label("unexpected")
        }

        InvalidToken { location: _ } => {
            unreachable!("external lexer; lalrpop can not recieve an invalid token")
        }
    }
}

fn format_expected(expected: &[String]) -> String {
    match expected.len() {
        0 => "expected nothing".into(),
        1 => format!("expected {}", expected[0]),
        _ => {
            let (last, rest) = expected.split_last().unwrap();
            format!("expected one of {}, or {}", rest.join(", "), last)
        }
    }
}
