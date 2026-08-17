//! Diagnostic sinks.
//!
//! An [`Emitter`] is the *only* thing that turns a [`Diagnostic`] into output. The
//! [`DiagCtxt`](crate::DiagCtxt) owns one and routes every diagnostic through it

use std::io::Write;
use std::{cell::RefCell, rc::Rc};

use ariadne::{IndexType, Label, Report, ReportKind};
use etac_source::{FileId, SourceMap};

use crate::{Level, dcx::Diagnostic};

/// Can take ownership of a diagnostic to emit it
pub trait Emitter {
    fn emit(&mut self, sm: &SourceMap, diag: crate::dcx::Diagnostic);
}

pub type EmitConfig = ariadne::Config;

/// Renders diagnostics to stderr with source snippets via `ariadne`.
#[derive(Debug, Default, Clone, Copy)]
pub struct IoEmitter<W: Write> {
    writer: W,
    conf: EmitConfig,
}

impl<W: Write> IoEmitter<W> {
    pub fn new(writer: W) -> Self {
        Self::with_config(writer, EmitConfig::default())
    }
    pub fn with_config(writer: W, conf: EmitConfig) -> Self {
        Self { writer, conf: conf.with_index_type(IndexType::Byte) }
    }
}

impl<W: Write> Emitter for IoEmitter<W> {
    fn emit(&mut self, sm: &SourceMap, diag: crate::dcx::Diagnostic) {
        diag.render_report(sm, &mut self.writer, self.conf);
    }
}

/// Records diagnostics into a shared buffer instead of printing them.
///
/// Cloning shares the same underlying buffer (it is an `Rc<RefCell<_>>`), so a test can
/// keep a handle, hand a clone to the [`DiagCtxt`](crate::DiagCtxt), run a phase, and
/// then read back exactly what was emitted via [`take`](BufferEmitter::take).
#[derive(Debug, Clone, Default)]
pub struct BufferEmitter(Rc<RefCell<Vec<Diagnostic>>>);

impl BufferEmitter {
    pub fn new() -> Self {
        Self::default()
    }
    /// Drain everything emitted so far, leaving the buffer empty.
    pub fn take(&self) -> Vec<Diagnostic> {
        std::mem::take(&mut self.0.borrow_mut())
    }

    /// Number of diagnostics currently buffered.
    pub fn len(&self) -> usize {
        self.0.borrow().len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.borrow().is_empty()
    }
}

impl Emitter for BufferEmitter {
    fn emit<'sm>(&mut self, _sm: &'sm SourceMap, diagnostic: crate::dcx::Diagnostic) {
        self.0.borrow_mut().push(diagnostic);
    }
}

impl Diagnostic {
    pub fn render_report<'sm>(&self, sm: &'sm SourceMap, writer: &mut impl std::io::Write, conf: EmitConfig) {
        let _ = self.make_report(sm, conf).write(sm, writer);
    }

    fn make_report<'sm>(&self, sm: &'sm SourceMap, conf: EmitConfig) -> ariadne::Report<'sm, (etac_source::FileId<'sm>, std::ops::Range<usize>)>{
        let kind = match self.level {
            Level::Error => ReportKind::Error,
            Level::Warning => ReportKind::Warning,
            Level::Note => ReportKind::Advice,
        };

        if let Some(loc) = self.loc {
            let aspan = sm.local_span(loc);
            let mut b = Report::build(kind, aspan)
                .with_config(conf)
                .with_message(&self.message);
            if let Some(c) = &self.code {
                b = b.with_code(c);
            }
            if let Some(n) = &self.note {
                b = b.with_note(n);
            }
            for (span, msg, color) in &self.labels {
                let aspan = sm.local_span(*span);
                b = b.with_label(Label::new(aspan).with_message(msg).with_color(*color));
            }
            b.finish()
        } else {
            let mut b = Report::build(kind, (FileId::DUMMY, 0..0)).with_message(&self.message);
            if let Some(c) = &self.code {
                b = b.with_code(c);
            }
            if let Some(n) = &self.note {
                b = b.with_note(n);
            }
            for (_span, msg, color) in &self.labels {
                b = b.with_label(Label::new((FileId::DUMMY, 0..0)).with_message(msg).with_color(*color));
            }
            b.finish()
        }
    }
}
