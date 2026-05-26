#![allow(unused)]

use std::{fmt::Debug, ops::Range, rc::Rc};
use std::convert::Infallible;
use ariadne::{Color, Label, Report, ReportKind};

use etac_span::{EtaSpan, FileId, SourceId, Sources};

#[macro_export]
macro_rules! error {
    ($name:expr, $span:expr; $($arg:tt)*) => {
        $crate::Diagnostic::new($crate::Level::Error, ($name, $span).into(), format!($($arg)*))
    };
    ($name:expr; $($arg:tt)*) => {
        $crate::Diagnostic::new_no_loc($crate::Level::Error, $name, format!($($arg)*))
    };
    ($($arg:tt)*) => {
        $crate::Diagnostic::new_generic($crate::Level::Error, format!($($arg)*))
    };
}

// these need to be fixed before they can be uncommented
// #[macro_export]
// macro_rules! warn {
//     ($name:ident, $span:expr, $($arg:tt)*) => {
//         $crate::Diagnostic::new($crate::Level::Warning, ($name, $span).into(), format!($($arg)*))
//     };
//     ($name:ident, $($arg:tt)*) => {
//         $crate::Diagnostic::new_no_loc($crate::Level::Warning, $name, format!($($arg)*))
//     };
//     ($($arg:tt)*) => {
//         $crate::Diagnostic::new_generic($crate::Level::Warning, format!($($arg)*))
//     };
// }
//
// #[macro_export]
// macro_rules! note {
//     ($name:expr, $span:expr, $($arg:tt)*) => {
//         $crate::Diagnostic::new($crate::Level::Note, ($name, $span).into(), format!($($arg)*))
//     };
//     ($name:expr, $($arg:tt)*) => {
//         $crate::Diagnostic::new_no_loc($crate::Level::Note, $name, format!($($arg)*))
//     };
//     ($($arg:tt)*) => {
//         $crate::Diagnostic::new_generic($crate::Level::Note, format!($($arg)*))
//     };
// }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Level {
    Error,
    Warning,
    Note,
}

mod diagnostic;
pub use diagnostic::*;

/// write the diagnostic to stderr (pretty)
pub fn emit(sources: &mut Sources, diag: Diagnostic) {
    let kind = match diag.level {
        Level::Error   => ReportKind::Error,
        Level::Warning => ReportKind::Warning,
        Level::Note    => ReportKind::Advice,
    };
    static NO_SPAN: NoSpan = NoSpan {};
    static NO_CACHE: NoCache = NoCache {};

    match diag.loc {
        Some(loc) => {
            let mut b = Report::build(kind, loc)
                .with_message(diag.message);
            if let Some(c) = diag.code { b = b.with_code(c); }
            if let Some(n) = diag.note { b = b.with_code(n); }
            for (span, msg, color) in diag.labels {
                b = b.with_label(Label::new(span).with_message(msg).with_color(color));
            }
            let _ = b.finish().eprint(sources);
        },
        None      => {
            let mut b = Report::build(kind, NO_SPAN)
                .with_message(diag.message);
            if let Some(c) = diag.code { b = b.with_code(c); }
            if let Some(n) = diag.note { b = b.with_code(n); }
            for (span, msg, color) in diag.labels {
                b = b.with_label(Label::new(NO_SPAN).with_message(msg).with_color(color));
            }
            let _ = b.finish().eprint(NO_CACHE);
        },
    };
}

#[derive(Clone, Copy)]
/// dummy struct for satisfying ariadne when we don't have a source
pub struct NoSpan {}
#[derive(Clone, Copy)]
/// dummy struct for satisfying ariadne when we don't have a source
pub struct NoCache {}
impl ariadne::Span for NoSpan {
    type SourceId = ();
    fn source(&self) -> &Self::SourceId {&()}
    fn start(&self) -> usize {0}
    fn end(&self) -> usize {0}
}
impl ariadne::Cache<()> for NoCache {
    type Storage = &'static str;
    fn fetch(&mut self, id: &()) -> Result<&ariadne::Source<Self::Storage>, impl std::fmt::Debug> {
        static SOURCE: std::sync::LazyLock<ariadne::Source<&'static str>> 
                     = std::sync::LazyLock::new(||ariadne::Source::from(""));
        Ok::<_, Infallible>(&SOURCE)
    }
    fn display<'a>(&self, id: &'a ()) -> Option<impl std::fmt::Display + 'a> {
        Some("")
    }
}
