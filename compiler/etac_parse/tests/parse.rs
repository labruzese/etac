use std::{cell::RefCell, rc::Rc};

use etac_errors::{dcx::DiagCtx, emitter::EmitConfig};
use etac_intern::EtacInterner;
use etac_lexer::EtaLexer;
use etac_parse::IParser;
use etac_source::{FileId, SourceMap};

pub enum FileType {
    Program,
    Interface,
}

/// testing interface for lexer
pub fn parse<'sm>(ty: FileType, sm: &SourceMap, interner: &mut EtacInterner, file_id: FileId<'sm>) -> String {
    let out = Rc::new(RefCell::new(String::new()));
    let dcx = DiagCtx::with_emitter(Box::new(etac_test::string_emitter(
        &out,
        EmitConfig::default().with_color(false).with_compact(false),
    )));
    let mut lexer = EtaLexer::new(sm.base_offset(file_id), &sm.get_source(file_id).source, interner, &dcx);
    match ty {
        FileType::Program => {
            let parser = etac_parse::ProgramParser::new(&dcx);
            parser.parse(lexer);
            etac_test::
        }
        FileType::Interface => Box::new(etac_parse::InterfaceParser::new(&dcx)),
    };

    loop {
        let item = lexer.next();
        match item {
            Some(Ok(token)) => {
                etac_test::write_token(&out, token, sm, lexer.interner());
            }
            Some(Err(diag)) => {
                let _ = diag.finish(); // queued for emission
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

