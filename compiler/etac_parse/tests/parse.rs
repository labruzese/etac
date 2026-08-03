use std::{cell::RefCell, rc::Rc};

use etac_errors::{dcx::{Diag, DiagCtx}, emitter::EmitConfig};
use etac_intern::EtacInterner;
use etac_lexer::EtaLexer;
use etac_parse::{IParser, result::ParseResult};
use etac_source::{FileId, SourceMap};

pub fn parse_program<'sm>(sm: &'sm SourceMap, interner: &mut EtacInterner, file_id: FileId<'sm>) -> String {
    let out = Rc::new(RefCell::new(String::new()));
    let dcx = DiagCtx::with_emitter(Box::new(etac_test::string_emitter(
        &out,
        EmitConfig::default().with_color(false).with_compact(false),
    )));
    let lexer = EtaLexer::new(sm.base_offset(file_id), &sm.get_source(file_id).source, interner, &dcx);

    let parser = etac_parse::ProgramParser::new(&dcx);

    let ParseResult { parse, mut diags, symbols } = parser.parse(lexer);
    diags.drain(..).map(Diag::finish).reduce(|_,g| g);
    etac_test::write_parse(&out, parse, file_id, sm, symbols);
    dcx.emit_diagnostics(sm);
    out.take()
}

pub fn parse_interface<'sm>(sm: &'sm SourceMap, interner: &mut EtacInterner, file_id: FileId<'sm>) -> String {
    let out = Rc::new(RefCell::new(String::new()));
    let dcx = DiagCtx::with_emitter(Box::new(etac_test::string_emitter(
        &out,
        EmitConfig::default().with_color(false).with_compact(false),
    )));
    let lexer = EtaLexer::new(sm.base_offset(file_id), &sm.get_source(file_id).source, interner, &dcx);

    let parser = etac_parse::InterfaceParser::new(&dcx);

    let ParseResult { parse, mut diags, symbols } = parser.parse(lexer);
    diags.drain(..).map(Diag::finish).reduce(|_,g| g);
    etac_test::write_parse(&out, parse, file_id, sm, symbols);

    dcx.emit_diagnostics(sm);
    out.take()
}

#[allow(unused_macros)]
macro_rules! parsetest {
    // --- single-item shortcuts (braces optional) ---
    ($name:literal, $src:expr, $expected:expr) => {
        parsetest! { @wrap $name { $src, $expected } }
    };

    // --- @item rules must come BEFORE the general catch-all ---
    (@item $cache:ident, $n:ident; $name:literal { $src:expr, $expected:expr } $($rest:tt)*) => {
        enum Ft { Prgm, Intf }
        let (ty, id) = match $name {
            ".eta" => (Ft::Prgm, $cache.sources.alloc_source(format!("src{}{}", { $n += 1; $n - 1 }, $name), $src.to_owned()).0),
            ".eti" => (Ft::Intf, $cache.sources.alloc_source(format!("src{}{}", { $n += 1; $n - 1 }, $name), $src.to_owned()).0),
            n if regex::regex!(r"src\d+\.(eti|eta)").is_match(n) => panic!("this file name is reserved for unnamed files"),
            n if n.ends_with(".eta") => (Ft::Prgm, $cache.sources.alloc_source(n.to_owned(), $src.to_owned()).0),
            n if n.ends_with(".eti") => (Ft::Intf, $cache.sources.alloc_source(n.to_owned(), $src.to_owned()).0),
            u => panic!("no parser for this file type {}", u),
        };
        let parse_res = match ty {
            Ft::Prgm => parse::parse_program(&$cache.sources, &mut $cache.state.interner, id),
            Ft::Intf => parse::parse_interface(&$cache.sources, &mut $cache.state.interner, id),
        };
        let expected: expect_test::Expect = $expected;
        expected.assert_eq(&parse_res);
        parsetest!(@item $cache, $n; $($rest)*);
    };
    (@item $cache:ident, $n:ident;) => {};

    // --- wrapped/general entry point ---
    (@wrap $($rest:tt)*) => {{
        let mut cache = etac_cache::EtaCache::default();
        let mut n: usize = 0;
        parsetest!(@item cache, n; $($rest)*);
    }};

    // --- top-level catch-all for multi-item / already-braced calls ---
    ($($rest:tt)*) => {
        parsetest! { @wrap $($rest)* }
    };
}
