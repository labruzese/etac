use std::fmt::Write;
use std::cell::RefCell;

use etac_errors::{dcx::DiagCtx, emitter::EmitConfig};
use etac_intern::EtacInterner;
use etac_lexer::token::Token;
use etac_lexer::{EtaLexer, LexResult};
use etac_source::{FileId, SourceMap, Span};

#[inline]
fn write_token(write_buf: &RefCell<String>, token: Token, span: Span, sm: &SourceMap, lookup: &EtacInterner) {
    let span_location = sm.span_location(span);
    let display_token = etac_lexer::token::TokenDisplay::new(token, lookup);
    let _ = writeln!(write_buf.borrow_mut(), "{span_location:<20} {display_token}");
}

/// testing interface for lexer
pub fn lex<'sm>(sm: &SourceMap, interner: &mut EtacInterner, file_id: FileId<'sm>) -> String {
    let (out, emitter) = etac_test::string_emitter(EmitConfig::default().with_color(false).with_compact(false));
    let dcx = DiagCtx::new_with_emitter(Box::new(emitter));

    let lexer = EtaLexer::new(sm.base_offset(file_id), interner, &dcx);

    let LexResult { tokens, diags } = lexer.lex(&sm.get_source(file_id).source);

    for (token, span) in tokens.data {
        write_token(&out, token, span, sm, interner);
    }

    for diag in diags {
        let _g = diag.finish();
    }

    dcx.emit_diagnostics(sm);
    out.take()
}

#[allow(unused_macros)]
macro_rules! lextest {
    // --- single-item shortcuts (braces optional) ---
    ($name:literal, $src:expr, $expected:expr) => {
        lextest! { @wrap $name { $src, $expected } }
    };
    ($src:expr, $expected:expr) => {
        lextest! { @wrap { $src, $expected } }
    };

    // --- @item rules must come BEFORE the general catch-all ---
    (@item $cache:ident, $ids:ident, $n:ident; $name:literal { $src:expr, $expected:expr } $($rest:tt)*) => {
        $ids.push_back($cache.sources.store_source($name, $src.to_owned()).0);
        let expected: expect_test::Expect = $expected;
        expected.assert_eq(&lexer::lex($cache, $ids.pop_front().unwrap()));
        lextest!(@item $cache, $ids, $n; $($rest)*);
    };
    (@item $cache:ident, $ids:ident, $n:ident; { $src:expr, $expected:expr } $($rest:tt)*) => {
        let name = format!("src{}", $n);
        $n += 1;
        $ids.push_back($cache.sources.alloc_source(name, $src.to_owned()).0);
        let expected: expect_test::Expect = $expected;
        expected.assert_eq(&lexer::lex(&$cache.sources, &mut $cache.state.interner, $ids.pop_front().unwrap()));
        lextest!(@item $cache, $ids, $n; $($rest)*);
    };
    (@item $cache:ident, $ids:ident, $n:ident;) => {};

    // --- wrapped/general entry point ---
    (@wrap $($rest:tt)*) => {{
        let mut cache = etac_cache::EtaCache::default();
        let mut ids = std::collections::VecDeque::new();
        let mut n: usize = 0;
        lextest!(@item cache, ids, n; $($rest)*);
    }};

    // --- top-level catch-all for multi-item / already-braced calls ---
    ($($rest:tt)*) => {
        lextest! { @wrap $($rest)* }
    };
}
