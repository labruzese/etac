use etac_errors::{dcx::{DiagBuilder, DiagCtx}, emitter::EmitConfig};
use etac_intern::EtacInterner;
use etac_lexer::EtaLexer;
use crate::{Parse, result::{Parsed, ParseResult}};
use etac_source::{FileId, SourceMap};

pub fn parse<'sm, T>(sm: &'sm SourceMap, interner: &mut EtacInterner, file_id: FileId<'sm>, mut dcx: DiagCtx) -> (String, DiagCtx)
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
    etac_debug::write_parse(&out, status, tree, file_id, sm, interner);

    (out.take(), dcx)
}

pub fn parse_program<'sm>(sm: &'sm SourceMap, interner: &mut EtacInterner, file_id: FileId<'sm>, dcx: DiagCtx) -> (String, DiagCtx) {
    parse::<etac_ast::ast::Program>(sm, interner, file_id, dcx)
}

pub fn parse_interface<'sm>(sm: &'sm SourceMap, interner: &mut EtacInterner, file_id: FileId<'sm>, dcx: DiagCtx) -> (String, DiagCtx) {
    parse::<etac_ast::ast::Interface>(sm, interner, file_id, dcx)
}
