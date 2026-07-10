use etac_errors::Diag;
use etac_lexer::ILexer;
use etac_parse::IParser;
use etac_session::logger::{lex::TeeLexer, parse::TeeParser};
use etac_span::SourceCache;

/// A wrapper that holds one of the possible lexers that etac can have
pub enum ULexer<'src, C: SourceCache, I> {
    Raw(I),
    Tee(TeeLexer<'src, C, I>),
}

impl<'dcx, 'src, C: SourceCache + 'dcx, I: ILexer<'dcx, 'src, C>> ILexer<'dcx, 'src, C> for ULexer<'src, C, I> {}

impl<'dcx, 'src, C: SourceCache + 'dcx, I: ILexer<'dcx, 'src, C>> Iterator for ULexer<'src, C, I> {
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            ULexer::Raw(lexer) => lexer.next(),
            ULexer::Tee(lexer) => lexer.next(),
        }
    }
}


/// A wrapper that holds one of the possible parsers that etac can have
pub enum UParser<'src, C: SourceCache, I> {
    Raw(I),
    Tee(TeeParser<'src, C, I>),
}

impl<'dcx, 'src, C: SourceCache + 'dcx, I> IParser<'dcx, 'src, C> for UParser<'src, C, I>
where
    I: IParser<'dcx, 'src, C>,
    I::Out: std::fmt::Display,
{
    type Out = I::Out;

    fn parse(&mut self, lexer: &mut impl ILexer<'dcx, 'src, C>) -> etac_parse::Parsed<Self::Out> {
        match self {
            UParser::Raw(parser) => parser.parse(lexer),
            UParser::Tee(parser) => parser.parse(lexer),
        }
    }

    fn errors_mut(&mut self) -> &mut [Diag<'dcx, C>] {
        match self {
            UParser::Raw(parser) => parser.errors_mut(),
            UParser::Tee(parser) => parser.errors_mut(),
        }
    }

    fn into_errors(self) -> Vec<Diag<'dcx, C>> {
        match self {
            UParser::Raw(parser) => parser.into_errors(),
            UParser::Tee(parser) => parser.into_errors(),
        }
    }

    fn diagnostic_context(&self) -> &'dcx etac_errors::DiagCtxt<C> {
        match self {
            UParser::Raw(parser) => parser.diagnostic_context(),
            UParser::Tee(parser) => parser.diagnostic_context(),
        }
    }
}
