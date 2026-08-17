use etac_errors::{dcx::{Diag, DiagCtx}, emitter::EmitConfig};
use etac_intern::EtacInterner;
use etac_lexer::EtaLexer;
use etac_parse::{Parse, result::{Parsed, ParseResult}};
use etac_source::{FileId, SourceMap};

fn run<'sm, T>(sm: &'sm SourceMap, interner: &mut EtacInterner, file_id: FileId<'sm>) -> String
where
    T: Parse,
    for<'a, 'b> etac_ast::display::sexpr::SexpPrinter<'a, 'b, EtacInterner, T>: std::fmt::Display,
    for<'a, 'b> etac_ast::display::span::SpanPrinter<'a, 'b, T>: std::fmt::Display,
{
    let (out, emitter) = etac_test::string_emitter(EmitConfig::default().with_color(false).with_compact(false));
    let dcx = DiagCtx::with_emitter(Box::new(emitter));
    // Lex first, then parse: the parser walks a finished token array.
    let lexed = EtaLexer::new(sm.base_offset(file_id), interner, &dcx).lex(&sm.get_source(file_id).source);
    for d in Vec::from(lexed.diags) {
        let _ = d.finish();
    }

    let ParseResult { parse, mut diags } = T::parse(&lexed.tokens, &dcx);
    diags.drain(..).map(Diag::finish).reduce(|_, g| g);

    let (status, tree) = match &parse {
        Parsed::Ok(t) => ("clean", Some(t)),
        Parsed::Recovered(t) => ("recovered", Some(t)),
        Parsed::Failed => ("failed", None),
    };
    etac_test::write_parse(&out, status, tree, file_id, sm, interner);

    dcx.emit_diagnostics(sm);
    out.take()
}

pub fn parse_program<'sm>(sm: &'sm SourceMap, interner: &mut EtacInterner, file_id: FileId<'sm>) -> String {
    run::<etac_ast::ast::Program>(sm, interner, file_id)
}

pub fn parse_interface<'sm>(sm: &'sm SourceMap, interner: &mut EtacInterner, file_id: FileId<'sm>) -> String {
    run::<etac_ast::ast::Interface>(sm, interner, file_id)
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
            Ft::Prgm => util::parse_program(&$cache.sources, &mut $cache.state.interner, id),
            Ft::Intf => util::parse_interface(&$cache.sources, &mut $cache.state.interner, id),
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
