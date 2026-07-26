use std::{cell::RefCell, rc::Rc};

use etac_errors::{dcx::DiagCtx, emitter::EmitConfig};
use etac_lexer::EtaLexer;
use etac_source::{FileId, SourceMap};

/// testing interface for lexer
pub fn lex<'sm>(sm: &'sm SourceMap, file_id: FileId<'sm>) -> String {
    let out = Rc::new(RefCell::new(String::new()));
    let dcx = DiagCtx::with_emitter(Box::new(etac_test::string_emitter(
        &out,
        EmitConfig::default().with_color(false).with_compact(false),
    )));
    let mut lexer = EtaLexer::new(sm.base_offset(file_id), &sm.get_source(file_id).source, &dcx);

    loop {
        let item = lexer.next();
        match item {
            Some(Ok(token)) => {
                etac_test::write_token(&out, token, sm, lexer.interner());
            }
            Some(Err(diag)) => {
                diag.finish(); // queued for emission
            }
            None => break,
        }
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
    (@item $sm:ident, $ids:ident, $n:ident; $name:literal { $src:expr, $expected:expr } $($rest:tt)*) => {
        $ids.push_back($sm.store_source($name, $src.to_owned()).0);
        let expected: expect_test::Expect = $expected;
        expected.assert_eq(&lexer::lex(&$sm, $ids.pop_front().unwrap()));
        lextest!(@item $sm, $ids, $n; $($rest)*);
    };
    (@item $sm:ident, $ids:ident, $n:ident; { $src:expr, $expected:expr } $($rest:tt)*) => {
        let name = format!("src{}", $n);
        $n += 1;
        $ids.push_back($sm.alloc_source(name, $src.to_owned()).0);
        let expected: expect_test::Expect = $expected;
        expected.assert_eq(&lexer::lex(&$sm, $ids.pop_front().unwrap()));
        lextest!(@item $sm, $ids, $n; $($rest)*);
    };
    (@item $sm:ident, $ids:ident, $n:ident;) => {};

    // --- wrapped/general entry point ---
    (@wrap $($rest:tt)*) => {{
        let cache = etac_source::SourceMap::default();
        let mut ids = std::collections::VecDeque::new();
        let mut n: usize = 0;
        lextest!(@item cache, ids, n; $($rest)*);
    }};

    // --- top-level catch-all for multi-item / already-braced calls ---
    ($($rest:tt)*) => {
        lextest! { @wrap $($rest)* }
    };
}
