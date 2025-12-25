use std::{path::PathBuf, sync::OnceLock};

#[derive(Debug, Clone)]
pub struct Flags {
    pub lex: bool,
    pub lex_file: PathBuf,
}

// gets init'd once then theres no overhead, everything is readonly
static FLAGS: OnceLock<Flags> = OnceLock::new();

pub fn init_flags(flags: Flags) {
    FLAGS.set(flags).expect("Flags already initialized");
}

pub fn flags() -> &'static Flags {
    FLAGS.get().expect("Flags not initialized")
}
