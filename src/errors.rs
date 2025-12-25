use std::fmt::Debug;

use ariadne::{Color, Label, Report, ReportKind};

use crate::sources::{EtaSpan, FileId, SourceManager};

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        Diagnostic::error(format!($($arg)*))
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        Diagnostic::warning(format!($($arg)*))
    };
}

#[macro_export]
macro_rules! note {
    ($($arg:tt)*) => {
        Diagnostic::note(format!($($arg)*))
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Level {
    Error,
    Warning,
    Note,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Diagnostic {
    pub level: Level,
    pub code: Option<String>,
    pub message: String,
    pub labels: Vec<(EtaSpan, String, Color)>,
    pub note: Option<String>,
}

impl Diagnostic {
    pub fn error(message: impl Into<String>) -> Self {
        Self {
            level: Level::Error,
            code: None,
            message: message.into(),
            labels: Vec::new(),
            note: None,
        }
    }

    pub fn warning(message: impl Into<String>) -> Self {
        Self {
            level: Level::Warning,
            code: None,
            message: message.into(),
            labels: Vec::new(),
            note: None,
        }
    }

    pub fn note(message: impl Into<String>) -> Self {
        Self {
            level: Level::Note,
            code: None,
            message: message.into(),
            labels: Vec::new(),
            note: None,
        }
    }

    pub fn with_code(mut self, code: impl Into<String>) -> Self {
        self.code = Some(code.into());
        self
    }

    pub fn with_primary_label(mut self, span: EtaSpan, message: impl Into<String>) -> Self {
        self.labels.push((span, message.into(), Color::Red));
        self
    }

    pub fn with_secondary_label(mut self, span: EtaSpan, message: impl Into<String>) -> Self {
        self.labels.push((span, message.into(), Color::Yellow));
        self
    }

    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.note = Some(note.into());
        self
    }
}

impl Default for Diagnostic {
    fn default() -> Self {
        Self::error("Default error message")
    }
}

//// Interface rest of project with ariadne

impl ariadne::Span for EtaSpan {
    type SourceId = FileId;

    fn source(&self) -> &Self::SourceId {
        &self.file_id
    }

    fn start(&self) -> usize {
        self.range.start
    }

    fn end(&self) -> usize {
        self.range.end
    }
}

impl SourceManager {
    pub fn emit(&self, diag: Diagnostic, span: EtaSpan) {
        let fid = span.file_id;

        let kind = match diag.level {
            Level::Error => ReportKind::Error,
            Level::Warning => ReportKind::Warning,
            Level::Note => ReportKind::Advice,
        };

        let mut builder = Report::build(kind, span).with_message(diag.message);

        if let Some(code) = diag.code {
            builder = builder.with_code(code);
        }

        if let Some(note) = diag.note {
            builder = builder.with_note(note);
        }

        for (span, label_msg, color) in diag.labels {
            builder =
                builder.with_label(Label::new(span).with_message(label_msg).with_color(color));
        }

        // Print to stderr
        if let Some(src) = self.get_source(fid) {
            // Pass a tuple of (FileId, Source) directly as the cache
            let _ = builder.finish().eprint((fid, ariadne::Source::from(src)));
        }
    }
}
