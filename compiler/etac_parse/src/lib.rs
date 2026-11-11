//! The Eta parser.
//!
//! Hand-written recursive descent.
//!
//! The parser always produces a tree. Errors are reported and recovered.

use etac_errors::dcx::DiagCtx;
use etac_lexer::Tokens;

use crate::parser::Parser;
use crate::result::{ParseResult, Parsed};

mod grammar;
mod parser;
mod token_set;

mod debug;
pub mod result;

/// A syntactic category that can be parsed from a token stream.
///
/// Implemented for the two file kinds, so a caller can write
/// `Program::parse(&tokens, &dcx)` or be generic over the two.
pub trait Parse: Sized {
    fn parse<'dcx>(tokens: &Tokens, dcx: &'dcx DiagCtx) -> ParseResult<'dcx, Self>;
    pub fn debug_parse<'sm>(
        out: &mut impl Write,
        sm: &'sm SourceMap,
        interner: &mut EtacInterner,
        file_id: FileId<'sm>,
        dcx: &DiagCtx,
    ) where
        for<'a, 'b> etac_ast::display::sexpr::SexpPrinter<'a, 'b, EtacInterner, Self>: std::fmt::Display,
        for<'a, 'b> etac_ast::display::span::SpanPrinter<'a, 'b, Self>: std::fmt::Display;
}

macro_rules! impl_parse {
    ($($ty:ty => $entry:path),+) => {
        $(impl Parse for $ty {
            fn parse<'dcx>(tokens: &Tokens, dcx: &'dcx DiagCtx) -> ParseResult<'dcx, Self> {
                let mut p = Parser::new(tokens, dcx);
                let out = $entry(&mut p);
                let diags = p.finish_parse();
                let parse = if diags.is_empty() { Parsed::Ok(out) } else { Parsed::Recovered(out) };
                ParseResult { parse, diags }
            }
            pub fn debug_parse<'sm>(
                out: &mut impl Write,
                sm: &'sm SourceMap,
                interner: &mut EtacInterner,
                file_id: FileId<'sm>,
                dcx: &DiagCtx,
            ) where
                for<'a, 'b> etac_ast::display::sexpr::SexpPrinter<'a, 'b, EtacInterner, Self>: std::fmt::Display,
                for<'a, 'b> etac_ast::display::span::SpanPrinter<'a, 'b, Self>: std::fmt::Display
            {
                debug::parse(out, sm, interner, file_id, dcx)
            }
        })+
    };
}

impl_parse! {
    etac_ast::ast::Program => grammar::program,
    etac_ast::ast::Interface => grammar::interface
}

/// Parse an `.eta` file.
pub fn parse_program<'dcx>(tokens: &Tokens, dcx: &'dcx DiagCtx) -> ParseResult<'dcx, etac_ast::ast::Program> {
    etac_ast::ast::Program::parse(tokens, dcx)
}

/// Parse an `.eti` file.
pub fn parse_interface<'dcx>(tokens: &Tokens, dcx: &'dcx DiagCtx) -> ParseResult<'dcx, etac_ast::ast::Interface> {
    etac_ast::ast::Interface::parse(tokens, dcx)
}

/// Parse a bare expression.
pub fn parse_expr<'dcx>(tokens: &Tokens, dcx: &'dcx DiagCtx) -> ParseResult<'dcx, etac_ast::ast::ExprKind> {
    let mut p = Parser::new(tokens, dcx);
    let out = grammar::expr(&mut p);
    if !p.at_eof() {
        p.err_expected(parser::Expected::Set(token_set::TokenSet::of(etac_lexer::Tok![EOF])));
    }
    let diags = p.finish_parse();
    let parse = if diags.is_empty() {
        Parsed::Ok(out)
    } else {
        Parsed::Recovered(out)
    };
    ParseResult { parse, diags }
}
