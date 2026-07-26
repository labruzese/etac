use etac_errors::dcx::{Diag, DiagCtx};
use etac_lexer::token;
use lalrpop_util::ErrorRecovery;

/// Mutable state threaded through grammar.
pub struct ParseState<'dcx> {
    pub dcx: &'dcx DiagCtx,
    pub lalrpop_errs: Vec<ErrorRecovery<u32, token::RawToken, Diag<'dcx>>>,
    pub etac_errs: Vec<Diag<'dcx>>,
}

impl<'dcx> ParseState<'dcx> {
    pub fn new(dcx: &'dcx DiagCtx) -> Self {
        ParseState {
            dcx,
            lalrpop_errs: Vec::new(),
            etac_errs: Vec::new(),
        }
    }
}
