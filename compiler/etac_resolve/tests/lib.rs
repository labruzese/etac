//! Loading the files named on the command line.

use std::path::Path;

use etac_debug::{EMIT_CONF_DEBUG, StringWriter};
use etac_errors::{dcx::DiagCtx, emitter::IoEmitter};
use etac_resolve::Resolver;
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
