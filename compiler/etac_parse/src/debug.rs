//! Parse dumps, for `--dparse` and the snapshot tests.

use std::io::Write;

use crate::{Parse, result::ParseResult};
use etac_intern::EtacInterner;
use etac_source::{FileId, SourceMap};

/// Write the outcome of parsing `file`: a status line ("clean" or "recovered"), then the tree as
/// [`Parse::write_tree`] prints it.
pub fn write_parse<T: Parse>(
    out: &mut impl Write,
    sm: &SourceMap,
    symbols: &EtacInterner,
    file: FileId,
    parse: &ParseResult<T>,
) {
    let name = &sm.get_source(file).name;
    let status = if parse.is_clean() { "clean" } else { "recovered" };
    let _ = writeln!(out, "{name}: parse({status})");
    parse.tree.write_tree(out, sm, symbols);
}
