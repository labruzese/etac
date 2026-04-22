use ariadne::{Cache, Source};
use std::borrow::Borrow;
use std::collections::HashMap;
use std::convert::Infallible;
use std::fmt;
use std::rc::Rc;

mod span;
pub use span::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FileId(Rc<str>);

impl FileId {
    pub fn new(name: impl Into<Rc<str>>) -> Self { Self(name.into()) }
    pub fn as_str(&self) -> &str { &self.0 }
}

impl fmt::Display for FileId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&*self.0, f)
    }
}

// Lets HashMap<FileId, _> be queried with &str directly:
impl Borrow<str> for FileId {
    fn borrow(&self) -> &str { &self.0 }
}

pub struct Sources {
    texts: HashMap<FileId, Rc<str>>,
    ariadne_sources: HashMap<FileId, Source<Rc<str>>>,
}

impl Sources {
    pub fn new() -> Self {
        Self {
            texts: HashMap::new(),
            ariadne_sources: HashMap::new(),
        }
    }

    /// Cheap: `Rc` bump on cache hit, disk read + insert on miss.
    /// Panics if the file can't be read
    pub fn text(&mut self, id: &FileId) -> Rc<str> {
        if let Some(rc) = self.texts.get(id) {
            return Rc::clone(rc);
        }
        let rc: Rc<str> = std::fs::read_to_string(id.as_str())
            .map(Rc::from)
            .unwrap_or_else(|e| panic!("failed to read {}: {}", id, e));
        self.texts.insert(id.clone(), Rc::clone(&rc));
        rc
    }

    /// Inject a source directly — handy for tests and for sources that
    /// don't correspond to a real file on disk.
    #[allow(unused)]
    pub fn insert(&mut self, id: FileId, text: impl Into<Rc<str>>) {
        let rc = text.into();
        self.texts.insert(id.clone(), rc);
        // Drop any stale ariadne Source so it gets rebuilt on next fetch.
        self.ariadne_sources.remove(&id);
    }

    #[allow(unused)]
    pub fn text_by_name(&self, name: &str) -> Option<Rc<str>> {
        self.texts.get(name).map(Rc::clone)
    }
}

impl Default for Sources {
    fn default() -> Self { Self::new() }
}

impl Cache<FileId> for Sources {
    type Storage = Rc<str>;

    fn fetch(&mut self, id: &FileId) -> Result<&Source<Rc<str>>, impl fmt::Debug> {
        if !self.ariadne_sources.contains_key(id) {
            let rc = self.text(id);
            self.ariadne_sources.insert(id.clone(), Source::from(rc));
        }
        Ok::<_, Infallible>(self.ariadne_sources.get(id).unwrap())
    }

    fn display<'a>(&self, id: &'a FileId) -> Option<impl fmt::Display + 'a> {
        Some(id.as_str())
    }
}
