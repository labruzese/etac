//! The driver for the compiler

use etac_ast::ast::{Interface, Program};
use etac_cache::EtaCache;
use etac_errors::{
    dcx::{DiagBuilder, DiagCtx},
    guarentee::ErrorGuaranteed,
};
use etac_intern::EtacInterner;
use etac_parse::{
    result::{ParseResult, Parsed},
    Parse,
};
use etac_resolve::{File, Resolver};
use etac_session::cli::Flags;
use etac_source::{FileId, SourceMap};

pub use crate::status::{CompilationFailure, CompilationSuccess};

mod status;

type Result<T> = std::result::Result<T, ErrorGuaranteed>;
type CompilationResult = std::result::Result<CompilationSuccess, CompilationFailure>;

fn parse_one<'src, T>(sm: &'src SourceMap, interner: &mut EtacInterner, dcx: &DiagCtx, file: FileId<'src>) -> Result<T>
where
    T: Parse,
{
    let source = sm.get_source(file);
    let lexed = etac_lexer::EtaLexer::new(sm.base_offset(file), interner, dcx).lex(&source.source);

    let mut lex_guarantee = None;
    for diag in Vec::from(lexed.diags) {
        lex_guarantee = Some(diag.finish());
    }

    let ParseResult { parse, mut diags } = T::parse(&lexed.tokens, dcx);
    let parse_guarantee = diags.drain(..).map(DiagBuilder::finish).reduce(|_, g| g);

    match parse {
        Parsed::Ok(tree) | Parsed::Recovered(tree) => Ok(tree),
        Parsed::Failed => Err(parse_guarantee
            .or(lex_guarantee)
            .expect("a failed parse must have reported at least one diagnostic")),
    }
}

pub fn run(flags: &Flags) -> CompilationResult {
    let mut cache = EtaCache::new();
    let dcx = DiagCtx::new();

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


    for file in files {
        match file {
            File::Program(id) => {
                if flags.dlex {
                    return run_dlex(&cache.sources, &mut cache.state.interner, id, dcx);
                }
                if flags.dparse {
                    return run_dparse::<Program>(&cache.sources, &mut cache.state.interner, id, dcx);
                }
                let _ast: Result<Program> = parse_one(&cache.sources, &mut cache.state.interner, &dcx, id);
            }
            File::Interface(id) => {
                if flags.dlex {
                    return run_dlex(&cache.sources, &mut cache.state.interner, id, dcx);
                }
                if flags.dparse {
                    return run_dparse::<Interface>(&cache.sources, &mut cache.state.interner, id, dcx);
                }
                let _ast: Result<Interface> = parse_one(&cache.sources, &mut cache.state.interner, &dcx, id);
            }
        }
    }

    let result = match dcx.has_errors() {
        true => Err((&dcx).into()),
        false => Ok((&dcx).into()),
    };

    dcx.emit_diagnostics(&cache.sources);
    result
}

pub fn run_dlex(sm: &SourceMap, interner: &mut EtacInterner, fid: FileId, dcx: DiagCtx) -> CompilationResult {
    let (debug_out, dcx) = etac_lexer::debug::lex(sm, interner, fid, dcx);
    let result = match dcx.has_errors() {
        true => Err((&dcx).into()),
        false => Ok((&dcx).into()),
    };
    dcx.emit_diagnostics(sm);
    println!("{}", debug_out);
    result
}

pub fn run_dparse<'sm, T>(sm: &'sm SourceMap, interner: &mut EtacInterner, fid: FileId<'sm>, dcx: DiagCtx) -> CompilationResult 
where
    T: Parse,
    for<'a, 'b> etac_ast::display::sexpr::SexpPrinter<'a, 'b, EtacInterner, T>: std::fmt::Display,
    for<'a, 'b> etac_ast::display::span::SpanPrinter<'a, 'b, T>: std::fmt::Display,
{
    let (debug_out, dcx) = etac_parse::debug::parse(sm, interner, fid, dcx);
    let result = match dcx.has_errors() {
        true => Err((&dcx).into()),
        false => Ok((&dcx).into()),
    };
    dcx.emit_diagnostics(sm);
    println!("{}", debug_out);
    result
}
