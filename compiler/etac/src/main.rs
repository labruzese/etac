fn main() -> std::process::ExitCode {
    env_logger::init();
    match etac_driver::run(etac_session::cli::parse_flags()) {
        Ok(_) => std::process::ExitCode::SUCCESS,
        Err(_) => std::process::ExitCode::FAILURE,
    }
}
