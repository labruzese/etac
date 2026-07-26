use std::marker::PhantomData;
use std::ops::Range;
use std::rc::Rc;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;
use crossbeam_skiplist::SkipMap;
use dashmap::DashMap;
use elsa::sync::FrozenMap;

pub type SourceId<'sm> = FileId<'sm>;
pub type InterfaceId<'sm> = FileId<'sm>;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct FileId<'sm> {
    pub base: u32,
    _cache: PhantomData<&'sm SourceMap>,
}
impl FileId<'_> {
    pub const DUMMY: FileId<'static> = FileId { base: u32::MAX, _cache: PhantomData };
    pub(crate) fn new(base: u32) -> Self {
        FileId {
            base,
            _cache: PhantomData,
        }
    }
}

pub struct SourceRecord {
    pub name: Rc<str>,
    pub source: Rc<str>,
}

#[derive(Default)]
pub struct SourceMap {
    lc_cache: FrozenMap<u32, Box<ariadne::Source<Rc<str>>>>,
    source_starts: SkipMap<u32, ()>,
    source_texts: FrozenMap<u32, Box<SourceRecord>>,
    name_to_id: DashMap<Rc<str>, u32>,
    next_base: AtomicU32,
}

impl SourceMap {
    pub fn alloc_source(&self, display_name: String, text: String) -> (FileId<'_>, &str) {
        let name: Rc<str> = Rc::from(display_name);
        let source: Rc<str> = Rc::from(text);

        let len = u32::try_from(source.len()).expect("text maximum is 4GB (u32::MAX bytes)");
        // +1 keeps bases unique even for empty files.
        let base = self.next_base.fetch_add(len + 1, Ordering::SeqCst);
        self.name_to_id.insert(name.clone(), base);
        self.source_starts.insert(base, ());
        let record = self.source_texts.insert(
            base,
            Box::new(SourceRecord {
                name,
                source,
            }),
        );
        (FileId::new(base), &record.source)
    }

    pub fn get_id(&self, display_name: &str) -> Option<FileId<'_>> {
        self.name_to_id.get(display_name).map(|e| FileId::new(*e.value()))
    }

    pub fn get_source<'ec>(&'ec self, id: FileId<'ec>) -> &'ec SourceRecord {
        self.source_texts
            .get(&id.base)
            .expect("FileId constructed outside this cache passed")
    }

    pub fn base_offset(&self, id: FileId<'_>) -> u32 {
        id.base
    }

    pub fn local_span(&self, span: Span) -> (FileId<'_>, std::ops::Range<usize>) {
        let entry = self
            .source_starts
            .upper_bound(std::ops::Bound::Included(&span.lo))
            .expect("span.lo below the first file start");
        let base = *entry.key();
        let id = FileId::new(base);
        (id, (span.lo - base) as usize..(span.hi - base) as usize)
    }

    pub fn ariadne_source(&self, id: FileId<'_>) -> &ariadne::Source<Rc<str>> {
        if let Some(s) = self.lc_cache.get(&id.base) {
            s
        } else {
            self.lc_cache.insert(id.base, Box::new(ariadne::Source::from(self.get_source(id).source.clone())))
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
}

impl<'sm> ariadne::Cache<FileId<'sm>> for &SourceMap {
    type Storage = Rc<str>;
    fn fetch(&mut self, id: &FileId<'sm>) -> Result<&ariadne::Source<Self::Storage>, impl std::fmt::Debug> {
        Ok::<_, std::convert::Infallible>(self.ariadne_source(*id))
    }
    fn display<'a>(&self, id: &'a FileId<'sm>) -> Option<impl std::fmt::Display + 'a> {
        Some(self.get_source(*id).name.clone())
    }
}

pub struct SpanLocation {
    pub file: Rc<str>,
    /// (line, col)
    pub start: (u32, u32),
    /// (line, col)
    pub end: (u32, u32),
}
impl std::fmt::Display for SpanLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}..{}:{}", self.file, self.start.0, self.start.1, self.end.0, self.end.1)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Span {
    pub lo: u32,
    pub hi: u32,
}
impl Span {
    pub const DUMMY: Span = Span { lo: u32::MAX, hi: u32::MAX };
    pub fn new(lo: impl Into<u32>, hi: impl Into<u32>) -> Self {
        Self { lo: lo.into(), hi: hi.into(), }
    }
}

impl From<std::ops::Range<u32>> for Span {
    fn from(value: std::ops::Range<u32>) -> Self {
        Span::new(value.start, value.end)
    }
}
