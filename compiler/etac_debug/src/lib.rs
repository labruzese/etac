use std::cell::RefCell;
use std::io;
use std::rc::Rc;

use etac_errors::emitter::EmitConfig;
use etac_errors::emitter::IoEmitter;

pub struct StringWriter(pub OutputBuffer);

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

pub type OutputBuffer = Rc<RefCell<String>>;

pub fn string_emitter(config: EmitConfig) -> (OutputBuffer, IoEmitter<StringWriter>) {
    let out = Rc::new(RefCell::new(String::new()));
    let emitter = IoEmitter::with_config(StringWriter(out.clone()), config);
    (out, emitter)
}
pub fn sink_emitter(config: EmitConfig) -> IoEmitter<std::io::Sink> {
    IoEmitter::with_config(std::io::sink(), config)
}
