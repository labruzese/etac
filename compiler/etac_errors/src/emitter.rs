//! Diagnostic sinks.
//!
//! An [`Emitter`] is the *only* thing that turns a [`Diagnostic`] into output. The
//! [`DiagCtxt`](crate::DiagCtxt) owns one and routes every diagnostic through it

use std::io::Write;

use ariadne::{IndexType, Label, Report, ReportKind};
use etac_source::{FileId, SourceMap};

use crate::{Level, dcx::Diagnostic};

pub type EmitConfig = ariadne::Config;

/// Renders diagnostics to stderr with source snippets via `ariadne`.
pub struct IoEmitter {
    writer: Option<Box<dyn Write>>,
    conf: EmitConfig,
}

impl std::fmt::Debug for IoEmitter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IoEmitter").field("writer", &"Box<dyn Write>").field("conf", &self.conf).finish()
    }
}

impl IoEmitter {
    pub fn new(writer: Box<dyn Write>) -> Self {
        Self::with_config(writer, EmitConfig::default())
    }
    pub fn sink() -> Self {
        IoEmitter {
            writer: None, conf: EmitConfig::default()
        }
    }
    pub fn with_config(writer: Box<dyn Write>, conf: EmitConfig) -> Self {
        Self { writer: Some(writer), conf: conf.with_index_type(IndexType::Byte) }
    }
}

impl Write for IoEmitter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.writer.as_mut().unwrap_or(&mut (Box::new(std::io::sink()) as Box<dyn Write>)).write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.as_mut().unwrap_or(&mut (Box::new(std::io::sink()) as Box<dyn Write>)).flush()
    }
}

impl IoEmitter {
    pub fn emit(&mut self, sm: &SourceMap, diag: crate::dcx::Diagnostic) {
        let Some(writer) = &mut self.writer else { return };
        diag.render_report(sm, writer, self.conf);
    }
}

impl Diagnostic {
    // diagnostic can render itself, usually this is done by an emitter, prefer creating an emitter
    // rather than emitting them directly.
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
