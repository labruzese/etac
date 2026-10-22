use etac_errors::dcx::DiagCtx;

#[derive(Debug)]
pub struct CompilationFailure {
    pub errors: usize,
    pub warnings: usize,
}
impl From<&DiagCtx> for CompilationFailure {
    fn from(value: &DiagCtx) -> Self {
        CompilationFailure {
            errors: value.err_count(),
            warnings: value.warn_count(),
        }
    }
}
pub struct CompilationSuccess {
    pub warnings: usize,
}
impl From<&DiagCtx> for CompilationSuccess {
    fn from(value: &DiagCtx) -> Self {
        CompilationSuccess {
            warnings: value.warn_count(),
        }
    }
}
