//! Diagnostic sinks.
//!
//! Rendering is done with `ariadne`, and this module is the only place that knows it: a [`Diagnostic`] and an
//! [`EmitConfig`] are described in the compiler's own terms and translated here.

use std::collections::HashMap;
use std::io::Write;
use std::sync::Arc;

use ariadne::{IndexType, Report, ReportKind};
use etac_source::{FileId, SourceMap, Span};

use crate::Level;
use crate::dcx::{Diagnostic, LabelKind};

/// How diagnostics are rendered.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EmitConfig {
    color: bool,
    compact: bool,
}

impl EmitConfig {
    /// Colored, with the roomy layout.
    pub const fn new() -> Self {
        EmitConfig {
            color: true,
            compact: false,
        }
    }

    /// Whether to color the output with ANSI escape codes.
    #[must_use]
    pub const fn with_color(mut self, color: bool) -> Self {
        self.color = color;
        self
    }

    /// Whether to leave out the blank lines that pad a report.
    #[must_use]
    pub const fn with_compact(mut self, compact: bool) -> Self {
        self.compact = compact;
        self
    }

    fn renderer(self) -> ariadne::Config {
        // spans are byte offsets
        ariadne::Config::new()
            .with_color(self.color)
            .with_compact(self.compact)
            .with_index_type(IndexType::Byte)
    }
}

impl Default for EmitConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Renders diagnostics, with source snippets, to a writer.
pub struct IoEmitter {
    writer: Option<Box<dyn Write + Send>>,
    conf: EmitConfig,
}

impl std::fmt::Debug for IoEmitter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IoEmitter")
            .field("writer", &"Box<dyn Write>")
            .field("conf", &self.conf)
            .finish()
    }
}

impl IoEmitter {
    pub fn new(writer: Box<dyn Write + Send>) -> Self {
        Self::with_config(writer, EmitConfig::default())
    }
    pub fn sink() -> Self {
        IoEmitter {
            writer: None,
            conf: EmitConfig::default(),
        }
    }
    pub fn with_config(writer: Box<dyn Write + Send>, conf: EmitConfig) -> Self {
        Self {
            writer: Some(writer),
            conf,
        }
    }
}

impl IoEmitter {
    /// Render each of `diags` in turn, looking their spans up in `sm`.
    pub fn emit(&mut self, sm: &SourceMap, diags: impl IntoIterator<Item = Diagnostic>) {
        let Some(writer) = &mut self.writer else { return };
        let mut sources = Sources::new(sm);
        for diag in diags {
            let _ = make_report(&diag, sm, self.conf).write(&mut sources, &mut *writer);
        }
    }
}

/// A span as the renderer wants it: the file, and a range of offsets into it.
type RenderSpan = (FileId, std::ops::Range<usize>);

/// The renderer's view of a [`SourceMap`]. It keeps the renderer's own line index of each file it shows, for as long
/// as one batch of diagnostics takes to write.
struct Sources<'sm> {
    sm: &'sm SourceMap,
    indexed: HashMap<FileId, ariadne::Source<Arc<str>>>,
}

impl<'sm> Sources<'sm> {
    fn new(sm: &'sm SourceMap) -> Self {
        Sources {
            sm,
            indexed: HashMap::new(),
        }
    }
}

impl ariadne::Cache<FileId> for Sources<'_> {
    type Storage = Arc<str>;

    fn fetch(&mut self, id: &FileId) -> Result<&ariadne::Source<Self::Storage>, impl std::fmt::Debug> {
        let sm = self.sm;
        Ok::<_, std::convert::Infallible>(
            self.indexed
                .entry(*id)
                .or_insert_with(|| ariadne::Source::from(sm.get_source(*id).source.clone())),
        )
    }

    fn display<'a>(&self, id: &'a FileId) -> Option<impl std::fmt::Display + 'a> {
        Some(self.sm.get_source(*id).name.clone())
    }
}

fn make_report(diag: &Diagnostic, sm: &SourceMap, conf: EmitConfig) -> Report<'static, RenderSpan> {
    let kind = match diag.level {
        Level::Error => ReportKind::Error,
        Level::Warning => ReportKind::Warning,
    };

    // A diagnostic with no location has nothing to show source for, labels included.
    let nowhere = || (FileId::DUMMY, 0..0);
    let place = |span: Span| {
        if diag.loc.is_some() {
            sm.local_span(span)
        } else {
            nowhere()
        }
    };

    let mut report = Report::build(kind, diag.loc.map_or_else(nowhere, |loc| sm.local_span(loc)))
        .with_config(conf.renderer())
        .with_message(&diag.message);
    if let Some(note) = &diag.note {
        report = report.with_note(note);
    }
    for label in &diag.labels {
        let color = match label.kind {
            LabelKind::Primary => ariadne::Color::Red,
            LabelKind::Secondary => ariadne::Color::Yellow,
        };
        report = report.with_label(
            ariadne::Label::new(place(label.span))
                .with_message(&label.message)
                .with_color(color),
        );
    }
    report.finish()
}
