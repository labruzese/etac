use clap::{CommandFactory, Parser};
use std::{path::PathBuf};

#[derive(Debug, Clone, Parser)]
#[command(name = "etac", about = "Rust Implementation of Eta Compiler")]
pub struct Flags {
    /// Specify where to find input source files.
    ///
    /// Relative source-file paths on the command line are resolved against
    /// this directory. The default is the current directory in which etac is
    /// run. The course-spec spelling `-sourcepath` is also accepted.
    #[arg(short = 's', long = "sourcepath", value_name = "PATH", default_value = ".")]
    pub source_path: PathBuf,

    /// Specify where to find library interface files.
    ///
    /// A `use F` declaration searches for `F.eti` next to the using source
    /// file first, then in this directory. The default is the current
    /// directory in which etac is run. The course-spec spelling `-libpath`
    /// is also accepted.
    #[arg(short = 'l', long = "libpath", value_name = "PATH", default_value = ".")]
    pub lib_path: PathBuf,

    /// Source files to compile (relative paths resolve against --sourcepath).
    #[arg(value_name = "SOURCE_FILES")]
    pub source_files: Vec<PathBuf>,

    /// Just run the lexer and print diagnostic output
    #[arg(long)]
    pub dlex: bool,

    /// Just run the paser and print diagnostic output
    #[arg(long)]
    pub dparse: bool,
}

#[must_use]
pub fn parse_flags() -> Flags {
    let flags = Flags::parse();

    if flags.source_files.is_empty() {
        let _ = Flags::command().print_help();
        std::process::exit(0);
    }

    flags
}

