use std::cell::RefCell;
use crate::{
    logger::Logger,
    sources::{FileId, Sources},
};

mod ast;
mod cli;
mod errors;
mod lexer;
mod logger;
mod parser;
mod sources;

// initialization of resources
// cli
// logger
// sources (lazy)
thread_local! {
    pub static SOURCES: RefCell<Sources> = RefCell::new(sources::sources_from_disk());
    pub static LOGGER: RefCell<Logger> = RefCell::new({
        cli::init();
        logger::Logger::new(cli::flags())
    });
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


fn main() {
    env_logger::init();
    let sources = cli::flags()
        .source_files
        .iter()
        .map(|x| FileId::new(x.to_str().expect("non-standard file names not supported")));

    for source in sources {
        let res = parser::parse(&source);
        match res {
            Ok(_program) => (), //println!("{_program}"),
            Err(diags) => diags
                .into_iter()
                .for_each(|d| source_manager.emit(d, &source.name)),
        }
    }
}
