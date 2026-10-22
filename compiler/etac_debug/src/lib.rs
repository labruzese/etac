use std::cell::RefCell;
use std::io;
use std::rc::Rc;
use std::sync::LazyLock;

use etac_errors::emitter::EmitConfig;

#[derive(Clone)]
pub struct StringWriter(pub StringBuffer);

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

pub type StringBuffer = Rc<RefCell<String>>;
pub const EMIT_CONF_DEBUG: LazyLock<EmitConfig> = LazyLock::new(|| EmitConfig::default().with_color(false).with_compact(false));
