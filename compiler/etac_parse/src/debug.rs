//! Parse dumps, for `--dparse` and the snapshot tests.

use std::io::Write;

use crate::{Parse, result::Parsed};
use etac_intern::EtacInterner;
use etac_source::{FileId, SourceMap};

/// Write the outcome of parsing `file`: a status line ("clean", "recovered" or "failed"), then the tree if there is
/// one, as [`Parse::write_tree`] prints it.
pub fn write_parse<T: Parse>(
    out: &mut impl Write,
    sm: &SourceMap,
    symbols: &EtacInterner,
    file: FileId,
    parse: &Parsed<T>,
) {
    let name = &sm.get_source(file).name;
    let (status, tree) = match parse {
        Parsed::Ok(t) => ("clean", Some(t)),
        Parsed::Recovered(t) => ("recovered", Some(t)),
        Parsed::Failed => ("failed", None),
    };
    let _ = writeln!(out, "{name}: parse({status})");
    if let Some(tree) = tree {
        tree.write_tree(out, sm, symbols);
    }
}
