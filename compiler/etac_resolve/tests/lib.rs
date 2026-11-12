//! Loading the files named on the command line.

use std::path::Path;

use etac_debug::{EMIT_CONF_DEBUG, StringWriter};
use etac_errors::{dcx::DiagCtx, emitter::IoEmitter};
use etac_resolve::{File, Resolver};
use etac_source::SourceMap;

#[test]
fn a_file_that_cannot_be_read_is_named_in_the_error() {
    let out = StringWriter::default();
    let dcx = DiagCtx::new(IoEmitter::with_config(Box::new(out.clone()), EMIT_CONF_DEBUG));
    let sm = SourceMap::default();
    let mut resolver = Resolver::new(Path::new("."), Path::new("."));

    let missing = Path::new("no/such/dir/missing.eta");
    let Err(diag) = resolver.classify_cli(&sm, &dcx, missing) else {
        panic!("{} should not exist", missing.display());
    };
    let _ = diag.finish();
    dcx.emit_diagnostics(&sm);

    let out = out.take();
    assert!(
        out.starts_with("Error: cannot read `no/such/dir/missing.eta`: "),
        "{out}"
    );
}

#[test]
fn a_file_named_twice_is_one_file_however_it_is_spelled() {
    let dir = std::env::temp_dir().join(format!("etac-resolve-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("a.eta"), "f() { }").unwrap();

    let dcx = DiagCtx::new(IoEmitter::sink());
    let sm = SourceMap::default();
    let mut resolver = Resolver::new(Path::new("."), Path::new("."));
    let mut classify = |path: std::path::PathBuf| match resolver.classify_cli(&sm, &dcx, &path) {
        Ok(file) => file,
        Err(diag) => {
            diag.cancel();
            panic!("{} should load", path.display());
        }
    };

    let first = classify(dir.join("a.eta"));
    assert!(matches!(first, Some(File::Program(_))));
    assert!(classify(dir.join("a.eta")).is_none(), "the same path again");
    assert!(
        classify(dir.join(".").join("a.eta")).is_none(),
        "a different spelling of it"
    );
    assert!(classify(dir.join("..").join(dir.file_name().unwrap()).join("a.eta")).is_none());

    dcx.cancel();
    std::fs::remove_dir_all(&dir).unwrap();
}
