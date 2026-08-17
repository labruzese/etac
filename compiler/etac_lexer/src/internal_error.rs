use etac_source::Span;

/// Internal lexer error type.
///
/// This is `pub` only because `logos`'s generated `Logos` impl for `Token`
/// requires the associated error type to be at least as visible as `Token`
/// itself (rustc E0446). It is **not** part of this crate's public API,
/// carries no semver guarantees, and should not be named or matched on by
/// downstream crates.
#[doc(hidden)]
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct InternalLexerError {
    pub(crate) span: Span,
    pub(crate) message: String,
    pub(crate) plabel: Option<String>,
    pub(crate) note: Option<String>,
}

impl Default for InternalLexerError {
    fn default() -> Self {
        panic!("Lexer should never use the default error")
    }
}
