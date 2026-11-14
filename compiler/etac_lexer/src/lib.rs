//! Lexer
#![allow(clippy::cast_possible_truncation)]
#![recursion_limit = "512"]

use etac_errors::dcx::DiagCtx;
use etac_errors::guarantee::ErrorGuaranteed;
use etac_intern::EtacInterner;
use etac_source::Span;

mod internal_error;
use internal_error::InternalLexerError;
use logos::Lexer;

pub mod debug;
pub mod token;

use crate::token::{Token, TokenKind};

mod strings;

// -- public lexer type --

pub struct EtaLexer<'dcx, R: etac_intern::Interner> {
    base: u32,
    interner: R,
    dcx: &'dcx DiagCtx,
}

/// A lexed file: its tokens, ending with an `EOF`.
///
/// There are always tokens, because the lexer recovers from the errors it reports: text it could not make a token of
/// becomes an [`Error`](Token::Error) token.
pub struct Tokens {
    pub kinds: Box<[TokenKind]>,
    pub data: Box<[(Token, Span)]>,
    /// Proof that lexing reported an error, if it did. There is an `Error` token for every one.
    pub errors: Option<ErrorGuaranteed>,
}

impl<'src, 'dcx> EtaLexer<'dcx, &'src EtacInterner> {
    pub fn new(base: u32, interner: &'src EtacInterner, dcx: &'dcx DiagCtx) -> Self {
        Self { base, interner, dcx }
    }

    /// Lex `source`, reporting what is not a token to the diagnostic context as it is found.
    pub fn lex(self, source: &str) -> Tokens {
        let eof_loc = self.base + u32::try_from(source.len()).unwrap();
        let mut errors = None;

        let mut lexer = logos::Lexer::<Token>::with_extras(source, (self.base, self.interner));

        let mut data = Vec::with_capacity(source.len() / 8); // heuristic that avg token length is 8 bytes
        while let Some(tresult) = lexer.next() {
            // errors
            let span = normalize_span(lexer.extras.0, lexer.span());
            let tok = match tresult {
                Ok(tok) => tok,
                Err(ie) => {
                    let mut d = self.dcx.new_err(ie.span, ie.message);
                    if let Some(l) = ie.plabel {
                        d = d.with_message(l);
                    }
                    if let Some(l) = ie.note {
                        d = d.with_note(l);
                    }
                    let reported = d.finish();
                    errors = Some(reported);
                    Token::Error(reported)
                }
            };
            data.push((tok, span));
        }
        // eof
        data.push((Token::EOF, Span::new(eof_loc, eof_loc)));

        let kinds = data.iter().map(|(tok, _)| tok.kind()).collect::<Vec<_>>();

        Tokens {
            kinds: kinds.into_boxed_slice(),
            data: data.into_boxed_slice(),
            errors,
        }
    }
}

// -- lexing callbacks --

/// The largest integer literal: 2^63, one more than the largest `int`.
///
/// A literal has no sign of its own, so `-9223372036854775808`, the smallest `int`, is a `-` applied to this. The
/// parser rejects it anywhere else.
pub const MAX_INT_LITERAL: u64 = 1 << 63;

fn parse_int(lex: &mut Lexer<'_, Token>) -> Result<u64, InternalLexerError> {
    // The token is all digits, so the only way this fails to parse is by not fitting.
    match lex.slice().parse::<u64>() {
        Ok(value) if value <= MAX_INT_LITERAL => Ok(value),
        _ => Err(InternalLexerError {
            span: current_span(lex),
            message: String::from(
                "illegal integer literal: this integer literal is too large to be stored in any Eta type",
            ),
            plabel: Some(String::from("this integer is too large")),
            note: Some(String::from(
                "an `int` is 64 bits: from -9223372036854775808 to 9223372036854775807",
            )),
        }),
    }
}

// -- helpers --

#[inline]
fn current_span(lex: &Lexer<'_, Token>) -> Span {
    normalize_span(lex.extras.0, lex.span())
}

#[inline]
fn normalize_span(base: u32, span: std::ops::Range<usize>) -> Span {
    Span::new(base + span.start as u32, base + span.end as u32)
}

fn internal_error_unknown_token(lex: &mut Lexer<'_, Token>) -> InternalLexerError {
    InternalLexerError {
        span: current_span(lex),
        message: String::from("unknown token"),
        plabel: Some(String::from("this token")),
        note: None,
    }
}
