use std::{fmt::Debug};
use std::convert::Infallible;
use ariadne::{Color, Label, Report, ReportKind};

use etac_span::{Span, SourceCache};

#[macro_export]
/// Creates a new Level::Error Diagnostic with a provided message. 
/// Note the syntax is to have a semicolon (`;`) after the span.
/// `error!(span; "no identifier called {}", id)` => Diagnostic with span
/// `error!("file does not exist")` => Diagnostic *without* a span
macro_rules! error {
    ($span:expr; $($arg:tt)*) => {
        $crate::Diagnostic::new($crate::Level::Error, $span, format!($($arg)*))
    };
    ($($arg:tt)*) => {
        $crate::Diagnostic::new_no_loc($crate::Level::Error, format!($($arg)*))
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Denotes the severity of the Diagnostic
pub enum Level {
    Error,
    Warning,
    Note,
}

// re-export diagnostic
mod diagnostic;
pub use diagnostic::*;

/// write the diagnostic to stderr (pretty)
/// consumes the diagnostic so it can only be emitted once after you have done all the modifications
pub fn emit(source_cache: &mut SourceCache, diag: Diagnostic) {
    let kind = match diag.level {
        Level::Error   => ReportKind::Error,
        Level::Warning => ReportKind::Warning,
        Level::Note    => ReportKind::Advice,
    };
    static NO_SPAN: NoSpan = NoSpan {};
    static NO_CACHE: NoCache = NoCache {};

    match diag.loc {
        Some(loc) => {
            let floc = source_cache.resolve(loc);
            let mut b = Report::build(kind, floc)
                .with_message(diag.message);
            if let Some(c) = diag.code { b = b.with_code(c); }
            if let Some(n) = diag.note { b = b.with_code(n); }
            for (span, msg, color) in diag.labels {
                let fspan = source_cache.resolve(span);
                b = b.with_label(Label::new(fspan).with_message(msg).with_color(color));
            }
            let _ = b.finish().eprint(source_cache);
        },
        None      => {
            let mut b = Report::build(kind, NO_SPAN)
                .with_message(diag.message);
            if let Some(c) = diag.code { b = b.with_code(c); }
            if let Some(n) = diag.note { b = b.with_code(n); }
            for (_span, msg, color) in diag.labels {
                //warn!("span added to a label of a diagnostic that doesn't have a location. It currently isn't possible for this span to get reported");
                b = b.with_label(Label::new(NO_SPAN).with_message(msg).with_color(color));
            }
            let _ = b.finish().eprint(NO_CACHE);
        },
    };
}

#[derive(Clone, Copy)]
/// Dummy struct for satisfying ariadne when we don't have a source. 0-sized and all of it's impls
/// are no-ops (or as close as they can be).
pub struct NoSpan {}
#[derive(Clone, Copy)]
/// Dummy struct for satisfying ariadne when we don't have a source. 0-sized and all of it's impls
/// are no-ops (or as close as they can be).
pub struct NoCache {}
impl ariadne::Span for NoSpan {
    type SourceId = ();
    fn source(&self) -> &Self::SourceId {&()}
    fn start(&self) -> usize {0}
    fn end(&self) -> usize {0}
}
impl ariadne::Cache<()> for NoCache {
    type Storage = &'static str;
    fn fetch(&mut self, _id: &()) -> Result<&ariadne::Source<Self::Storage>, impl std::fmt::Debug> {
        static SOURCE: std::sync::LazyLock<ariadne::Source<&'static str>> 
                     = std::sync::LazyLock::new(||ariadne::Source::from(""));
        Ok::<_, Infallible>(&SOURCE)
    }
    fn display<'a>(&self, _id: &'a ()) -> Option<impl std::fmt::Display + 'a> {
        Some("")
    }
}
