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
        let (parse_res, dcx) = match ty {
            Ft::Prgm => etac_parse::debug::parse_program(&$cache.sources, &mut $cache.state.interner, id, etac_errors::dcx::DiagCtx::default()),
            Ft::Intf => etac_parse::debug::parse_interface(&$cache.sources, &mut $cache.state.interner, id, etac_errors::dcx::DiagCtx::default()),
        };
        dcx.emit_diagnostics(&$cache.sources);
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
