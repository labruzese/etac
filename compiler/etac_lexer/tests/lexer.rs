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
    (@item $writer:ident, $session:ident, $ids:ident, $n:ident; $name:literal { $src:expr, $expected:expr } $($rest:tt)*) => {
        $ids.push_back($session.sources.alloc_source($name.to_owned(), $src.to_owned()).0);
        lextest!(@check $writer, $session, $ids; $expected);
        lextest!(@item $writer, $session, $ids, $n; $($rest)*);
    };
    (@item $writer:ident, $session:ident, $ids:ident, $n:ident; { $src:expr, $expected:expr } $($rest:tt)*) => {
        let name = format!("src{}", $n);
        $n += 1;
        $ids.push_back($session.sources.alloc_source(name, $src.to_owned()).0);
        lextest!(@check $writer, $session, $ids; $expected);
        lextest!(@item $writer, $session, $ids, $n; $($rest)*);
    };
    (@item $writer:ident, $session:ident, $ids:ident, $n:ident;) => {};

    // --- lex the next file, and compare its token dump and diagnostics to the snapshot ---
    (@check $writer:ident, $session:ident, $ids:ident; $expected:expr) => {{
        let id = $ids.pop_front().unwrap();
        let expected: expect_test::Expect = $expected;
        let emitter = etac_errors::emitter::IoEmitter::with_config(Box::new($writer.clone()), etac_debug::EMIT_CONF_DEBUG);
        let dcx = etac_errors::dcx::DiagCtx::new(emitter); // a new context per file: emitting consumes it
        let lexed = etac_lexer::EtaLexer::new($session.sources.base_offset(id), &$session.state.interner, &dcx)
            .lex(&$session.sources.get_source(id).source);
        etac_lexer::debug::write_tokens(&mut $writer, &$session.sources, &$session.state.interner, &lexed.tokens);
        for diag in lexed.diags {
            let _ = diag.finish();
        }
        dcx.emit_diagnostics(&$session.sources);
        expected.assert_eq(&$writer.take()); // clear the buffer we compare output for one file at
                                               // a time
    }};

    // --- wrapped/general entry point ---
    (@wrap $($rest:tt)*) => {{
        let mut writer = etac_debug::StringWriter(etac_debug::StringBuffer::default());
        let session = etac_session::Session::default();
        let mut ids = std::collections::VecDeque::new();
        let mut n: usize = 0;
        lextest!(@item writer, session, ids, n; $($rest)*);
    }};

    // --- top-level catch-all for multi-item / already-braced calls ---
    ($($rest:tt)*) => {
        lextest! { @wrap $($rest)* }
    };
}
