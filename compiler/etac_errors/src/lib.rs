//! Compiler diagnostics.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Denotes the severity of the Diagnostic
pub enum Level {
    Error,
    Warning,
    Note,
}

pub mod dcx;
pub mod emitter;
mod macros;
pub mod guarentee;

#[cfg(debug_assertions)]
mod drop_bomb;
