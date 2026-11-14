/// Panics when dropped, unless it was [defused](DropBomb::defuse) first.
///
/// Put one in a value that must not be forgotten, such as a diagnostic that has to be reported or cancelled.
#[derive(Debug)]
pub struct DropBomb(bool, &'static str);
impl DropBomb {
    pub fn new(msg: &'static str) -> Self {
        Self(true, msg)
    }
    pub fn defuse(&mut self) {
        self.0 = false
    }
}
impl Drop for DropBomb {
    fn drop(&mut self) {
        // While a panic is unwinding, everything in its way is dropped unfinished, and that is not the bug this is
        // here to catch. Panicking again would abort the process and bury the panic that started it.
        if std::thread::panicking() {
            return;
        }
        assert!(!self.0, "BOOM: {}", self.1);
    }
}
