//! The exit status is the compiler's verdict on the program, whoever is still listening.

use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// A directory of our own to write source files in.
fn scratch_dir() -> PathBuf {
    let dir = std::env::temp_dir().join(format!("etac-closed-pipe-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

/// A pipe nobody will ever read from, so every write to it fails.
fn closed_pipe() -> Stdio {
    let (reader, writer) = std::io::pipe().unwrap();
    drop(reader);
    writer.into()
}

/// Run `etac --dlex --dparse` on `file` with both of its output streams closed, and return its exit code.
fn exit_code_with_nobody_reading(file: &Path) -> Option<i32> {
    Command::new(env!("CARGO_BIN_EXE_etac"))
        .args(["--dlex", "--dparse"])
        .arg(file)
        .stdin(Stdio::null())
        .stdout(closed_pipe())
        .stderr(closed_pipe())
        .status()
        .unwrap()
        .code()
}

/// As with `etac .. 2>&1 | head`, once `head` has read its fill and gone.
#[test]
fn closed_output_does_not_change_the_exit_status() {
    let dir = scratch_dir();
    let good = dir.join("good.eta");
    let bad = dir.join("bad.eta");
    std::fs::write(&good, "f() { }\n").unwrap();
    std::fs::write(&bad, "f() { x = }\n").unwrap();

    // a panic would exit with 101, and say nothing about the program
    assert_eq!(exit_code_with_nobody_reading(&good), Some(0));
    assert_eq!(exit_code_with_nobody_reading(&bad), Some(1));

    std::fs::remove_dir_all(&dir).unwrap();
}
