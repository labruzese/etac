//! The frontend's interface cache and hooks.

use std::sync::Arc;

use etac_debug::StringWriter;
use etac_errors::{dcx::DiagCtx, emitter::IoEmitter};
use etac_frontend::{Debugger, Frontend};
use etac_intern::EtacInterner;
use etac_source::SourceMap;

fn interner() -> EtacInterner {
    EtacInterner::with_hasher(std::hash::BuildHasherDefault::default())
}

#[test]
fn an_interface_is_parsed_and_diagnosed_once() {
    let dcx = DiagCtx::new(IoEmitter::sink());
    let sm = SourceMap::default();
    let interner = interner();
    let (id, _) = sm.alloc_source("bad.eti".to_owned(), "f(x: int) +".to_owned());

    let frontend = Frontend::new(&sm, &interner, &dcx, ());
    let first = frontend.interface(id);
    let errors = dcx.err_count();
    assert!(errors > 0, "the interface has a syntax error");
    assert!(!first.is_clean(), "and the file says so itself");

    let second = frontend.interface(id);
    assert!(Arc::ptr_eq(&first.tree, &second.tree), "every use shares the one tree");
    assert!(!second.is_clean(), "the errors are remembered along with the tree");
    assert_eq!(dcx.err_count(), errors, "asking again must not report again");

    dcx.cancel();
}

#[test]
fn the_debugger_dumps_only_what_it_is_asked_to() {
    let dump = |tokens: bool, trees: bool| {
        let out = StringWriter::default();
        let dcx = DiagCtx::new(IoEmitter::sink());
        let sm = SourceMap::default();
        let interner = interner();
        let (id, _) = sm.alloc_source("f.eti".to_owned(), "f()".to_owned());

        let debugger = Debugger::new(out.clone(), &sm).tokens(tokens).trees(trees);
        assert!(Frontend::new(&sm, &interner, &dcx, debugger).interface(id).is_clean());
        dcx.cancel();
        out.take()
    };

    assert_eq!(dump(false, false), "");

    let tokens = dump(true, false);
    assert!(
        tokens.contains("Identifier(f)") && !tokens.contains("parse("),
        "{tokens}"
    );

    let trees = dump(false, true);
    assert!(
        trees.starts_with("f.eti: parse(clean)") && !trees.contains("Identifier"),
        "{trees}"
    );

    // tokens first, then the tree: each hook runs right after its phase
    assert_eq!(dump(true, true), tokens + &trees);
}

#[test]
fn threads_share_one_frontend() {
    let out = StringWriter::default();
    let dcx = DiagCtx::new(IoEmitter::sink());
    let sm = SourceMap::default();
    let interner = interner();
    let (bad, _) = sm.alloc_source("bad.eti".to_owned(), "f(x: int) +".to_owned());
    let programs: Vec<_> = (0..8)
        .map(|i| {
            sm.alloc_source(format!("p{i}.eta"), format!("f{i}(shared: int) {{ }}"))
                .0
        })
        .collect();

    let frontend = Frontend::new(&sm, &interner, &dcx, Debugger::new(out.clone(), &sm).trees(true));
    let interfaces = std::thread::scope(|threads| {
        let workers: Vec<_> = programs
            .iter()
            .map(|&program| {
                let frontend = &frontend;
                threads.spawn(move || {
                    assert!(frontend.program(program).is_clean());
                    frontend.interface(bad).tree
                })
            })
            .collect();
        workers.into_iter().map(|w| w.join().unwrap()).collect::<Vec<_>>()
    });

    assert!(
        interfaces.iter().all(|i| Arc::ptr_eq(i, &interfaces[0])),
        "every thread shares the one tree"
    );
    assert_eq!(
        dcx.err_count(),
        1,
        "the interface is diagnosed once, whoever asked first"
    );

    // one dump per file, each in one piece
    let dump = out.take();
    assert_eq!(dump.matches(": parse(").count(), programs.len() + 1, "{dump}");
    for i in 0..programs.len() {
        assert!(
            dump.contains(&format!("(method (decl f{i} (params (: shared int))")),
            "{dump}"
        );
    }
    dcx.cancel();
}

#[test]
fn a_lex_error_alone_makes_a_file_unclean() {
    let dcx = DiagCtx::new(IoEmitter::sink());
    let sm = SourceMap::default();
    let interner = interner();
    // `@` is not a token, and the parser stays quiet about a token the lexer already reported
    let (id, _) = sm.alloc_source("lex.eta".to_owned(), "f() { x: int = @ }".to_owned());

    let parsed = Frontend::new(&sm, &interner, &dcx, ()).program(id);
    assert!(!parsed.is_clean());
    assert_eq!(dcx.err_count(), 1);
    dcx.cancel();
}
