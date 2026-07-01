#[derive(Debug)]
pub struct DropBomb(bool);
impl DropBomb {
    pub fn new() -> Self {
        Self(true)
    }
    pub fn defuse(&mut self) {
        self.0 = false
    }
}
impl Drop for DropBomb {
    fn drop(&mut self) {
        if self.0 {
            // A diagnostic was built and then dropped on the floor. In debug that is a
            // bug worth surfacing loudly; in release we still emit it so the user is
            // never silently denied an error they should have seen.
            debug_assert!(false, "Diag dropped without `.emit()`/`.cancel()`: {self:?}");
        }
    }
}
