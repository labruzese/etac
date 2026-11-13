//! Path resolution

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use etac_errors::dcx::{DiagBuilder, DiagCtx};
use etac_source::{FileId, InterfaceId, SourceId, SourceMap, Span};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum File {
    Program(SourceId),
    Interface(InterfaceId),
}

impl File {
    pub fn id(&self) -> FileId {
        match self {
            File::Program(file_id) | File::Interface(file_id) => *file_id,
        }
    }
}

pub struct Resolver {
    source_path: PathBuf,
    lib_path: PathBuf,
    seen: HashSet<FileId>,
    /// Every file read so far, by its canonical path. The source map knows files by the path they were asked for,
    /// which is what diagnostics show, but one file can be asked for in more than one way.
    loaded: HashMap<PathBuf, FileId>,
}

impl Resolver {
    #[must_use]
    pub fn new(source_path: &Path, lib_path: &Path) -> Self {
        Self {
            source_path: source_path.to_path_buf(),
            lib_path: lib_path.to_path_buf(),
            seen: HashSet::new(),
            loaded: HashMap::new(),
        }
    }

    /// Classify and load a file named on the command line, resolving relative
    /// paths against `--sourcepath`.
    ///
    /// `Err` carries an unemitted diagnostic: the path was unusable (non-UTF8
    /// name, unknown extension, or an I/O error).
    pub fn classify_cli<'dcx>(
        &mut self,
        sm: &SourceMap,
        dcx: &'dcx DiagCtx,
        path: &Path,
    ) -> Result<Option<File>, DiagBuilder<'dcx>> {
        let path = resolve_against(&self.source_path, path);
        let Some(path_str) = path.to_str() else {
            return Err(dcx.new_err_unspanned(format!("non-UTF8 file name {}", path.to_string_lossy())));
        };

        let is_interface = match path.extension().and_then(|x| x.to_str()) {
            Some("eta") => false,
            Some("eti") => true,
            ext => {
                return Err(dcx.new_err_unspanned(format!("unknown file type `{}` for {path_str}", ext.unwrap_or(""))));
            }
        };

        let id = self.load(sm, dcx, path_str)?;
        self.seen.insert(id);
        match self.seen.get(&id).unwrap() {
            id if is_interface => Ok(Some(File::Interface(*id))),
            id => Ok(Some(File::Program(*id))),
        }
    }

    /// Resolve one `use name` appearing in `from`. The search order is the directory
    /// of `from`, then `--libpath`.
    ///
    /// Takes the name and blame span rather than an AST node, so the resolver
    /// stays independent of `etac_ast` and trivially testable.
    ///
    /// `Err` carries an unemitted diagnostic: no candidate exists on the search path (blamed at
    /// `at`, naming every location searched).
    pub fn resolve_use<'dcx>(
        &mut self,
        sm: &SourceMap,
        dcx: &'dcx DiagCtx,
        from: SourceId,
        name: &str,
        at: Span,
    ) -> Result<Option<InterfaceId>, DiagBuilder<'dcx>> {
        let file_name = format!("{name}.eti");
        let from_path = sm.get_source(from).name.to_string();
        let from_dir = Path::new(&from_path).parent().unwrap_or_else(|| Path::new(""));

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
                let iid = self
                    .load(sm, dcx, candidate_str)
                    .map_err(|diag| diag.with_label(at, "needed by this `use`"))?;
                return Ok(self.seen.insert(iid).then_some(iid));
            }
        }

        let searched = candidates
            .iter()
            .map(|c| c.display().to_string())
            .collect::<Vec<_>>()
            .join(", ");
        Err(dcx
            .new_err(at, format!("cannot find interface `{}`", name))
            .with_message(format!("no `{}` on the search path", file_name))
            .with_note(format!("searched: {}", searched)))
    }

    /// Read `path_str` from disk and store it in the source map, reusing the existing [`FileId`] if this file has
    /// already been loaded, under this spelling of its path or any other.
    fn load<'dcx>(&mut self, sm: &SourceMap, dcx: &'dcx DiagCtx, path_str: &str) -> Result<FileId, DiagBuilder<'dcx>> {
        // `a.eta` and `./a.eta` are one file, as is a symlink to either. A path that cannot be canonicalized cannot
        // be read either, so leave it to the read below to say why.
        let real_path = std::fs::canonicalize(path_str).ok();
        if let Some(id) = real_path.as_ref().and_then(|real| self.loaded.get(real)) {
            return Ok(*id);
        }

        let id = sm
            .get_or_load(path_str, || std::fs::read_to_string(path_str))
            .map_err(|ioe| dcx.new_err_io(path_str, ioe))?;
        if let Some(real) = real_path {
            self.loaded.insert(real, id);
        }
        Ok(id)
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
