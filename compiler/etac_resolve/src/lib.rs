//! Path resolution

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use etac_cache::sources::{FileId, InterfaceId, SourceId, SourceMap, Span};
use etac_errors::etac_error;
use etac_errors::dcx::{DiagCtx, Diag};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum File<'sm> {
    Program(SourceId<'sm>),
    Interface(InterfaceId<'sm>),
}

pub struct Resolver<'sm> {
    source_path: PathBuf,
    lib_path: PathBuf,
    seen: HashSet<FileId<'sm>>,
}

impl<'sm> Resolver<'sm> {
    #[must_use]
    pub fn new(source_path: &Path, lib_path: &Path) -> Self {
        Self {
            source_path: source_path.to_path_buf(),
            lib_path: lib_path.to_path_buf(),
            seen: HashSet::new(),
        }
    }

    /// Classify and load a file named on the command line, resolving relative
    /// paths against `--sourcepath`.
    ///
    /// `Ok(None)` means the path names a file that is already queued.
    /// `Err` carries an unemitted diagnostic: the path was unusable (non-UTF8
    /// name, unknown extension, or an I/O error).
    pub fn classify_cli<'dcx>(
        &mut self,
        sm: &'sm mut SourceMap,
        dcx: &'dcx DiagCtx,
        path: &Path,
    ) -> Result<Option<File<'sm>>, Diag<'dcx>> {
        let path = resolve_against(&self.source_path, path);
        let Some(path_str) = path.to_str() else {
            return Err(dcx.err_no_span(format!("non-UTF8 file name {}", path.to_string_lossy())));
        };

        let is_interface = match path.extension().and_then(|x| x.to_str()) {
            Some("eta") => false,
            Some("eti") => true,
            ext => {
                return Err(dcx.err_no_span(format!(
                    "unknown file type `{}` for {path_str}",
                    ext.unwrap_or("")
                )));
            }
        };

        let id = self.load(sm, dcx, path_str)?;
        match self.seen.insert(id) {
            true if is_interface => Ok(Some(File::Interface(id))),
            true => Ok(Some(File::Program(id))),
            false => Ok(None),
        }
    }

    /// Resolve one `use name` appearing in `from`. The search order is the directory
    /// of `from`, then `--libpath`.
    ///
    /// Takes the name and blame span rather than an AST node, so the resolver
    /// stays independent of `etac_ast` and trivially testable.
    ///
    /// `Ok(None)` means the interface is already queued. `Err` carries an
    /// unemitted diagnostic: no candidate exists on the search path (blamed at
    /// `at`, naming every location searched).
    pub fn resolve_use<'dcx>(
        &mut self,
        sm: &'sm mut SourceMap,
        dcx: &'dcx DiagCtx,
        from: SourceId<'sm>,
        name: &str,
        at: Span,
    ) -> Result<Option<InterfaceId<'sm>>, Diag<'dcx>> {
        let file_name = format!("{name}.eti");
        let from_path = sm.get_source(from).name.to_string();
        let from_dir = Path::new(&from_path)
            .parent()
            .unwrap_or_else(|| Path::new(""));

        let mut candidates = vec![from_dir.join(&file_name)];
        let in_lib = self.lib_path.join(&file_name);
        if in_lib != candidates[0] {
            candidates.push(in_lib);
        }

        for candidate in &candidates {
            if candidate.is_file() {
                let Some(candidate_str) = candidate.to_str() else {
                    continue;
                };
                let iid = self.load(sm, dcx, candidate_str)?;
                return Ok(self.seen.insert(iid).then_some(iid));
            }
        }

        let searched = candidates
            .iter()
            .map(|c| c.display().to_string())
            .collect::<Vec<_>>()
            .join(", ");
        Err(etac_error! {
            dcx, at, "cannot find interface `{}`", name;
            primary: "no `{}` on the search path", file_name;
            note: "searched: {}", searched;
        })
    }

    /// Read `path_str` from disk and store it in the cache, reusing the
    /// existing [`FileId`] if this path has already been loaded.
    fn load<'dcx>(&self, sm: &'sm mut SourceMap, dcx: &'dcx DiagCtx, path_str: &str) -> Result<FileId<'sm>, Diag<'dcx>> {
        if let Some(id) = sm.get_id(path_str) {
            return Ok(id);
        }
        match std::fs::read_to_string(path_str) {
            Ok(contents) => Ok(sm.alloc_source(path_str.to_string(), contents).0),
            Err(ioe) => Err(dcx.io_err(ioe)),
        }
    }
}

/// Join `path` onto `root` unless the path is absolute or the root is the
/// default `.` — keeping the common no-flag case byte-identical to what the
/// user typed, so diagnostics and `-D` log paths reproduce it verbatim.
fn resolve_against(root: &Path, path: &Path) -> PathBuf {
    if path.is_absolute() || root == Path::new(".") {
        path.to_path_buf()
    } else {
        root.join(path)
    }
}
