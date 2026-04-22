use std::cell::{Cell, RefCell};

/// Global flags for this program
static FLAGS: Cell<Flags> = OnceLock::new();
static SOURCES: RefCell<Sources> = RefCell::new(sources::sources_from_disk());
static LOGGER: RefCell<Logger> = RefCell::new({
    cli::init();
    logger::Logger::new(cli::flags())
});

/// Parses arguments from CLI and initializes the global FLAGS.
pub fn init() {
    let flags = Flags::parse();

    // Requirement: Invoking etac without any source files should also print a synopsis.
    if flags.source_files.is_empty() {
        let _ = Flags::command().print_help();
        std::process::exit(0);
    }

    FLAGS.set(flags).expect("Flags already initialized");
}

pub fn flags() -> &'static Flags {
    FLAGS.get().expect("Flags not initialized")
}

/// fetches the requested source, loading it if not in cache; panic if it doesn't exist
#[macro_export]
macro_rules! source {
    ($file_id:expr) => {{
        use ariadne::Cache;
        crate::SOURCES.with_borrow_mut(|sources| {
            sources
                .fetch(&$file_id)
                .expect(&format!("couldn't find {}", $file_id))
        })
    }};
}

/// fetches the requested source, loading it if not in cache; panic if it doesn't exist
#[macro_export]
macro_rules! logger {
    ($action:expr) => {{
        crate::LOGGER.with_borrow_mut($action)
    }};
}
