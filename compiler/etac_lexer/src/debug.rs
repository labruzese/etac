use std::fmt::Write;
use std::cell::RefCell;

use etac_debug::OutputBuffer;
use etac_errors::{dcx::DiagCtx, emitter::EmitConfig};
use etac_intern::EtacInterner;
use crate::token::Token;
use crate::{EtaLexer, LexResult};
use etac_source::{FileId, SourceMap, Span};

#[inline]
fn write_token(write_buf: &RefCell<String>, token: Token, span: Span, sm: &SourceMap, lookup: &EtacInterner) {
    let span_location = sm.span_location(span);
    let display_token = crate::token::TokenDisplay::new(token, lookup);
    let _ = writeln!(write_buf.borrow_mut(), "{span_location:<20} {display_token}");
}

/// testing interface for lexer
pub fn lex<'sm>(sm: &SourceMap, interner: &mut EtacInterner, file_id: FileId<'sm>, mut dcx: DiagCtx) -> (OutputBuffer, DiagCtx) {
    let (out, emitter) = etac_debug::string_emitter(EmitConfig::default().with_color(false).with_compact(false));
    dcx.change_emitter(Box::new(emitter));

    let lexer = EtaLexer::new(sm.base_offset(file_id), interner, &dcx);

    let LexResult { tokens, diags } = lexer.lex(&sm.get_source(file_id).source);

    for (token, span) in tokens.data {
        write_token(&out, token, span, sm, interner);
    }

    for diag in diags {
        let _g = diag.finish();
    }

    (out, dcx)
}
