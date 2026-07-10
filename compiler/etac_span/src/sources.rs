use std::{
    fmt, io, ops::Range,
};

use ariadne::{Source};

use crate::{FileId, Span};

pub trait SourceCache: Send + Sync {
    fn contains(&self, display_name: &str) -> Option<FileId>;

    fn store(&mut self, display_name: String, value: String) -> (FileId, &Source<String>);

    fn load_source(&self, id: FileId) -> &ariadne::Source<String>; 

    fn load_name(&self, id: FileId) -> &str;

    fn resolve_span(&self, span: Span) -> (Range<u32>, FileId);
}

pub struct AriadneAdapter<T>(T);

impl<T: SourceCache> ariadne::Cache<FileId> for AriadneAdapter<T> {
    type Storage = String;

    fn fetch(&mut self, id: &FileId) -> Result<&Source<Self::Storage>, impl fmt::Debug> {
        Ok::<_, std::convert::Infallible>(self.0.load_source(*id))
    }

    fn display<'a>(&self, id: &'a FileId) -> Option<impl fmt::Display + 'a> {
        Some(self.0.load_name(*id).to_owned())
    }
}

pub mod global_context;


#[doc(hidden)]
pub fn lc_from_ariadne_source(source: &ariadne::Source, at: usize) -> io::Result<(u32, u32)> {
    let (_line, linen, coln) = source
        .get_byte_line(at)
        .map(|(a, b, c)| {
            (
                a,
                u32::try_from(b).expect("requested line/col is out of bounds"),
                u32::try_from(c).expect("requested line/col is out of bounds"),
            )
        })
        .expect("requested line/col is out of bounds");

    Ok((linen + 1, coln + 1))
}
