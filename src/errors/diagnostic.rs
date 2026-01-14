/// Clone of no_file_diagnostic except the spans are complete (file specific)
use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Diagnostic<'fid> {
    pub level: Level,
    pub code: Option<String>,
    pub message: String,
    pub labels: Vec<(EtaSpan<'fid>, String, Color)>,
    pub note: Option<String>,
}

/// Diagnostic Builder except every state is valid so we don't need an explicit builder struct
impl<'fid> Diagnostic<'fid> {
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

    pub fn with_primary_label(mut self, span: EtaSpan<'fid>, message: impl Into<String>) -> Self {
        self.labels.push((span, message.into(), Color::Red));
        self
    }

    pub fn with_secondary_label(mut self, span: EtaSpan<'fid>, message: impl Into<String>) -> Self {
        self.labels.push((span, message.into(), Color::Yellow));
        self
    }

    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.note = Some(note.into());
        self
    }
}

/// Required to be ariadne::Span
impl<'fid> Default for Diagnostic<'fid> {
    fn default() -> Self {
        Self::error("Default error message")
    }
}
