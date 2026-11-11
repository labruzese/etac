use crossbeam_skiplist::SkipMap;
use dashmap::DashMap;
use dashmap::mapref::entry::Entry;
use elsa::sync::FrozenMap;
use std::sync::Arc;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;

/// The [`FileId`] of an `.eta` file.
pub type SourceId = FileId;
/// The [`FileId`] of an `.eti` file.
pub type InterfaceId = FileId;

/// Names a file in a [`SourceMap`].
///
/// An id is a plain value: it borrows nothing, so it can be stored in trees and tables that outlive any one borrow of
/// the map. It only means something to the map that handed it out.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct FileId {
    /// The offset of the file's first byte, which is what a [`Span`] into the file counts from.
    base: u32,
}
impl FileId {
    /// The id of no file. Looking it up in a map panics.
    pub const DUMMY: FileId = FileId { base: u32::MAX };

    pub(crate) fn new(base: u32) -> Self {
        FileId { base }
    }
}

pub struct SourceRecord {
    pub name: Arc<str>,
    pub source: Arc<str>,
}

/// The text of every file in a compilation, addressed by [`FileId`] and by [`Span`].
///
/// Every method takes `&self` and the map is `Send + Sync`, so the threads of a compilation can share one and add
/// files to it as they find them. A file never changes or goes away once it is in the map.
#[derive(Default)]
pub struct SourceMap {
    lc_cache: FrozenMap<u32, Box<ariadne::Source<Arc<str>>>>,
    source_starts: SkipMap<u32, ()>,
    source_texts: FrozenMap<u32, Box<SourceRecord>>,
    name_to_id: DashMap<Arc<str>, u32>,
    next_base: AtomicU32,
}
const fn assert_send_sync<T: Send + Sync>() {}
const _: () = assert_send_sync::<SourceMap>();

impl SourceMap {
    /// Add a file to the map, and make `display_name` name it.
    ///
    /// This always adds a file: if the name was already taken, it now names the new one. To add a file only the first
    /// time its name comes up, use [`get_or_load`](Self::get_or_load).
    pub fn alloc_source(&self, display_name: String, text: String) -> (FileId, &str) {
        let name: Arc<str> = Arc::from(display_name);
        let (base, source) = self.store(name.clone(), text);
        self.name_to_id.insert(name, base);
        (FileId::new(base), source)
    }

    /// The file called `display_name`, calling `read` for its text if the map does not have it yet.
    ///
    /// However many threads ask for the same name at once, `read` runs once and they all get the same id. If `read`
    /// fails nothing is stored, and the next call tries again.
    ///
    /// `read` runs while the name is locked, so it must not use this map.
    pub fn get_or_load<E>(&self, display_name: &str, read: impl FnOnce() -> Result<String, E>) -> Result<FileId, E> {
        if let Some(id) = self.get_id(display_name) {
            return Ok(id);
        }
        match self.name_to_id.entry(Arc::from(display_name)) {
            Entry::Occupied(known) => Ok(FileId::new(*known.get())),
            Entry::Vacant(slot) => {
                let (base, _) = self.store(slot.key().clone(), read()?);
                slot.insert(base);
                Ok(FileId::new(base))
            }
        }
    }

    /// Give `text` a range of offsets and store it. The caller publishes the name.
    fn store(&self, name: Arc<str>, text: String) -> (u32, &str) {
        let source: Arc<str> = Arc::from(text);

        // +1 keeps bases unique even for empty files, and puts a file's end-of-file offset inside its own range.
        let len = u32::try_from(source.len())
            .ok()
            .and_then(|len| len.checked_add(1))
            .expect("a source file is at most 4GB");
        let base = self
            .next_base
            .try_update(Ordering::SeqCst, Ordering::SeqCst, |base| base.checked_add(len))
            .expect("a compilation reads at most 4GB of source text");

        // The text goes in before anything that leads to it: a thread that finds this base, through the start table
        // here or through the name the caller publishes afterwards, always finds the text behind it.
        let record = self.source_texts.insert(base, Box::new(SourceRecord { name, source }));
        self.source_starts.insert(base, ());
        (base, &record.source)
    }

    pub fn get_id(&self, display_name: &str) -> Option<FileId> {
        self.name_to_id.get(display_name).map(|e| FileId::new(*e.value()))
    }

    pub fn get_source(&self, id: FileId) -> &SourceRecord {
        self.source_texts
            .get(&id.base)
            .expect("this FileId is not from this source map")
    }

    pub fn base_offset(&self, id: FileId) -> u32 {
        id.base
    }

    pub fn local_span(&self, span: Span) -> (FileId, std::ops::Range<usize>) {
        let entry = self
            .source_starts
            .upper_bound(std::ops::Bound::Included(&span.lo))
            .expect("span.lo below the first file start");
        let base = *entry.key();
        let id = FileId::new(base);
        (id, (span.lo - base) as usize..(span.hi - base) as usize)
    }

    pub fn ariadne_source(&self, id: FileId) -> &ariadne::Source<Arc<str>> {
        if let Some(s) = self.lc_cache.get(&id.base) {
            s
        } else {
            self.lc_cache.insert(
                id.base,
                Box::new(ariadne::Source::from(self.get_source(id).source.clone())),
            )
        }
    }

    pub fn span_location(&self, span: Span) -> SpanLocation {
        let (fid, lspan) = self.local_span(span);
        let asource = self.ariadne_source(fid);
        let (_, sl, sc) = asource.get_byte_line(lspan.start).unwrap();
        let (_, el, ec) = asource.get_byte_line(lspan.end).unwrap();
        SpanLocation {
            file: self.get_source(fid).name.clone(),
            start: (sl as u32 + 1, sc as u32 + 1), // ariadne is 0 indexed
            end: (el as u32 + 1, ec as u32 + 1),   // ariadne is 0 indexed
        }
    }

    pub fn slice(&self, span: Span) -> &str {
        let (f, ls) = self.local_span(span);
        &self.get_source(f).source[ls.start..ls.end]
    }
}

impl ariadne::Cache<FileId> for &SourceMap {
    type Storage = Arc<str>;
    fn fetch(&mut self, id: &FileId) -> Result<&ariadne::Source<Self::Storage>, impl std::fmt::Debug> {
        Ok::<_, std::convert::Infallible>(self.ariadne_source(*id))
    }
    fn display<'a>(&self, id: &'a FileId) -> Option<impl std::fmt::Display + 'a> {
        Some(self.get_source(*id).name.clone())
    }
}

pub struct SpanLocation {
    pub file: Arc<str>,
    /// (line, col)
    pub start: (u32, u32),
    /// (line, col)
    pub end: (u32, u32),
}
impl std::fmt::Display for SpanLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.width().is_none() && f.precision().is_none() {
            return write!(
                f,
                "{}:{}:{}..{}:{}",
                self.file, self.start.0, self.start.1, self.end.0, self.end.1
            );
        }
        let s = format!(
            "{}:{}:{}..{}:{}",
            self.file, self.start.0, self.start.1, self.end.0, self.end.1
        );
        f.pad(&s)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Span {
    pub lo: u32,
    pub hi: u32,
}
impl Span {
    pub const DUMMY: Span = Span {
        lo: u32::MAX,
        hi: u32::MAX,
    };
    pub fn new(lo: impl Into<u32>, hi: impl Into<u32>) -> Self {
        Self {
            lo: lo.into(),
            hi: hi.into(),
        }
    }
}

impl From<std::ops::Range<u32>> for Span {
    fn from(value: std::ops::Range<u32>) -> Self {
        Span::new(value.start, value.end)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicUsize;

    #[test]
    fn a_file_is_loaded_once_however_many_threads_ask() {
        let sm = SourceMap::default();
        let reads = AtomicUsize::new(0);
        let ids = std::thread::scope(|threads| {
            let workers: Vec<_> = (0..16)
                .map(|_| {
                    threads.spawn(|| {
                        sm.get_or_load("a.eta", || {
                            reads.fetch_add(1, Ordering::SeqCst);
                            Ok::<_, std::convert::Infallible>("f() { }".to_owned())
                        })
                        .unwrap()
                    })
                })
                .collect();
            workers.into_iter().map(|w| w.join().unwrap()).collect::<Vec<_>>()
        });

        assert_eq!(reads.load(Ordering::SeqCst), 1);
        assert!(ids.iter().all(|id| *id == ids[0]));
        assert_eq!(&*sm.get_source(ids[0]).source, "f() { }");
    }

    #[test]
    fn a_failed_load_stores_nothing() {
        let sm = SourceMap::default();
        assert_eq!(sm.get_or_load("a.eta", || Err("unreadable")), Err("unreadable"));
        assert!(sm.get_id("a.eta").is_none());
        let id = sm.get_or_load("a.eta", || Ok::<_, ()>(String::new())).unwrap();
        assert_eq!(sm.get_id("a.eta"), Some(id));
    }
}
