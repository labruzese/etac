use etac_errors::dcx::DiagBuilder;

/// The outcome of a parse. There is always a tree: the parser recovers from every error it reports.
#[derive(Debug)]
pub enum Parsed<Out> {
    /// The input was syntactically valid.
    Ok(Out),
    /// The parser reported errors. The tree has error nodes standing in for what it could not make sense of.
    Recovered(Out),
}

impl<Out> Parsed<Out> {
    pub fn tree(&self) -> &Out {
        match self {
            Parsed::Ok(tree) | Parsed::Recovered(tree) => tree,
        }
    }

    pub fn into_tree(self) -> Out {
        match self {
            Parsed::Ok(tree) | Parsed::Recovered(tree) => tree,
        }
    }

    /// Whether the parser got through without reporting anything.
    pub fn is_ok(&self) -> bool {
        matches!(self, Parsed::Ok(_))
    }
}

/// The tree plus every diagnostic the parse produced.
///
/// The diagnostics are *unemitted*: each one is a `Diag` the caller still has to
/// `.finish()` (or `.cancel()`). Lets a caller decorate or
/// filter before anything reaches the emitter.
pub struct ParseResult<'dcx, Out> {
    pub parse: Parsed<Out>,
    pub diags: Vec<DiagBuilder<'dcx>>,
}
