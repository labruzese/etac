//! A diagnostic, or a whole context of them, must not be forgotten: dropping one unreported panics.
//!
//! The bombs behind that only exist with debug assertions, so these tests do too.
#![cfg(debug_assertions)]

use etac_errors::dcx::DiagCtx;
use etac_errors::emitter::IoEmitter;
use etac_source::SourceMap;

#[test]
#[should_panic(expected = "BOOM: Diag dropped without writing diagnostic to context")]
fn a_diagnostic_goes_off_when_forgotten() {
    let dcx = DiagCtx::new(IoEmitter::sink());
    let _forgotten = dcx.new_err_unspanned("never finished");
}

#[test]
#[should_panic(expected = "BOOM: DiagCtx dropped without writing diagnostics to emitter")]
fn a_context_goes_off_when_forgotten() {
    let _forgotten = DiagCtx::new(IoEmitter::sink());
}

#[test]
fn stays_quiet_when_defused() {
    // a diagnostic is defused by reporting it or by withdrawing it
    let dcx = DiagCtx::new(IoEmitter::sink());
    let _ = dcx.new_err_unspanned("reported").finish();
    dcx.new_err_unspanned("withdrawn").cancel();
    // and a context by writing its diagnostics out, or by saying it should not
    dcx.emit_diagnostics(&SourceMap::default());
    DiagCtx::new(IoEmitter::sink()).cancel();
}

/// A panic with live bombs in scope must unwind as itself. If a bomb went off too, the double panic would abort the
/// test process instead of being caught here.
#[test]
fn does_not_go_off_during_another_panic() {
    let unwound = std::panic::catch_unwind(|| {
        let dcx = DiagCtx::new(IoEmitter::sink());
        let _forgotten = dcx.new_err_unspanned("never finished");
        panic!("the real problem");
    });
    let message = unwound.unwrap_err();
    assert_eq!(message.downcast_ref::<&str>(), Some(&"the real problem"));
}
