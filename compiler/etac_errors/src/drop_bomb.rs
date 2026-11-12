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

#[cfg(test)]
mod tests {
    use super::DropBomb;

    #[test]
    #[should_panic(expected = "BOOM: forgotten")]
    fn goes_off_when_forgotten() {
        let _bomb = DropBomb::new("forgotten");
    }

    #[test]
    fn stays_quiet_when_defused() {
        let mut bomb = DropBomb::new("forgotten");
        bomb.defuse();
    }

    /// A panic with a live bomb in scope must unwind as itself. If the bomb went off too, the double panic would
    /// abort the test process instead of being caught here.
    #[test]
    fn does_not_go_off_during_another_panic() {
        let unwound = std::panic::catch_unwind(|| {
            let _bomb = DropBomb::new("forgotten");
            panic!("the real problem");
        });
        let message = unwound.unwrap_err();
        assert_eq!(message.downcast_ref::<&str>(), Some(&"the real problem"));
    }
}
