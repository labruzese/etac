use etac_errors::dcx::Diag;

#[derive(Debug)]
pub enum Parsed<Out> {
    Ok(Out),
    Recovered(Out),
    /// we actually never fail, but callers should handle a case where we fail anyway.
    Failed,
}

impl<Out> Parsed<Out> {
    pub fn output(&self) -> Option<&Out> {
        match self {
            Parsed::Ok(ast) | Parsed::Recovered(ast) => Some(ast),
            Parsed::Failed => None,
        }
    }
}

/// The tree plus every diagnostic the parse produced.
///
/// The diagnostics are *unemitted*: each one is a `Diag` the caller still has to
/// `.finish()` (or `.cancel()`). Lets a caller decorate or
/// filter before anything reaches the emitter.
pub struct ParseResult<'dcx, Out> {
    pub parse: Parsed<Out>,
    pub diags: Vec<Diag<'dcx>>,
}
