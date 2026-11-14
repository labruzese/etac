use etac_errors::guarantee::ErrorGuaranteed;

/// The outcome of a parse. There is always a tree: the parser recovers from every error it reports, leaving error
/// nodes standing in for what it could not make sense of.
///
/// The diagnostics themselves have already gone to the diagnostic context, as they were found.
#[derive(Debug, Clone)]
pub struct ParseResult<Out> {
    pub tree: Out,
    /// Proof that the tokens did not parse cleanly, if they did not: the parser reported an error, or the lexer had
    /// reported one while making the tokens.
    pub errors: Option<ErrorGuaranteed>,
}

impl<Out> ParseResult<Out> {
    /// Whether the input was lexically and syntactically valid, so the tree has no error nodes in it.
    pub fn is_clean(&self) -> bool {
        self.errors.is_none()
    }
}
