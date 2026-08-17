use std::cell::RefCell;
use std::io;
use std::rc::Rc;

use etac_errors::emitter::EmitConfig;
use etac_errors::emitter::IoEmitter;

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

// #[inline]
// pub fn write_token(write_buf: &RefCell<String>, token: Token, span: Span, sm: &SourceMap, lookup: &EtacInterner) {
//     let (start, tok, end) = token;
//     let span_location = sm.span_location(Span::new(start, end));
//     let display_token = etac_lexer::token::TokenDisplay::new(tok, lookup);
//     let _ = writeln!(write_buf.borrow_mut(), "{span_location} {display_token}");
// }

// pub fn write_parse<'sm, T>(write_buf: &RefCell<String>, res: Parsed<T>, file_id: FileId<'sm>, sm: &'sm SourceMap, lookup: &EtacInterner) 
// where 
//     for<'ast, 'sym> SexpPrinter<'ast, 'sym, EtacInterner, T>: std::fmt::Display, 
//     for<'ast, 'sym> SpanPrinter<'ast, 'sym, T>: std::fmt::Display {
//     match res {
//         Parsed::Ok(out) => {
//             let name = sm.get_source(file_id).name.clone();
//             let _ = writeln!(write_buf.borrow_mut(), "{name}: parse(clean)\n{}", SexpPrinter::new(lookup, &out));
//             let _ = writeln!(write_buf.borrow_mut(), "{}", SpanPrinter::new(&out, sm));
//         }
//         Parsed::Recovered(out) => {
//             let name = sm.get_source(file_id).name.clone();
//             let _ = writeln!(write_buf.borrow_mut(), "{name}: parse(recovered)\n{}", SexpPrinter::new(lookup, &out));
//             let _ = writeln!(write_buf.borrow_mut(), "{}", SpanPrinter::new(&out, sm));
//         }
//         Parsed::Failed => {
//             let name = sm.get_source(file_id).name.clone();
//             let _ = writeln!(write_buf.borrow_mut(), "{name}: parse(failed)");
//         }
//     }
// }
