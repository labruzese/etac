//! Lexer
#![allow(clippy::cast_possible_truncation)]

use std::num::ParseIntError;

use etac_errors::dcx::{DiagCtx, Diag};
use etac_intern::{EtacInterner};
use etac_source::Span;
use logos::Logos;

mod internal_error;
use internal_error::{InternalLexerError, lexer_error};

pub mod token;

use crate::token::{RawToken, SourceToken};

mod strings;

// -- inner lexer type -- 

type LogosLexer<'src> = logos::Lexer<'src, RawToken>;

// -- public lexer type --

pub struct EtaLexer<'src, 'dcx> {
    inner: LogosLexer<'src>,
    dcx: &'dcx DiagCtx,
}

impl<'src, 'dcx> EtaLexer<'src, 'dcx> {
    pub fn new(base: u32, source: &'src str, interner: &'src mut EtacInterner, diag_context: &'dcx DiagCtx) -> Self {
        Self {
            dcx: diag_context,
            inner: <RawToken as Logos>::lexer_with_extras(source, (base, interner)),
        }
    }

    pub fn interner(&self) -> &EtacInterner {
        self.inner.extras.1
    }

    pub fn into_interner_mut(self) -> &'src mut EtacInterner {
        self.inner.extras.1
    }
}

// transformed for lalrpop
impl<'src, 'dcx> Iterator for EtaLexer<'src, 'dcx> {
    type Item = Result<SourceToken, Diag<'dcx>>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.inner.next()?;
        let local_span = self.inner.span();
        let base = self.inner.extras.0;
        let span = Span::new(base + local_span.start as u32, base + local_span.end as u32);
        match next {
            Ok(tok) => Some(Ok((span.lo, tok, span.hi))),
            Err(ie) => {
                let mut d = self.dcx.err(ie.span, ie.message);
                if let Some(l) = ie.plabel { d = d.with_primary_label(l); }
                if let Some(l) = ie.note { d = d.with_note(l); }
                Some(Err(d))
            }
        }
    }
}


// -- lexing callbacks -- 

fn parse_int(lex: &mut LogosLexer<'_>) -> Result<i64, InternalLexerError> {
    lex.slice().parse::<i64>().map_err(|err: ParseIntError| {
        let (msg, prim) = match err.kind() {
            std::num::IntErrorKind::PosOverflow => (
                "this integer literal is too large to be stored in any Eta type",
                "this integer is too large"),
            std::num::IntErrorKind::NegOverflow => (
                "this integer literal is too small to be stored in any Eta type",
                "this integer is too small"),
            _ => panic!("internal int parsing error"),
        };
        lexer_error! {
            span = current_span(lex),
            message = format!("illegal integer literal: {}", msg),
            plabel = prim,
            note = "eta only supports ints in the range [-2^63, 2^63)"
        }}
    )
}

// -- helpers --

fn current_span(lex: &LogosLexer) -> Span {
    Span::new(
        lex.extras.0 + lex.span().start as u32,
        lex.extras.0 + lex.span().end as u32
    )
}

fn internal_error_unknown_token(lex: &mut LogosLexer) -> InternalLexerError {
    lexer_error! {
        span = current_span(lex),
        message = "unknown token",
        plabel = "this token",
    }
}
