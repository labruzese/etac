use etac_driver::{CompilationFailure, CompilationSuccess};

const ANSI_BOLD_GREEN: &str = "\x1b[1;32m";
const ANSI_BOLD_RED: &str = "\x1b[1;31m";
const ANSI_RESET: &str = "\x1b[0m";

fn main() -> std::process::ExitCode {
    // the same rule the diagnostics follow, so the summary never has colors they do not
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
            eprintln!("{green}Compiled with {warnings} warnings{reset}");
            std::process::ExitCode::SUCCESS
        }
        Err(CompilationFailure { errors, warnings }) => {
            let (red, reset) = paint(ANSI_BOLD_RED);
            eprintln!("{red}Compilation failed with {errors} errors and {warnings} warnings{reset}");
            std::process::ExitCode::FAILURE
        }
    }
}
