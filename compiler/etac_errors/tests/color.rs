//! When diagnostics are colored.

use std::ffi::OsStr;

use etac_errors::emitter::wants_color;

#[test]
fn color_is_for_terminals_that_have_not_declined_it() {
    assert!(wants_color(true, None));
    assert!(wants_color(true, Some(OsStr::new(""))));
    assert!(!wants_color(true, Some(OsStr::new("1"))));
    assert!(!wants_color(false, None));
    assert!(!wants_color(false, Some(OsStr::new(""))));
}
