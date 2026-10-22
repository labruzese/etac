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
    (@item $writer:ident, $cache:ident, $ids:ident, $n:ident; $name:literal { $src:expr, $expected:expr } $($rest:tt)*) => {
        $ids.push_back($cache.sources.store_source($name, $src.to_owned()).0);
        let expected: expect_test::Expect = $expected;
        etac_lexer::debug::lex(&mut $writer, &$cache.sources, &mut $cache.state.interner, $ids.pop_front().unwrap(), &$dcx);
        let emitter = etac_errors::emitter::IoEmitter::with_config(Box::new($writer.clone()), *etac_debug::EMIT_CONF_DEBUG);
        let dcx = etac_errors::dcx::DiagCtx::new(emitter);
        dcx.emit_diagnostics(&$cache.sources);
        expected.assert_eq(&actual_buf.borrow());
        lextest!(@item $cache, $ids, $n; $($rest)*);
    };
    (@item $writer:ident, $cache:ident, $ids:ident, $n:ident; { $src:expr, $expected:expr } $($rest:tt)*) => {
        let name = format!("src{}", $n);
        $n += 1;
        $ids.push_back($cache.sources.alloc_source(name, $src.to_owned()).0);
        let expected: expect_test::Expect = $expected;
        let emitter = etac_errors::emitter::IoEmitter::with_config(Box::new($writer.clone()), *etac_debug::EMIT_CONF_DEBUG);
        let dcx = etac_errors::dcx::DiagCtx::new(emitter);
        etac_lexer::debug::lex(&mut $writer, &$cache.sources, &mut $cache.state.interner, $ids.pop_front().unwrap(), &dcx);
        dcx.emit_diagnostics(&$cache.sources);
        expected.assert_eq(&$writer.0.take()); // clear the buffer we compare output for one file at
                                               // a time
        lextest!(@item $cache, $ids, $n; $($rest)*);
    };
    (@item $cache:ident, $ids:ident, $n:ident;) => {};

    // --- wrapped/general entry point ---
    (@wrap $($rest:tt)*) => {{
        let mut writer = etac_debug::StringWriter(etac_debug::StringBuffer::default());
        let mut cache = etac_cache::EtaCache::default();
        let mut ids = std::collections::VecDeque::new();
        let mut n: usize = 0;
        lextest!(@item writer, cache, ids, n; $($rest)*);
    }};

    // --- top-level catch-all for multi-item / already-braced calls ---
    ($($rest:tt)*) => {
        lextest! { @wrap $($rest)* }
    };
}
