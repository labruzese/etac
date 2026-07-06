//! Compiler diagnostics.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Denotes the severity of the Diagnostic
pub enum Level {
    Error,
    Warning,
    Note,
}

mod dcx;
mod emitter;
mod macros;

#[cfg(debug_assertions)]
mod drop_bomb;

pub use dcx::*;
pub use emitter::*;
