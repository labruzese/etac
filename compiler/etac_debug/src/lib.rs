use std::io;
use std::sync::{Arc, Mutex, PoisonError};

use etac_errors::emitter::EmitConfig;

/// A writer that collects what is written to it in a shared [`StringBuffer`].
///
/// Clones write to the same buffer, so a test can hand one to an emitter and keep another to read the output back.
#[derive(Clone, Default)]
pub struct StringWriter(pub StringBuffer);

impl StringWriter {
    /// Take everything written so far, leaving the buffer empty.
    pub fn take(&self) -> String {
        std::mem::take(&mut *self.0.lock().unwrap_or_else(PoisonError::into_inner))
    }
}

impl io::Write for StringWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let s = std::str::from_utf8(buf).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        self.0.lock().unwrap_or_else(PoisonError::into_inner).push_str(s);
        Ok(buf.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

pub type StringBuffer = Arc<Mutex<String>>;
pub const EMIT_CONF_DEBUG: EmitConfig = EmitConfig::new().with_color(false).with_compact(false);
