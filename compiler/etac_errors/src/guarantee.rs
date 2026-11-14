/// Proof that an error was recorded in the diagnostic context.
///
/// The only way to get one is to [`finish`](crate::dcx::DiagBuilder::finish) an error. Finishing a warning does not
/// give one: a compilation that only warned has not failed.
#[must_use = "consider returning this guarantee as proof that the error has been reported"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ErrorGuaranteed(());

impl ErrorGuaranteed {
    /// Private so it can only originate on a real error path in this module.
    #[inline]
    pub(crate) fn new() -> Self {
        ErrorGuaranteed(())
    }
    /// Assert that an error was already reported elsewhere, without emitting one here.
    ///
    /// # Safety
    /// The compiler can't prove that an error was actually reported when constructed this way.
    /// You're responsible for making sure that actually emit the error to the user.
    #[inline]
    pub unsafe fn claim_already_emitted() -> Self {
        ErrorGuaranteed(())
    }
}

/// What finishing a diagnostic gives back: an [`ErrorGuaranteed`] for an error, and `()` for anything less.
///
/// It is the type parameter of a [`DiagBuilder`](crate::dcx::DiagBuilder), fixed by how the diagnostic was started,
/// so the type of the builder says whether what it reports is an error.
pub trait EmissionGuarantee: Sized {
    /// Only the diagnostic context can name a [`Recorded`], so only it can call this.
    #[doc(hidden)]
    fn emitted(recorded: Recorded) -> Self;
}

/// Says a diagnostic has just been recorded. It cannot be built outside this crate.
pub struct Recorded(pub(crate) ());

impl EmissionGuarantee for ErrorGuaranteed {
    fn emitted(_: Recorded) -> Self {
        ErrorGuaranteed::new()
    }
}

impl EmissionGuarantee for () {
    fn emitted(_: Recorded) -> Self {}
}
