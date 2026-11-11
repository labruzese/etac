use std::io::Write;

use crate::token::Token;
use etac_intern::EtacInterner;
use etac_source::{SourceMap, Span};

#[inline]
pub fn write_token(write_buf: &mut impl Write, token: Token, span: Span, sm: &SourceMap, lookup: &EtacInterner) {
    let span_location = sm.span_location(span);
    let display_token = crate::token::TokenDisplay::new(token, lookup);
    let _ = writeln!(write_buf, "{span_location:<20} {display_token}");
}
