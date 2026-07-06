use core::fmt;
use std::{collections::HashMap, sync::{LazyLock, RwLock}};

/// A `Copy` handle naming a source or interface file.
///
/// The handle is a small index into a process-wide table of paths, so it can be
/// stored in maps and passed by value without cloning or borrowing. Recover the
/// original path with [`FileId::as_str`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FileId(u32);
/// file containing source code
pub type SourceId = FileId;
/// file containing an interface
pub type InterfaceId = FileId;

/// File paths, one entry per distinct path. 
///
/// Never cleared, and entries are leaked to `'static` so a [`FileId`] can 
/// return its path as a plain `&'static str`.
static FILE_NAMES: LazyLock<RwLock<FileNames>> =
    LazyLock::new(|| RwLock::new(FileNames::default()));

#[derive(Default)]
struct FileNames {
    by_index: Vec<&'static str>,
    by_name: HashMap<&'static str, u32>,
}

impl FileNames {
    fn intern(&mut self, name: &str) -> u32 {
        if let Some(&id) = self.by_name.get(name) {
            return id;
        }
        let name: &'static str = String::from(name).leak();
        #[allow(clippy::cast_possible_truncation)]
        let id = self.by_index.len() as u32;
        self.by_index.push(name);
        self.by_name.insert(name, id);
        id
    }
}

impl FileId {
    pub fn new(name: impl AsRef<str>) -> Self {
        FileId(
            FILE_NAMES
                .write()
                .expect("file-name table poisoned")
                .intern(name.as_ref()),
        )
    }

    #[must_use]
    pub fn as_str(&self) -> &'static str {
        // Entries are leaked to `'static`, so the borrow may outlive the guard.
        FILE_NAMES.read().expect("file-name table poisoned").by_index[self.0 as usize]
    }
}

impl fmt::Display for FileId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
