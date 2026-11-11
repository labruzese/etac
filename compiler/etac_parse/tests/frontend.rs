//! The frontend's interface cache and hooks.

use std::sync::Arc;

use etac_debug::{StringBuffer, StringWriter};
use etac_errors::{dcx::DiagCtx, emitter::IoEmitter};
use etac_intern::EtacInterner;
use etac_parse::{Debugger, Frontend};
use etac_source::SourceMap;

fn interner() -> EtacInterner {
    EtacInterner::with_hasher(std::hash::BuildHasherDefault::default())
}

#[test]
fn an_interface_is_parsed_and_diagnosed_once() {
    let dcx = DiagCtx::new(IoEmitter::sink());
    let sm = SourceMap::default();
    let mut interner = interner();
    let (id, _) = sm.alloc_source("bad.eti".to_owned(), "f(x: int) +".to_owned());

    let mut frontend = Frontend::new(&sm, &mut interner, &dcx, ());
    let first = frontend.interface(id).expect("the parser recovers");
    let errors = dcx.err_count();
    assert!(errors > 0, "the interface has a syntax error");

    let second = frontend.interface(id).expect("the parser recovers");
    assert!(Arc::ptr_eq(&first, &second), "every use shares the one tree");
    assert_eq!(dcx.err_count(), errors, "asking again must not report again");

    dcx.cancel();
}

#[test]
fn the_debugger_dumps_only_what_it_is_asked_to() {
    let dump = |tokens: bool, trees: bool| {
        let out = StringWriter(StringBuffer::default());
        let dcx = DiagCtx::new(IoEmitter::sink());
        let sm = SourceMap::default();
        let mut interner = interner();
        let (id, _) = sm.alloc_source("f.eti".to_owned(), "f()".to_owned());

        let debugger = Debugger::new(out.clone(), &sm).tokens(tokens).trees(trees);
        Frontend::new(&sm, &mut interner, &dcx, debugger)
            .interface(id)
            .expect("a clean parse");
        dcx.cancel();
        out.0.take()
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
