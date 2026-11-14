use std::io::Write;

use etac_driver::{CompilationFailure, CompilationSuccess};

const ANSI_BOLD_GREEN: &str = "\x1b[1;32m";
const ANSI_BOLD_RED: &str = "\x1b[1;31m";
const ANSI_RESET: &str = "\x1b[0m";

fn main() -> std::process::ExitCode {
    // in accordance with [`etac_errors::emitter`]
    let paint = |color: &'static str| {
        if etac_errors::emitter::stderr_wants_color() {
            (color, ANSI_RESET)
        } else {
            ("", "")
        }
    };

    match etac_driver::run(&etac_cli::parse_flags()) {
        Ok(CompilationSuccess { warnings }) => {
            let (green, reset) = paint(ANSI_BOLD_GREEN);
            summarize(format_args!("{green}Compiled with {warnings} warnings{reset}"));
            std::process::ExitCode::SUCCESS
        }
        Err(CompilationFailure { errors, warnings }) => {
            let (red, reset) = paint(ANSI_BOLD_RED);
            summarize(format_args!(
                "{red}Compilation failed with {errors} errors and {warnings} warnings{reset}"
            ));
            std::process::ExitCode::FAILURE
        }
    }
}

/// Write the closing line to standard error, if anyone is still reading it.
///
/// `eprintln!` panics when the write fails, which it does once the reader has gone: `etac .. 2>&1 | head` closes the
/// pipe early. That would turn the exit status into a panic's, whatever the compilation's verdict was, so a line
/// that cannot be written is dropped instead. Diagnostics and the debug dumps are written the same way.
fn summarize(line: std::fmt::Arguments<'_>) {
    let _ = writeln!(std::io::stderr(), "{line}");
}
