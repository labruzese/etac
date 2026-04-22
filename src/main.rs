mod ast;
mod cli;
mod context;
mod errors;
mod lexer;
mod logger;
mod parser;
mod sources;

use context::Context;
use sources::FileId;

fn main() {
    env_logger::init();

    let mut ctx = Context::new(cli::parse_flags());

    let files: Vec<FileId> = ctx.flags.source_files.iter()
        .map(|p| FileId::new(p.to_str().expect("non-standard filename")))
        .collect();

    for file_id in &files {
        match parser::parse(&mut ctx, file_id) {
            Ok(_program) => {}
            Err(diags) => {
                for d in diags {
                    errors::emit(&mut ctx.sources, d);
                }
            }
        }
    }

    ctx.logger.flush();
}
