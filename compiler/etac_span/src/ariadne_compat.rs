use std::ops::Range;

use crate::{FileId, SourceCache, Span};

/// Span that keeps track of its source cache.
///
/// This is really only for aridane error reporting since we're incabable of 
/// passing the global space context to it from outside the library.
pub struct ReportableSpan<'a, Cache: SourceCache> {
    cache: &'a Cache,
    pub span: Span,
    own: std::cell::OnceCell<(Range<u32>, FileId)>,
}

impl<'a, Cache: SourceCache> From<(&'a Cache, Span)> for ReportableSpan<'a, Cache> {
    fn from(value: (&'a Cache, Span)) -> Self {
        ReportableSpan {
            cache: value.0,
            span: value.1,
            own: std::cell::OnceCell::new(),
        }
    }
}

impl<Cache: SourceCache> ariadne::Span for ReportableSpan<'_, Cache> {
    type SourceId = FileId;

    fn source(&self) -> &Self::SourceId {
        &self.own.get_or_init(|| self.cache.resolve_span(self.span)).1
    }

    fn start(&self) -> usize {
        self.own.get_or_init(|| self.cache.resolve_span(self.span)).0.start as usize
    }

    fn end(&self) -> usize {
        self.own.get_or_init(|| self.cache.resolve_span(self.span)).0.end as usize
    }
}
