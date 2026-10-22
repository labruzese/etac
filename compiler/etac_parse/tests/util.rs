#[allow(unused_macros)]
macro_rules! parsetest {
    // --- single-item shortcuts (braces optional) ---
    ($name:literal, $src:expr, $expected:expr) => {
        parsetest! { @wrap $name { $src, $expected } }
    };

    // --- @item rules must come BEFORE the general catch-all ---
    (@item $writer:ident, $cache:ident, $n:ident; $name:literal { $src:expr, $expected:expr } $($rest:tt)*) => {
        enum Ft { Prgm, Intf }
        let (ty, id) = match $name {
            ".eta" => (Ft::Prgm, $cache.sources.alloc_source(format!("src{}{}", { $n += 1; $n - 1 }, $name), $src.to_owned()).0),
            ".eti" => (Ft::Intf, $cache.sources.alloc_source(format!("src{}{}", { $n += 1; $n - 1 }, $name), $src.to_owned()).0),
            n if regex::regex!(r"src\d+\.(eti|eta)").is_match(n) => panic!("this file name is reserved for unnamed files"),
            n if n.ends_with(".eta") => (Ft::Prgm, $cache.sources.alloc_source(n.to_owned(), $src.to_owned()).0),
            n if n.ends_with(".eti") => (Ft::Intf, $cache.sources.alloc_source(n.to_owned(), $src.to_owned()).0),
            u => panic!("no parser for this file type {}", u),
        };
        let emitter = etac_errors::emitter::IoEmitter::with_config(Box::new($writer.clone()), *etac_debug::EMIT_CONF_DEBUG);
        let dcx = etac_errors::dcx::DiagCtx::new(emitter); // we need a new context every time cause
                                                           // we flush the diagnostics for every
                                                           // file which consumes them. same thing
                                                           // in lextest!
        match ty {
            Ft::Prgm => etac_parse::debug::parse_program(&mut $writer, &$cache.sources, &mut $cache.state.interner, id, &dcx),
            Ft::Intf => etac_parse::debug::parse_interface(&mut $writer, &$cache.sources, &mut $cache.state.interner, id, &dcx),
        };
        dcx.emit_diagnostics(&$cache.sources);
        let expected: expect_test::Expect = $expected;
        expected.assert_eq(&$writer.0.take()); // clear the buffer we compare one file output at a
                                               // time
        parsetest!(@item $cache, $n; $($rest)*);
    };
    (@item $cache:ident, $n:ident;) => {};

    // --- wrapped/general entry point ---
    (@wrap $($rest:tt)*) => {{
        let mut writer = etac_debug::StringWriter(etac_debug::StringBuffer::default());
        let mut cache = etac_cache::EtaCache::default();
        let mut n: usize = 0;
        parsetest!(@item writer, cache, n; $($rest)*);
    }};

    // --- top-level catch-all for multi-item / already-braced calls ---
    ($($rest:tt)*) => {
        parsetest! { @wrap $($rest)* }
    };
}
