//! What rendered diagnostics look like.

use std::io::Write;
use std::sync::{Arc, Mutex};

use etac_errors::dcx::DiagCtx;
use etac_errors::emitter::{EmitConfig, IoEmitter};
use etac_source::{SourceMap, Span};

#[derive(Clone, Default)]
struct Buf(Arc<Mutex<Vec<u8>>>);

impl Write for Buf {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        self.0.lock().unwrap().extend_from_slice(bytes);
        Ok(bytes.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

/// Report with `report` against a map holding `a.eta`, and return what gets written.
fn render(report: impl FnOnce(&DiagCtx, &dyn Fn(u32, u32) -> Span)) -> String {
    let out = Buf::default();
    let dcx = DiagCtx::new(IoEmitter::with_config(
        Box::new(out.clone()),
        EmitConfig::new().with_color(false),
    ));
    let sm = SourceMap::default();
    let (id, _) = sm.alloc_source("a.eta".to_owned(), "f() {\n    x = y\n}\n".to_owned());
    let base = sm.base_offset(id);
    report(&dcx, &|lo, hi| Span::new(base + lo, base + hi));
    dcx.emit_diagnostics(&sm);
    String::from_utf8(out.0.lock().unwrap().clone()).unwrap()
}

/// The `y` on line 2.
const Y: (u32, u32) = (14, 15);

#[test]
fn a_diagnostic_without_a_label_still_shows_where_it_is() {
    let out = render(|dcx, span| {
        let _ = dcx.new_err(span(Y.0, Y.1), "no label").finish();
    });
    assert!(out.contains("Error: no label"), "{out}");
    assert!(out.contains("a.eta:2:9"), "{out}");
    assert!(out.contains("x = y"), "{out}");
}

#[test]
fn a_secondary_label_alone_does_not_hide_the_primary_location() {
    let out = render(|dcx, span| {
        let _ = dcx
            .new_err(span(Y.0, Y.1), "only secondary")
            .with_label(span(0, 1), "declared here")
            .finish();
    });
    assert!(out.contains("a.eta:2:9"), "{out}");
    assert!(out.contains("declared here") && out.contains("x = y"), "{out}");
}

#[test]
fn every_note_is_shown_in_order() {
    let out = render(|dcx, span| {
        let _ = dcx
            .new_err(span(Y.0, Y.1), "two notes")
            .with_note("first note")
            .with_note("second note")
            .finish();
    });
    let first = out.find("first note").unwrap_or_else(|| panic!("{out}"));
    let second = out.find("second note").unwrap_or_else(|| panic!("{out}"));
    assert!(first < second, "{out}");
}

#[test]
fn a_diagnostic_without_a_location_is_just_its_message() {
    let out = render(|dcx, _| {
        let _ = dcx
            .new_err_unspanned("nowhere in particular")
            .with_note("a note")
            .finish();
    });
    assert!(out.contains("Error: nowhere in particular"), "{out}");
    assert!(out.contains("a note"), "{out}");
    assert!(!out.contains("a.eta"), "{out}");
}

#[test]
fn a_label_on_a_diagnostic_without_a_location_is_shown_where_it_points() {
    let out = render(|dcx, span| {
        let _ = dcx
            .new_err_unspanned("nowhere in particular")
            .with_label(span(Y.0, Y.1), "but see this")
            .finish();
    });
    assert!(out.contains("Error: nowhere in particular"), "{out}");
    assert!(out.contains("a.eta:2:9"), "{out}");
    assert!(out.contains("but see this"), "{out}");
}

#[test]
fn diagnostics_come_out_in_source_order() {
    let out = render(|dcx, span| {
        let _ = dcx.new_err(span(Y.0, Y.1), "third: line 2, reported first").finish();
        let _ = dcx.new_err(span(0, 1), "second: line 1").finish();
        let _ = dcx.new_err(span(Y.0, Y.1), "fourth: line 2, reported later").finish();
        let _ = dcx.new_err_unspanned("first: no location").finish();
    });
    let position = |text: &str| out.find(text).unwrap_or_else(|| panic!("{out}"));
    let order = ["first", "second", "third", "fourth"].map(position);
    assert!(order.is_sorted(), "{out}");
}
