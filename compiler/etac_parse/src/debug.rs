use std::fmt::Write;
use std::{cell::RefCell, fmt::Display};

use etac_ast::display::{sexpr::SexpPrinter, span::SpanPrinter};
use etac_debug::OutputBuffer;
use etac_errors::{dcx::{DiagBuilder, DiagCtx}, emitter::EmitConfig};
use etac_intern::EtacInterner;
use etac_lexer::EtaLexer;
use crate::{Parse, result::{Parsed, ParseResult}};
use etac_source::{FileId, SourceMap};

/// render a parse for an `expect_test` snapshot: the status line, the s-expression tree,
/// then every node's span against the source.
///
/// `status` is the parse outcome ("clean", "recovered", ...) and `out` is the tree, if
/// there is one. Kept generic over the node type so this crate does not have to depend on
/// `etac_parse`, `etac_parse` dev-depends on it.
fn write_parse<'sm, T>(
    write_buf: &RefCell<String>,
    status: &str,
    out: Option<&T>,
    file_id: FileId<'sm>,
    sm: &'sm SourceMap,
    lookup: &EtacInterner,
) where
    for<'ast, 'sym> SexpPrinter<'ast, 'sym, EtacInterner, T>: Display,
    for<'ast, 'sm2> SpanPrinter<'ast, 'sm2, T>: Display,
{
    let name = sm.get_source(file_id).name.clone();
    let mut buf = write_buf.borrow_mut();
    match out {
        Some(tree) => {
            let _ = writeln!(buf, "{name}: parse({status})\n{}", SexpPrinter::new(lookup, tree));
            let _ = writeln!(buf, "{}", SpanPrinter::new(tree, sm));
        }
        None => {
            let _ = writeln!(buf, "{name}: parse({status})");
        }
    }
}

pub fn parse<'sm, T>(sm: &'sm SourceMap, interner: &mut EtacInterner, file_id: FileId<'sm>, mut dcx: DiagCtx) -> (OutputBuffer, DiagCtx)
where
    T: Parse,
    for<'a, 'b> etac_ast::display::sexpr::SexpPrinter<'a, 'b, EtacInterner, T>: std::fmt::Display,
    for<'a, 'b> etac_ast::display::span::SpanPrinter<'a, 'b, T>: std::fmt::Display,
{
    let (out, emitter) = etac_debug::string_emitter(EmitConfig::default().with_color(false).with_compact(false));
    dcx.change_emitter(Box::new(emitter));
    // Lex first, then parse: the parser walks a finished token array.
    let lexed = EtaLexer::new(sm.base_offset(file_id), interner, &dcx).lex(&sm.get_source(file_id).source);
    for d in Vec::from(lexed.diags) {
        let _ = d.finish();
    }

    let ParseResult { parse, mut diags } = T::parse(&lexed.tokens, &dcx);
    diags.drain(..).map(DiagBuilder::finish).reduce(|_, g| g);

    let (status, tree) = match &parse {
        Parsed::Ok(t) => ("clean", Some(t)),
        Parsed::Recovered(t) => ("recovered", Some(t)),
        Parsed::Failed => ("failed", None),
    };
    write_parse(&out, status, tree, file_id, sm, interner);

    (out, dcx)
}

pub fn parse_program<'sm>(sm: &'sm SourceMap, interner: &mut EtacInterner, file_id: FileId<'sm>, dcx: DiagCtx) -> (OutputBuffer, DiagCtx) {
    parse::<etac_ast::ast::Program>(sm, interner, file_id, dcx)
}

pub fn parse_interface<'sm>(sm: &'sm SourceMap, interner: &mut EtacInterner, file_id: FileId<'sm>, dcx: DiagCtx) -> (OutputBuffer, DiagCtx) {
    parse::<etac_ast::ast::Interface>(sm, interner, file_id, dcx)
}
