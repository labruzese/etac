//! Only an error proves that the compilation failed.

use etac_errors::dcx::DiagCtx;
use etac_errors::emitter::IoEmitter;
use etac_errors::guarantee::ErrorGuaranteed;
use etac_source::Span;

#[test]
fn an_error_gives_a_guarantee_and_a_warning_does_not() {
    let dcx = DiagCtx::new(IoEmitter::sink());
    let span = Span::new(0u32, 1u32);

    // The types are the test: a warning's `finish` returns `()`, so there is no `ErrorGuaranteed` to mistake it for.
    let _: ErrorGuaranteed = dcx.new_err(span, "an error").finish();
    let _: ErrorGuaranteed = dcx.new_err_unspanned("another").finish();
    let () = dcx.new_warn(span, "a warning").finish();

    assert_eq!((dcx.err_count(), dcx.warn_count()), (2, 1));
    assert!(dcx.has_errors());
    dcx.cancel();
}

#[test]
fn warnings_alone_are_not_errors() {
    let dcx = DiagCtx::new(IoEmitter::sink());
    dcx.new_warn(Span::new(0u32, 1u32), "a warning")
        .with_note("with a note")
        .finish();
    assert!(!dcx.has_errors());
    assert_eq!(dcx.warn_count(), 1);
    dcx.cancel();
}
