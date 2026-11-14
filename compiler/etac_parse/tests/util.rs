#[allow(unused_macros)]
macro_rules! parsetest {
    // --- single-item shortcuts (braces optional) ---
    ($name:literal, $src:expr, $expected:expr) => {
        parsetest! { @wrap $name { $src, $expected } }
    };

    // --- @item rules must come BEFORE the general catch-all ---
    (@item $writer:ident, $session:ident, $n:ident; $name:literal { $src:expr, $expected:expr } $($rest:tt)*) => {
        enum Ft { Prgm, Intf }
        let (ty, id) = match $name {
            ".eta" => (Ft::Prgm, $session.sources.alloc_source(format!("src{}{}", { $n += 1; $n - 1 }, $name), $src.to_owned()).0),
            ".eti" => (Ft::Intf, $session.sources.alloc_source(format!("src{}{}", { $n += 1; $n - 1 }, $name), $src.to_owned()).0),
            n if regex::regex!(r"src\d+\.(eti|eta)").is_match(n) => panic!("this file name is reserved for unnamed files"),
            n if n.ends_with(".eta") => (Ft::Prgm, $session.sources.alloc_source(n.to_owned(), $src.to_owned()).0),
            n if n.ends_with(".eti") => (Ft::Intf, $session.sources.alloc_source(n.to_owned(), $src.to_owned()).0),
            u => panic!("no parser for this file type {}", u),
        };
        let emitter = etac_errors::emitter::IoEmitter::with_config(Box::new($writer.clone()), etac_debug::EMIT_CONF_DEBUG);
        let dcx = etac_errors::dcx::DiagCtx::new(emitter); // we need a new context every time cause
                                                           // we flush the diagnostics for every
                                                           // file which consumes them. same thing
                                                           // in lextest!
        {
            let debugger = etac_frontend::Debugger::new($writer.clone(), &$session.sources).trees(true);
            let frontend = etac_frontend::Frontend::new(&$session.sources, &$session.state.interner, &dcx, debugger);
            match ty {
                Ft::Prgm => drop(frontend.program(etac_source::SourceId::new(id))),
                Ft::Intf => drop(frontend.interface(etac_source::InterfaceId::new(id))),
            };
        }
        dcx.emit_diagnostics(&$session.sources);
        let expected: expect_test::Expect = $expected;
        expected.assert_eq(&$writer.take()); // clear the buffer we compare one file output at a
                                               // time
        parsetest!(@item $writer, $session, $n; $($rest)*);
    };
    (@item $writer:ident, $session:ident, $n:ident;) => {};

    // --- wrapped/general entry point ---
    (@wrap $($rest:tt)*) => {{
        let mut writer = etac_debug::StringWriter(etac_debug::StringBuffer::default());
        let session = etac_session::Session::default();
        let mut n: usize = 0;
        parsetest!(@item writer, session, n; $($rest)*);
    }};

    // --- top-level catch-all for multi-item / already-braced calls ---
    ($($rest:tt)*) => {
        parsetest! { @wrap $($rest)* }
    };
}
