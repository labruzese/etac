//! Ergonomic constructors for diagnostics: [`etac_error!`] and [`etac_warn!`].
//!
//! These macros are thin, declarative sugar over the [`DiagCtxt`](crate::DiagCtxt)
//! builder API. They exist to remove three papercuts from the ~thousands of diagnostic
//! sites a compiler accumulates:
//!
//! 1. **Formatting.** The builders take `impl Into<String>`, so every interesting
//!    message is a `format!(...)` at the call site. The macros fold that in: write
//!    `"expected {}, found {}", a, b` directly, exactly like `println!`.
//! 2. **Span vs. no-span.** [`DiagCtxt::err`] and [`DiagCtxt::err_no_span`] are two
//!    calls; the macro picks the right one from the shape of the arguments.
//! 3. **Decoration boilerplate.** Primary/secondary labels, notes, and codes become a
//!    small `;`-separated block instead of a `.with_*()` chain.
//!
//! # These macros *build*, they do not *emit*
//!
//! An invocation evaluates to a [`Diag`](crate::Diag) — the same builder you'd get from
//! `dcx.err(..)`. You finish it yourself with [`.emit()`](crate::Diag::emit) (or
//! [`.cancel()`](crate::Diag::cancel)). This is intentional and preserves every
//! invariant the diagnostics module is built around:
//!
//! * [`ErrorGuaranteed`](crate::ErrorGuaranteed) is still minted **only** on a real
//!   `.emit()`, so `Result<T, ErrorGuaranteed>` keeps meaning "if `Err`, the user saw
//!   why." The macro has no back door.
//! * The drop-bomb still fires if you build and forget to send. Together with the
//!   `#[must_use]` on `Diag`, that means a forgotten diagnostic is caught at compile
//!   time (lint) *and* at runtime (bomb).
//! * You can still branch: keep the `Diag` in a `let` and add
//!   [`.with_secondary_label()`](crate::Diag::with_secondary_label) conditionally
//!   before emitting.
//!
//! The idiomatic error path is therefore:
//!
//! ```ignore
//! return Err(etac_error!(dcx, span, "type mismatch";
//!     primary: "expected {}, found {}", expected, found;
//! ).emit());
//! ```
//!
//! # Grammar
//!
//! ```text
//! etac_error! ( DCX ,            FMT [, ARG]*  [ ; DECOR ] )   // location-less error
//! etac_error! ( DCX , SPAN ,     FMT [, ARG]*  [ ; DECOR ] )   // error at SPAN
//! etac_warn!  ( DCX , SPAN ,     FMT [, ARG]*  [ ; DECOR ] )   // warning at SPAN
//!
//! DECOR   := (DECORATION ;)* DECORATION [;]        // ; separates; trailing ; optional
//! DECORATION :=
//!       primary        : FMT [, ARG]*              // primary (red) label at the diag's span
//!     | secondary(EXPR): FMT [, ARG]*              // secondary (yellow) label at EXPR span
//!     | note           : FMT [, ARG]*              // trailing note
//!     | code           : EXPR                      // diagnostic code, e.g. "E0308"
//! ```
//!
//! `FMT` is always a **string literal** (`format!`-style, incl. `"{captured}"` args).
//! Requiring a literal is what lets `etac_error!` distinguish a message from a span: the
//! first argument after `DCX` is treated as the message iff it is a literal, otherwise
//! it is the span. To message with a runtime `String`, format it: `"{}", s` or `"{s}"`.
//!
//! Decorations may appear in **any order**, and `secondary` may repeat.
//!
//! # Examples
//!
//! ```ignore
//! // Minimal.
//! etac_error!(dcx, span, "unexpected token").emit();
//!
//! // Formatted message.
//! etac_error!(dcx, span, "cannot find `{}` in this scope", name).emit();
//!
//! // Location-less (I/O, CLI): note there is no span argument.
//! etac_error!(dcx, "could not read {}: {}", path.display(), err).emit();
//!
//! // Fully decorated. `{ }` and `( )` invocation are equivalent.
//! etac_error! { dcx, span, "type mismatch";
//!     primary: "expected `{}`, found `{}`", expected, found;
//!     secondary(def_span): "expected because of this definition";
//!     note: "no implicit coercion from `{}` to `{}` exists", found, expected;
//!     code: "E0308";
//! }.emit();
//!
//! // A warning (span is required — see note below).
//! etac_warn!(dcx, span, "unused variable `{}`", name;
//!     primary: "help: prefix with an underscore: `_{}`", name;
//! ).emit();
//! ```
//!
//! # Note on warnings and spans
//!
//! [`etac_warn!`] requires a span, because [`DiagCtxt`](crate::DiagCtxt) exposes only
//! [`warn`](crate::DiagCtxt::warn) and no `warn_no_span` (a warning is nearly always
//! about a place in the source). If you ever need location-less warnings, add a
//! `warn_no_span` to `DiagCtxt` mirroring `err_no_span`, then give `etac_warn!` the same
//! two no-span arms `etac_error!` has.

/// Build an error [`Diag`](crate::Diag). See the [module docs](self) for full syntax.
///
/// Evaluates to a [`Diag`](crate::Diag); call [`.emit()`](crate::Diag::emit) to report
/// it (which yields the [`ErrorGuaranteed`](crate::ErrorGuaranteed) proof). Two forms:
/// with a leading span (`err`) and without one (`err_no_span`), chosen automatically —
/// the message must be a string literal so the two can be told apart.
///
/// ```ignore
/// etac_error!(dcx, span, "expected {}, found {}", a, b).emit();
/// etac_error!(dcx, "bad --flag `{}`", flag; note: "see --help";).emit();
/// ```
#[macro_export]
macro_rules! etac_error {
    // ---- location-less (message literal comes right after the context) ----
    ($dcx:expr, $fmt:literal $(, $args:expr)* $(,)?) => {
        $dcx.err_no_span($crate::__etac_diag!(@fmt $fmt $(, $args)*))
    };
    ($dcx:expr, $fmt:literal $(, $args:expr)* ; $($decor:tt)*) => {
        $crate::__etac_diag!(@decorate
            $dcx.err_no_span($crate::__etac_diag!(@fmt $fmt $(, $args)*)) ; $($decor)*)
    };

    // ---- at a span ----
    ($dcx:expr, $span:expr, $fmt:literal $(, $args:expr)* $(,)?) => {
        $dcx.err($span, $crate::__etac_diag!(@fmt $fmt $(, $args)*))
    };
    ($dcx:expr, $span:expr, $fmt:literal $(, $args:expr)* ; $($decor:tt)*) => {
        $crate::__etac_diag!(@decorate
            $dcx.err($span, $crate::__etac_diag!(@fmt $fmt $(, $args)*)) ; $($decor)*)
    };
}

/// Build a warning [`Diag`](crate::Diag) at a span. See the [module docs](self).
///
/// Same shape as [`etac_error!`], but a span is required (there is no `warn_no_span`).
///
/// ```ignore
/// etac_warn!(dcx, span, "deprecated"; note: "use `{}` instead", repl;).emit();
/// ```
#[macro_export]
macro_rules! etac_warn {
    ($dcx:expr, $span:expr, $fmt:literal $(, $args:expr)* $(,)?) => {
        $dcx.warn($span, $crate::__etac_diag!(@fmt $fmt $(, $args)*))
    };
    ($dcx:expr, $span:expr, $fmt:literal $(, $args:expr)* ; $($decor:tt)*) => {
        $crate::__etac_diag!(@decorate
            $dcx.warn($span, $crate::__etac_diag!(@fmt $fmt $(, $args)*)) ; $($decor)*)
    };
}

/// Internal engine for [`etac_error!`]/[`etac_warn!`]. Not part of the public API.
///
/// Two internal rule families:
/// * `@fmt` — turn `LITERAL [, ARGS]` into either the literal itself (no args, so no
///   `useless_format` lint) or a `format!(..)` call.
/// * `@decorate` — a tt-muncher that folds the `;`-separated decoration list onto a
///   `Diag` builder, one `.with_*()` call at a time. `;` is the separator so `,` stays
///   free for format arguments; the final `;` is optional (terminal arms).
#[doc(hidden)]
#[macro_export]
macro_rules! __etac_diag {
    // -- @fmt: message / label text --------------------------------------------------
    (@fmt $fmt:literal) => { $fmt };
    (@fmt $fmt:literal, $($args:expr),+ $(,)?) => { format!($fmt, $($args),+) };

    // -- @decorate: fold decorations onto the builder --------------------------------
    // Done: nothing left (an optional trailing `;` has been consumed).
    (@decorate $diag:expr $(;)?) => { $diag };

    // Recursive arms: one decoration, a `;`, then the rest.
    (@decorate $diag:expr; primary: $fmt:literal $(, $a:expr)* ; $($rest:tt)*) => {
        $crate::__etac_diag!(@decorate
            $diag.with_primary_label($crate::__etac_diag!(@fmt $fmt $(, $a)*)) ; $($rest)*)
    };
    (@decorate $diag:expr; secondary($span:expr): $fmt:literal $(, $a:expr)* ; $($rest:tt)*) => {
        $crate::__etac_diag!(@decorate
            $diag.with_secondary_label($span, $crate::__etac_diag!(@fmt $fmt $(, $a)*)) ; $($rest)*)
    };
    (@decorate $diag:expr; note: $fmt:literal $(, $a:expr)* ; $($rest:tt)*) => {
        $crate::__etac_diag!(@decorate
            $diag.with_note($crate::__etac_diag!(@fmt $fmt $(, $a)*)) ; $($rest)*)
    };
    (@decorate $diag:expr; code: $code:expr ; $($rest:tt)*) => {
        $crate::__etac_diag!(@decorate $diag.with_code($code) ; $($rest)*)
    };

    // Terminal arms: the final decoration, no trailing `;` required.
    (@decorate $diag:expr; primary: $fmt:literal $(, $a:expr)*) => {
        $diag.with_primary_label($crate::__etac_diag!(@fmt $fmt $(, $a)*))
    };
    (@decorate $diag:expr; secondary($span:expr): $fmt:literal $(, $a:expr)*) => {
        $diag.with_secondary_label($span, $crate::__etac_diag!(@fmt $fmt $(, $a)*))
    };
    (@decorate $diag:expr; note: $fmt:literal $(, $a:expr)*) => {
        $diag.with_note($crate::__etac_diag!(@fmt $fmt $(, $a)*))
    };
    (@decorate $diag:expr; code: $code:expr) => {
        $diag.with_code($code)
    };
}
