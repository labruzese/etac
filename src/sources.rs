use std::collections::HashMap;
use std::fmt;
use std::fmt::Debug;
use std::rc::Rc;

/// A unique identifier for a source file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FileId(usize);

impl fmt::Display for FileId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<file:{}>", self.0)
    }
}

/// Concrete struct where the source information lives
pub struct EtaSource {
    pub name: String,
    pub source: Rc<str>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EtaSpan {
    pub file_id: FileId,
    pub range: std::ops::Range<usize>,
}

impl From<(FileId, std::ops::Range<usize>)> for EtaSpan {
    fn from(value: (FileId, std::ops::Range<usize>)) -> Self {
        EtaSpan {
            file_id: value.0,
            range: value.1,
        }
    }
}

/// Stores the source code and names for all files being compiled, to be indexed with FileId
pub struct SourceManager {
    /// Storage: EtaSource(File Name, Source Content)
    sources: Vec<Box<EtaSource>>,
    /// Quick lookup for file IDs if needed by name
    pub by_path: HashMap<String, FileId>,
}

impl SourceManager {
    pub fn new() -> Self {
        Self {
            sources: Vec::new(),
            by_path: HashMap::new(),
        }
    }

    /// Add a new source file to the manager.
    pub fn add(&mut self, name: impl Into<String>, src: Rc<str>) -> FileId {
        let name = name.into();

        let id = FileId(self.sources.len());
        self.by_path.insert(name.clone(), id);
        self.sources.push(Box::new(EtaSource { name, source: src }));

        id
    }

    /// id -> Borrow the file name
    pub fn get_file_name(&self, id: FileId) -> Option<&str> {
        self.sources.get(id.0).map(|s| s.name.as_str())
    }

    /// id -> Get a new (rc) pointer to the source str
    pub fn get_source(&self, id: FileId) -> Option<Rc<str>> {
        self.sources.get(id.0).map(|s| s.source.clone())
    }
}
