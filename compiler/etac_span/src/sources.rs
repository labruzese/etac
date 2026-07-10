use std::{fmt, ops::Range};

use ariadne::{Source};

use crate::{FileId, ReportableSpan, Span};

pub trait SourceCache: Send + Sync {
    fn contains(&self, display_name: &str) -> Option<FileId>;

    fn store(&self, display_name: String, value: String) -> (FileId, &Source<String>);

    fn load_source(&self, id: FileId) -> &ariadne::Source<String>; 

    fn load_name(&self, id: FileId) -> &str;

    fn resolve_span(&self, span: Span) -> (Range<u32>, FileId);

    /// The global base offset a [`FileId`] was allocated at, paired with its source.
    /// Together with [`SourceCache::load_source`] this is everything needed to feed a
    /// freshly resolved file into the lexer.
    fn load(&self, id: FileId) -> (u32, &ariadne::Source<String>);

    fn reportable_span(&self, span: Span) -> ReportableSpan<'_, Self> {
        ReportableSpan::new(self, span)
    }

    /// 1-indexed `(line, column)` of a global byte offset. Used by `-D` loggers to
    /// print human-readable locations without going through a full `ariadne` report.
    fn line_column(&self, global_offset: u32) -> (u32, u32) {
        let (local_range, file_id) = self.resolve_span(Span::new(global_offset, global_offset));
        let source = self.load_source(file_id);
        let (_line, linen, coln) = source
            .get_byte_line(local_range.start as usize)
            .expect("requested line/col is out of bounds");
        (
            u32::try_from(linen).expect("requested line/col is out of bounds") + 1,
            u32::try_from(coln).expect("requested line/col is out of bounds") + 1,
        )
    }
}

impl<C: SourceCache + ?Sized> SourceCache for &C {
    fn contains(&self, display_name: &str) -> Option<FileId> {
        (**self).contains(display_name)
    }

    fn store(&self, display_name: String, value: String) -> (FileId, &Source<String>) {
        (**self).store(display_name, value)
    }

    fn load_source(&self, id: FileId) -> &ariadne::Source<String> {
        (**self).load_source(id)
    }

    fn load_name(&self, id: FileId) -> &str {
        (**self).load_name(id)
    }

    fn resolve_span(&self, span: Span) -> (Range<u32>, FileId) {
        (**self).resolve_span(span)
    }

    fn load(&self, id: FileId) -> (u32, &ariadne::Source<String>) {
        (**self).load(id)
    }
}

pub struct AriadneAdapter<'a, T>(pub &'a T);

impl<'a, T: SourceCache> ariadne::Cache<FileId> for AriadneAdapter<'a, T> {
    type Storage = String;

    fn fetch(&mut self, id: &FileId) -> Result<&Source<Self::Storage>, impl fmt::Debug> {
        Ok::<_, std::convert::Infallible>(self.0.load_source(*id))
    }

    fn display<'b>(&self, id: &'b FileId) -> Option<impl fmt::Display + 'b> {
        Some(self.0.load_name(*id).to_owned())
    }
}

pub mod global_context;
pub use global_context::*;

// pub fn line_column(source: &ariadne::Source, at: usize) -> (u32, u32) {
//     let (_line, linen, coln) = source
//         .get_byte_line(at)
//         .map(|(a, b, c)| {
//             (
//                 a,
//                 u32::try_from(b).expect("requested line/col is out of bounds"),
//                 u32::try_from(c).expect("requested line/col is out of bounds"),
//             )
//         })
//         .expect("requested line/col is out of bounds");
//
//     (linen + 1, coln + 1)
// }
