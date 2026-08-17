use std::cell::RefCell;
use std::fmt::Display;
use std::fmt::Write as _;
use std::io;
use std::rc::Rc;

use etac_ast::display::sexpr::SexpPrinter;
use etac_ast::display::span::SpanPrinter;
use etac_errors::emitter::EmitConfig;
use etac_errors::emitter::IoEmitter;
use etac_intern::EtacInterner;
use etac_source::{FileId, SourceMap};

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

pub fn string_emitter(config: EmitConfig) -> (Rc<RefCell<String>>, IoEmitter<StringWriter>) {
    let out = Rc::new(RefCell::new(String::new()));
    let emitter = IoEmitter::with_config(StringWriter(out.clone()), config);
    (out, emitter)
}
pub fn sink_emitter(config: EmitConfig) -> IoEmitter<std::io::Sink> {
    IoEmitter::with_config(std::io::sink(), config)
}

/// render a parse for an `expect_test` snapshot: the status line, the s-expression tree,
/// then every node's span against the source.
///
/// `status` is the parse outcome ("clean", "recovered", ...) and `out` is the tree, if
/// there is one. Kept generic over the node type so this crate does not have to depend on
/// `etac_parse`, `etac_parse` dev-depends on it.
pub fn write_parse<'sm, T>(
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
