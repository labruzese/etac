//! The driver for the compiler
use etac_cache::EtaCache;
use etac_errors::dcx::DiagCtx;
use etac_frontend::{Debugger, Frontend};
use etac_resolve::{File, Resolver};
use etac_session::cli::Flags;

pub use crate::status::{CompilationFailure, CompilationSuccess};

mod status;

type CompilationResult = std::result::Result<CompilationSuccess, CompilationFailure>;

pub fn run(flags: &Flags) -> CompilationResult {
    let mut cache = EtaCache::new();
    let dcx = DiagCtx::default();

    let mut resolver = Resolver::new(&flags.source_path, &flags.lib_path);

    let files: Vec<File<'_>> = flags
        .source_files
        .iter()
        .filter_map(|path| match resolver.classify_cli(&cache.sources, &dcx, path) {
            Ok(file) => file,
            Err(diag) => {
                let _ = diag.finish();
                None
            }
        })
        .collect();

    {
        // debug info goes to stdout, diagnostics to the dcx's emitter (stderr)
        let debugger = Debugger::new(std::io::stdout(), &cache.sources)
            .tokens(flags.dlex)
            .trees(flags.dparse);
        let mut frontend = Frontend::new(&cache.sources, &mut cache.state.interner, &dcx, debugger);

        let mut programs = Vec::new();
        for file in files {
            match file {
                File::Program(id) => {
                    if let Ok(ast) = frontend.program(id) {
                        programs.push((id, ast));
                    }
                }
                // nothing uses an interface named on the command line, it only gets checked
                File::Interface(id) => {
                    let _ = frontend.interface(id);
                }
            }
        }

        // TODO: lower `programs` to the HIR here. Lowering takes `&mut frontend` and `&mut resolver`, so an
        // interface it finds through a `use` is lexed and parsed by the same frontend, hooks included.
    }

    let result = match dcx.has_errors() {
        true => Err((&dcx).into()),
        false => Ok((&dcx).into()),
    };

    dcx.emit_diagnostics(&cache.sources);
    result
}
