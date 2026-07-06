//! Source positions and the source map.
//!
//! All loaded files share one global space indexed with byte-offset: 
//!
//! Each file is assigned a `base` and occupies `[base, base + len)`, 
//! (a one-byte gap between files).
//!
//! A [`Span`] is two offsets into that space.
//!
//! The owning file is recovered on demand via [`SourceCache::file_for`], 
//!
//! The space is addressed with `u32`, capping total loaded source at 4 GiB.
//!
//! The cache is append-only: files are inserted into an 
//! [`elsa::sync::FrozenMap`] and never mutated or removed, so 
//! [`SourceCache::load`] can hand out `&str` borrows tied to `&self` while 
//! later loads keep appending.
//!
//! Both the table behind [`FileId`] and the [`SourceCache`] are `Sync`:
//!
//! The canonical cache is the process-global [`SOURCES`]. Because the map is
//! append-only text borrows from it are `&'static str` 

mod sources;
pub use sources::*;

mod id;
pub use id::*;

mod ariadne_compat;
pub use ariadne_compat::*;

use std::fmt;

/// A byte range `[lo, hi)` in the global source space. Meaningless without the
/// [`SourceCache`] that owns the source space.
///
/// Use [`SourceCache::file_for`] to recover a `(FileId, local range)`.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Span {
    pub lo: u32,
    pub hi: u32,
}

impl Span {
    /// Placeholder span for synthesized nodes. Should never reach a diagnostic.
    pub const DUMMY: Span = Span { lo: 0, hi: 0 };

    pub fn new(lo: impl Into<u32>, hi: impl Into<u32>) -> Self {
        Self {
            lo: lo.into(),
            hi: hi.into(),
        }
    }

    /// Smallest span covering both `self` and `other`.
    #[must_use]
    pub fn to(self, other: Span) -> Span {
        Span {
            lo: self.lo.min(other.lo),
            hi: self.hi.max(other.hi),
        }
    }

    #[must_use]
    pub fn len(self) -> u32 {
        self.hi - self.lo
    }
    #[must_use]
    pub fn is_empty(self) -> bool {
        self.lo == self.hi
    }
}

impl fmt::Debug for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}..{}", self.lo, self.hi)
    }
}



// Parallel per-file frontends require sharing the cache (and passing FileIds)
// across threads.
const _: () = {
    const fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<SourceCache>();
    assert_send_sync::<FileId>();
    assert_send_sync::<Span>();
};
