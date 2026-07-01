//! Compiler diagnostics.
//!
//! The pipeline is: build a [`Diagnostic`] (plain data), then route it through a
//! [`DiagCtxt`] — the single context that owns an [`Emitter`] and the error/warning
//! counts. Nothing emits except through that context, and an emitted error yields an
//! [`ErrorGuaranteed`] proof token so "we failed" can be made to entail "the user was
//! told why" at the type level.
//!
//! * Plain-data producers (the lexer's logos callbacks, lalrpop's recovered errors) keep
//!   constructing [`Diagnostic`]s directly — often via the [`error!`] macro — because they
//!   have no context on hand. The layer above funnels them in with [`DiagCtxt::emit`].
//! * Code that holds a `&DiagCtxt` should prefer the builders ([`DiagCtxt::err`] etc.),
//!   which return a must-use [`Diag`] that emits or cancels before it drops.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Denotes the severity of the Diagnostic
pub enum Level {
    Error,
    Warning,
    Note,
}

mod dcx;
mod emitter;
mod drop_bomb;
mod macros;

pub use dcx::*;
pub use emitter::*;
