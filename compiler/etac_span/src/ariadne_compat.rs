use std::ops::Range;

use crate::{FileId, SourceCache, Span};

/// Span that keeps track of its source cache.
///
/// This is really only for aridane error reporting since we're incabable of 
/// passing the global space context to it from outside the library.
pub struct ReportableSpan<'a> {
    cache: &'a SourceCache,
    pub span: Span,
    own: std::cell::OnceCell<(FileId, Range<u32>)>,
}

impl<'a> From<(&'a SourceCache, Span)> for ReportableSpan<'a> {
    fn from(value: (&'a SourceCache, Span)) -> Self {
        ReportableSpan {
            cache: value.0,
            span: value.1,
            own: std::cell::OnceCell::new(),
        }
    }
}

impl ariadne::Span for ReportableSpan<'_> {
    type SourceId = FileId;

    fn source(&self) -> &Self::SourceId {
        &self.own.get_or_init(|| self.cache.file_for(self.span)).0
    }

    fn start(&self) -> usize {
        self.own.get_or_init(|| self.cache.file_for(self.span)).1.start as usize
    }

    fn end(&self) -> usize {
        self.own.get_or_init(|| self.cache.file_for(self.span)).1.end as usize
    }
}
