//! The driver for the compiler
//!
//! Creates the compilation's [`EtaCache`] and diagnostic context, then passes
//! input between each phase, storing every phase's output back into the cache
//! and attaching loggers along the way.

use std::convert::identity;

use etac_cache::EtaCache;
use etac_errors::{
    dcx::{Diag, DiagCtx},
    guarentee::ErrorGuaranteed,
};
use etac_intern::{EtacInterner};
use etac_parse::{
    result::{ParseResult, Parsed},
    IParser,
};
use etac_resolve::{File, Resolver};
use etac_session::cli::Flags;
use etac_source::{FileId, SourceMap};

pub use crate::status::{CompilationFailure, CompilationSuccess};

mod status;

type Result<T> = std::result::Result<T, ErrorGuaranteed>;
type CompilationResult = std::result::Result<CompilationSuccess, CompilationFailure>;

struct ParseOneSuccess<'src, Out> {
    _out: Out,
    _interner: &'src mut EtacInterner,
}

fn parse_one<'src, 'dcx, P>(
    sm: &'src SourceMap,
    interner: &'src mut EtacInterner,
    dcx: &'dcx DiagCtx,
    file: FileId<'src>,
    parser: P,
) -> Result<ParseOneSuccess<'src, P::Out>>
where
    P: IParser<'src, 'dcx>,
{
    let source = sm.get_source(file);
    let offset = sm.base_offset(file);
    let lexer = etac_lexer::EtaLexer::new(offset, &source.source, interner, dcx);

    let ParseResult {
        parse,
        mut diags,
        symbols,
    } = parser.parse(lexer);

    match parse {
        Parsed::Ok(tree) => Ok(ParseOneSuccess {
            _out: tree,
            _interner: symbols,
        }),
        Parsed::Recovered(tree) => {
            let _g = diags
                .drain(..)
                .map(Diag::finish)
                .reduce(|_, g| identity(g))
                .expect("recovered tree must have at least one diagnostic");
            Ok(ParseOneSuccess {
                _out: tree,
                _interner: symbols,
            })
        }
        Parsed::Failed => {
            let g = diags
                .drain(..)
                .map(Diag::finish)
                .reduce(|_, g| identity(g))
                .expect("failed tree must have at least one diagnostic");
            Err(g)
        }
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
                if let Ok(ParseOneSuccess{ _out: _, _interner: _ }) = parse_one(&cache.sources, &mut cache.state.interner, &dcx, id, etac_parse::ProgramParser::new(&dcx)) {
                    
                }
            }
            File::Interface(id) => {
                if let Ok(ParseOneSuccess{ _out: _, _interner: _ }) = parse_one(&cache.sources, &mut cache.state.interner, &dcx, id, etac_parse::InterfaceParser::new(&dcx)) {
                    
                }
            }
        }
    }

    match dcx.has_errors() {
        true => Err((&dcx).into()),
        false => Ok((&dcx).into()),
    }
}
