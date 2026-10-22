//! Differential test against the reference parser's accept/reject decisions.

use std::path::Path;

use etac_errors::dcx::DiagCtx;
use etac_errors::emitter::EmitConfig;
use etac_intern::EtacInterner;
use etac_parse::{Parse, result::{ParseResult, Parsed}};
use etac_source::SourceMap;

fn accepts<T: Parse>(src: &str) -> bool {
    let emitter = etac_debug::sink_emitter(EmitConfig::default().with_color(false));
    let dcx = DiagCtx::new_with_emitter(Box::new(emitter));
    let sm = SourceMap::default();
    let mut interner = EtacInterner::with_hasher(std::hash::BuildHasherDefault::default());

    let (id, text) = sm.alloc_source("corpus.eta".to_owned(), src.to_owned());
    let lexed = etac_lexer::EtaLexer::new(sm.base_offset(id), &mut interner, &dcx).lex(text);
    let lexed_clean = lexed.diags.is_empty();
    for d in Vec::from(lexed.diags) {
        d.cancel();
    }

    let ParseResult { parse, diags } = T::parse(&lexed.tokens, &dcx);
    for d in diags {
        d.cancel();
    }
    dcx.cancel();

    lexed_clean && matches!(parse, Parsed::Ok(_))
}

#[test]
fn agrees_with_reference_parser_on_pa2() {
    let dir = Path::new("reference_parser");
    if !dir.is_dir() {
        eprintln!("corpus not present at {}; skipping", dir.display());
        return;
    }

    let (mut checked, mut false_reject, mut false_accept) = (0u32, Vec::new(), Vec::new());

    let mut entries: Vec<_> = std::fs::read_dir(dir)
        .expect("read corpus dir")
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "parsedsol"))
        .collect();
    entries.sort();

    for sol_path in entries {
        let sol = std::fs::read_to_string(&sol_path).expect("read .parsedsol");
        let eta = sol_path.with_extension("eta");
        let eti = sol_path.with_extension("eti");

        let (src, is_interface) = if eta.is_file() {
            (std::fs::read_to_string(&eta).expect("read source"), false)
        } else if eti.is_file() {
            (std::fs::read_to_string(&eti).expect("read source"), true)
        } else {
            continue;
        };

        let reference_accepted = !sol.contains("error:");
        let we_accept = if is_interface {
            accepts::<etac_ast::ast::Interface>(&src)
        } else {
            accepts::<etac_ast::ast::Program>(&src)
        };

        checked += 1;
        let name = sol_path.file_stem().unwrap().to_string_lossy().into_owned();
        match (reference_accepted, we_accept) {
            (true, false) => false_reject.push(name),
            (false, true) => false_accept.push(name),
            _ => {}
        }
    }

    assert!(checked > 0, "corpus directory contained no comparable files");
    assert!(
        false_reject.is_empty() && false_accept.is_empty(),
        "{checked} files checked\n  rejected but valid: {false_reject:?}\n  accepted but invalid: {false_accept:?}"
    );
}
