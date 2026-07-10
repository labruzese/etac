use etac_errors::DiagCtxt;
use etac_span::SourceCache;

#[derive(Debug)]
pub struct CompilationFailure {
    pub errors: usize,
    pub warnings: usize,
}
impl<C: SourceCache> From<&DiagCtxt<C>> for CompilationFailure {
    fn from(value: &DiagCtxt<C>) -> Self {
        CompilationFailure {
            errors: value.err_count(),
            warnings: value.warn_count(),
        }
    }
}
pub struct CompilationSuccess {
    pub warnings: usize
}
impl<C: SourceCache> From<&DiagCtxt<C>> for CompilationSuccess {
    fn from(value: &DiagCtxt<C>) -> Self {
        CompilationSuccess {
            warnings: value.warn_count(),
        }
    }
}
