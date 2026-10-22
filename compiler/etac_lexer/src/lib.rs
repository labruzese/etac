//! Lexer
//!
#![allow(clippy::cast_possible_truncation)]
#![recursion_limit = "512"]

use std::{num::ParseIntError};

use etac_errors::dcx::{DiagCtx, DiagBuilder};
use etac_intern::{EtacInterner};
use etac_source::Span;

mod internal_error;
use internal_error::{InternalLexerError};
use logos::Lexer;

pub mod token;

use crate::token::{Token, TokenKind};

mod strings;

// -- public lexer type --

pub struct EtaLexer<'dcx, R: etac_intern::Interner> {
    base: u32,
    interner: R,
    dcx: &'dcx DiagCtx,
}

pub struct LexResult<'dcx> {
    pub tokens: Tokens,
    pub diags: Box<[DiagBuilder<'dcx>]>
}

pub struct Tokens {
    pub kinds: Box<[TokenKind]>,
    pub data: Box<[(Token, Span)]>,
}

impl<'src, 'dcx> EtaLexer<'dcx, &'src mut EtacInterner> {
    pub fn new(base: u32, interner: &'src mut EtacInterner, dcx: &'dcx DiagCtx) -> Self {
        Self {
            base,
            interner,
            dcx,
        }
    }
    
    pub fn lex(self, source: &str) -> LexResult<'dcx> {
        let eof_loc = self.base + u32::try_from(source.len()).unwrap();
        let mut diags = Vec::new();

        let mut lexer = logos::Lexer::<Token>::with_extras(source, (self.base, self.interner));

        let mut data = Vec::with_capacity(source.len()/8); // heuristic that avg token length is 8 bytes
        while let Some(tresult) = lexer.next() {
            // errors
            let span = normalize_span(lexer.extras.0, lexer.span());
            let tok = match tresult {
                Ok(tok) => tok,
                Err(ie) => {
                    let mut d = self.dcx.new_err(ie.span, ie.message);
                    if let Some(l) = ie.plabel { d = d.with_message(l); }
                    if let Some(l) = ie.note { d = d.with_note(l); }
                    diags.push(d);
                    Token::Error
                }
            };
            data.push((tok, span));
        }
        // eof
        data.push((Token::EOF, Span::new(eof_loc, eof_loc)));

        let kinds = data.iter().map(|(tok, _)| tok.kind()).collect::<Vec<_>>();

        LexResult {
            tokens: Tokens { kinds: kinds.into_boxed_slice(), data: data.into_boxed_slice() },
            diags: diags.into_boxed_slice(),
        }
    }
}

// -- lexing callbacks -- 

fn parse_int(lex: &mut Lexer<'_, Token>) -> Result<u64, InternalLexerError> {
    lex.slice().parse::<u64>().map_err(|err: ParseIntError| {
        let (msg, prim) = match err.kind() {
            std::num::IntErrorKind::PosOverflow => (
                "this integer literal is too large to be stored in any Eta type",
                "this integer is too large"),
            std::num::IntErrorKind::NegOverflow => (
                "this integer literal is too small to be stored in any Eta type",
                "this integer is too small"),
            _ => panic!("internal int parsing error"),
        };
        InternalLexerError {
            span: current_span(lex),
            message: format!("illegal integer literal: {}", msg),
            plabel: Some(String::from(prim)),
            note: Some(String::from("eta only supports ints in the range [-2^63, 2^63)"))
        }}
    )
}

// -- helpers --

#[inline]
fn current_span(lex: &Lexer<'_, Token>) -> Span {
    normalize_span(lex.extras.0, lex.span())
}

#[inline]
fn normalize_span(base: u32, span: std::ops::Range<usize>) -> Span {
    Span::new(
        base + span.start as u32,
        base + span.end as u32
    )
}

fn internal_error_unknown_token(lex: &mut Lexer<'_, Token>) -> InternalLexerError {
    InternalLexerError {
        span: current_span(lex),
        message: String::from("unknown token"),
        plabel: Some(String::from("this token")),
        note: None
    }
}
