//! The Eta parser.
//!
//! Hand-written recursive descent.
//!
//! The parser always produces a tree. Errors are reported to the diagnostic context as they are found, and recovered
//! from: an error node stands in for what could not be parsed, carrying proof that it was reported.

use std::io::Write;

use etac_ast::display::{sexpr::SexpPrinter, span::SpanPrinter};
use etac_errors::dcx::DiagCtx;
use etac_intern::EtacInterner;
use etac_lexer::Tokens;
use etac_source::SourceMap;

use crate::parser::Parser;
use crate::result::ParseResult;

mod grammar;
mod parser;
mod token_set;

pub mod debug;
pub mod result;

/// A syntactic category that can be parsed from a token stream.
///
/// Implemented for the two file kinds, so a caller can write
/// `Program::parse(&tokens, &dcx)` or be generic over the two.
pub trait Parse: Sized {
    /// Parse `tokens`, reporting syntax errors to `dcx` as they are found.
    fn parse(tokens: &Tokens, dcx: &DiagCtx) -> ParseResult<Self>;

    /// Write the tree as an s-expression, then every node's span against the source.
    ///
    /// This is the body of a [`debug::write_parse`] dump.
    fn write_tree(&self, out: &mut impl Write, sm: &SourceMap, symbols: &EtacInterner);
}

macro_rules! impl_parse {
    ($($ty:ty => $entry:path),+) => {
        $(impl Parse for $ty {
            fn parse(tokens: &Tokens, dcx: &DiagCtx) -> ParseResult<Self> {
                let mut p = Parser::new(tokens, dcx);
                let tree = $entry(&mut p);
                ParseResult { tree, errors: p.finish_parse().or(tokens.errors) }
            }

            fn write_tree(&self, out: &mut impl Write, sm: &SourceMap, symbols: &EtacInterner) {
                let _ = writeln!(out, "{}", SexpPrinter::new(symbols, self));
                let _ = writeln!(out, "{}", SpanPrinter::new(self, sm));
            }
        })+
    };
}

impl_parse! {
    etac_ast::ast::Program => grammar::program,
    etac_ast::ast::Interface => grammar::interface
}

/// Parse an `.eta` file.
pub fn parse_program(tokens: &Tokens, dcx: &DiagCtx) -> ParseResult<etac_ast::ast::Program> {
    etac_ast::ast::Program::parse(tokens, dcx)
}

/// Parse an `.eti` file.
pub fn parse_interface(tokens: &Tokens, dcx: &DiagCtx) -> ParseResult<etac_ast::ast::Interface> {
    etac_ast::ast::Interface::parse(tokens, dcx)
}

/// Parse a bare expression.
pub fn parse_expr(tokens: &Tokens, dcx: &DiagCtx) -> ParseResult<etac_ast::ast::ExprKind> {
    let mut p = Parser::new(tokens, dcx);
    let tree = grammar::expr(&mut p);
    if !p.at_eof() {
        let _ = p.err_expected(parser::Expected::Set(token_set::TokenSet::of(etac_lexer::Tok![EOF])));
    }
    ParseResult {
        tree,
        errors: p.finish_parse().or(tokens.errors),
    }
}
