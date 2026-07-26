use std::cell::RefCell;
use std::fmt::Write as _;
use std::io;
use std::rc::Rc;

use etac_errors::emitter::EmitConfig;
use etac_errors::emitter::IoEmitter;
use etac_intern::Interner;
use etac_lexer::token::SourceToken;
use etac_source::SourceMap;
use etac_source::Span;

pub struct StringWriter(pub Rc<RefCell<String>>);
    impl io::Write for StringWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            let s = std::str::from_utf8(buf)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
            self.0.borrow_mut().push_str(s);
            Ok(buf.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

pub fn string_emitter(write_buf: &Rc<RefCell<String>>, config: EmitConfig) -> IoEmitter<StringWriter> {
    IoEmitter::with_config(StringWriter(write_buf.clone()), config)
}

pub fn write_token(write_buf: &RefCell<String>, token: SourceToken, sm: &SourceMap, lookup: &Interner) {
    let (start, tok, end) = token;
    let span_location = sm.span_location(Span::new(start, end));
    let display_token = etac_lexer::token::TokenDisplay::new(tok, |sym| lookup.resolve(*sym));
    let _ = writeln!(write_buf.borrow_mut(), "{span_location} {display_token}");
}

// pub fn write_parse_output<T: etac_ast::sexpr::Sexpr>(buffer: &mut String, file: FileId, output: etac_parse::Parsed<T>, cache: &EtaCache) {
//     let file = cache.source_name(file);
//     match output.output() {
//         Some(out) => {
//             let spanctx = etac_ast::printer::spans::SpanCtx::new(|f| {
//                 let span = cache.span(f);
//                 let (line_start, column_start) = cache.line_column(span.lo);
//                 let (line_end, column_end) = cache.line_column(span.hi);
//                 Some(format!("{line_start}:{column_start}..{line_end}:{column_end}"))
//             });
//             let mut ast_str = String::new();
//             let _ = out.to_doc(&etac_ast::sexpr::Plain).render_fmt(etac_ast::sexpr::WIDTH, &mut ast_str);
//             let _ = writeln!(buffer, "{file}::ast {{\n{ast_str}\n}}");
//         }
//         None => {
//             let _ = writeln!(buffer, "{file}::ast {{Failed}}");
//         }
//     }
// }
