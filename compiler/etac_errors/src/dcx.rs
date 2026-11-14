use std::marker::PhantomData;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Mutex, PoisonError};

use etac_source::{SourceMap, Span};

use crate::Level;
use crate::emitter::IoEmitter;

#[cfg(debug_assertions)]
use crate::drop_bomb::DropBomb;
use crate::guarantee::{EmissionGuarantee, ErrorGuaranteed, Recorded};

#[derive(Debug)]
pub struct Diagnostic {
    pub level: Level,
    pub message: String,
    pub loc: Option<Span>,
    pub labels: Vec<Label>,
    pub notes: Vec<String>,
}

/// A message attached to a piece of source.
#[derive(Debug)]
pub struct Label {
    pub span: Span,
    pub message: String,
    pub kind: LabelKind,
}

/// What a [`Label`] is for. How each kind looks is up to the emitter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LabelKind {
    /// Points at the diagnostic's own span: what went wrong.
    Primary,
    /// Points somewhere else that helps explain it.
    Secondary,
}

/// Collects the diagnostics of a compilation, and writes them out at the end.
///
/// Reporting goes through `&self` and the context is `Send + Sync`, so every thread of a compilation reports to the
/// same one.
pub struct DiagCtx {
    diagnostics: elsa::sync::FrozenVec<Box<Diagnostic>>,
    emitter: Mutex<IoEmitter>,
    err_count: AtomicUsize,
    warn_count: AtomicUsize,
    #[cfg(debug_assertions)]
    bomb: DropBomb,
}

const fn assert_send_sync<T: Send + Sync>() {}
const _: () = assert_send_sync::<DiagCtx>();
const _: () = assert_send_sync::<DiagBuilder<'static>>();

impl Default for DiagCtx {
    fn default() -> Self {
        Self::new(IoEmitter::stderr())
    }
}
impl DiagCtx {
    #[must_use]
    pub fn new(emitter: IoEmitter) -> Self {
        Self {
            diagnostics: elsa::sync::FrozenVec::new(),
            emitter: Mutex::new(emitter),
            err_count: 0.into(),
            warn_count: 0.into(),
            #[cfg(debug_assertions)]
            bomb: DropBomb::new("DiagCtx dropped without writing diagnostics to emitter"),
        }
    }

    /// Start building an error at `span`.
    pub fn new_err(&self, span: Span, msg: impl Into<String>) -> DiagBuilder<'_> {
        DiagBuilder::new(self, Level::Error, span, msg)
    }

    /// Start building a location-less error.
    pub fn new_err_unspanned(&self, msg: impl Into<String>) -> DiagBuilder<'_> {
        DiagBuilder::new_unspanned(self, Level::Error, msg)
    }

    /// Start building a warning at `span`.
    pub fn new_warn(&self, span: Span, msg: impl Into<String>) -> DiagBuilder<'_, ()> {
        DiagBuilder::new(self, Level::Warning, span, msg)
    }

    /// Start building the error for a file that could not be read.
    pub fn new_err_io(&self, path: impl std::fmt::Display, io_err: std::io::Error) -> DiagBuilder<'_> {
        self.new_err_unspanned(format!("cannot read `{path}`: {io_err}"))
    }

    pub fn err_count(&self) -> usize {
        self.err_count.load(Ordering::Acquire)
    }

    pub fn warn_count(&self) -> usize {
        self.warn_count.load(Ordering::Acquire)
    }

    pub fn has_errors(&self) -> bool {
        self.err_count() > 0
    }

    /// Write every diagnostic reported so far to the emitter, in the order they appear in the source.
    pub fn emit_diagnostics(mut self, source_map: &SourceMap) {
        // In source order, whatever order they were reported in: a file's lexical errors are all reported before
        // its syntax errors, and threads report as they go. Spans are offsets into the whole compilation, so this
        // also keeps each file's diagnostics together. Those without a location come first, and the sort is stable,
        // so diagnostics at the same place stay in the order they were reported.
        let mut diagnostics = self.diagnostics.into_vec();
        diagnostics.sort_by_key(|diag| diag.loc.map(|loc| loc.lo));

        let emitter = self.emitter.get_mut().unwrap_or_else(PoisonError::into_inner);
        emitter.emit(source_map, diagnostics.into_iter().map(|diag| *diag));
        #[cfg(debug_assertions)]
        self.bomb.defuse();
    }

    /// deliberately don't emit the diagnostics to emitter
    #[allow(unused_mut)]
    pub fn cancel(mut self) {
        #[cfg(debug_assertions)]
        self.bomb.defuse();
    }
}

/// A diagnostic under construction, knowing its [`DiagCtx`].
///
/// [`Drop`] bomb will panic in debug mode if dropped without [`finish`](DiagBuilder::finish) or
/// [`cancel`](DiagBuilder::cancel).
#[must_use = "a diagnostic is not reported until you `.finish()` it (or `.cancel()` it)"]
pub struct DiagBuilder<'dcx, G: EmissionGuarantee = ErrorGuaranteed> {
    pub(crate) dcx: &'dcx DiagCtx,
    diagnostic: Box<Diagnostic>,
    /// What [`finish`](Self::finish) returns: proof of an error if this is one, and nothing if it is a warning.
    guarantee: PhantomData<G>,
    #[cfg(debug_assertions)]
    bomb: DropBomb,
}
impl<'dcx, G: EmissionGuarantee> DiagBuilder<'dcx, G> {
    /// Create a new diagnostic at a location with a message.
    fn new(dcx: &'dcx DiagCtx, level: Level, span: Span, message: impl Into<String>) -> Self {
        Self {
            dcx,
            guarantee: PhantomData,
            diagnostic: Box::new(Diagnostic {
                level,
                message: message.into(),
                labels: Vec::new(),
                loc: Some(span),
                notes: Vec::new(),
            }),
            #[cfg(debug_assertions)]
            bomb: DropBomb::new("Diag dropped without writing diagnostic to context"),
        }
    }

    /// Create a new diagnostic that doesn't have a location
    fn new_unspanned(dcx: &'dcx DiagCtx, level: Level, message: impl Into<String>) -> Self {
        Self {
            dcx,
            guarantee: PhantomData,
            diagnostic: Box::new(Diagnostic {
                level,
                message: message.into(),
                labels: Vec::new(),
                loc: None,
                notes: Vec::new(),
            }),
            #[cfg(debug_assertions)]
            bomb: DropBomb::new("Diag dropped without writing diagnostic to context"),
        }
    }

    /// Point the primary label at the diagnostic's own span.
    pub fn with_message(mut self, msg: impl Into<String>) -> Self {
        self.diagnostic.labels.push(Label {
            span: self
                .diagnostic
                .loc
                .unwrap_or_else(|| panic!("can not add primary label to a diagnostic without a location")),
            message: msg.into(),
            kind: LabelKind::Primary,
        });
        self
    }

    /// Add a secondary label at another span. This works on a diagnostic without a location of its own, too.
    pub fn with_label(mut self, span: Span, msg: impl Into<String>) -> Self {
        self.diagnostic.labels.push(Label {
            span,
            message: msg.into(),
            kind: LabelKind::Secondary,
        });
        self
    }

    /// Add a note, shown after the source snippet. A diagnostic can have several; they are shown in the order they
    /// were added.
    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.diagnostic.notes.push(note.into());
        self
    }

    /// Report a fully-built diagnostic.
    ///
    /// For an error this returns the [`ErrorGuaranteed`] that proves it was reported. A warning returns `()`.
    pub fn finish(#[allow(unused_mut)] mut self) -> G {
        let level = self.diagnostic.level;
        match level {
            Level::Error => self.dcx.err_count.fetch_add(1, Ordering::Release),
            Level::Warning => self.dcx.warn_count.fetch_add(1, Ordering::Release),
        };

        #[cfg(debug_assertions)]
        self.bomb.defuse();

        self.dcx.diagnostics.push(self.diagnostic);
        G::emitted(Recorded(()))
    }

    /// Throw the diagnostic away deliberately (drop without panic in debug mode)
    pub fn cancel(#[allow(unused_mut)] mut self) {
        #[cfg(debug_assertions)]
        self.bomb.defuse();
    }
}
