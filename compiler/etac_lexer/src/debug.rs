//! Token dumps, for `--dlex` and the snapshot tests.

use std::io::Write;

use crate::Tokens;
use crate::token::Token;
use etac_intern::EtacInterner;
use etac_source::{SourceMap, Span};

/// Write one token as a line: where it is, then what it is.
#[inline]
pub fn write_token(out: &mut impl Write, token: Token, span: Span, sm: &SourceMap, symbols: &EtacInterner) {
    let span_location = sm.span_location(span);
    let display_token = crate::token::TokenDisplay::new(token, symbols);
    let _ = writeln!(out, "{span_location:<20} {display_token}");
}

/// Write every token of a lexed file, one per line, ending with its `EOF`.
pub fn write_tokens(out: &mut impl Write, sm: &SourceMap, symbols: &EtacInterner, tokens: &Tokens) {
    for &(token, span) in &tokens.data {
        write_token(out, token, span, sm, symbols);
    }
}
