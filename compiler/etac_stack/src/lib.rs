//! Recursing as deep as the input goes.
//!
//! The compiler is recursive wherever the language is: the parser descends through nested expressions and
//! statements, and everything after it walks the tree the same way, down to dropping it. How deep that goes is up to
//! the program being compiled, and a thread's stack is a fixed size.
//!
//! Rather than cap what a program may nest, every such recursion goes through [`ensure_sufficient_stack`], which
//! continues on a fresh piece of stack from the heap when the current one is about to run out. Depth is then limited
//! by memory, like everything else about a program's size.

/// How much stack must be left for a recursive step to run in place.
///
/// This has to cover everything that runs between one call of [`ensure_sufficient_stack`] and the next, so it is
/// generous: a step that is not itself recursive can be far heavier than the recursion around it.
const RED_ZONE: usize = 128 * 1024;

/// How much stack to add when less than [`RED_ZONE`] is left.
const GROW_BY: usize = 2 * 1024 * 1024;

/// Run `step`, on more stack if there is not enough left to do it safely.
///
/// Wrap the body of anything that recurses as deep as its input: one call per level is enough. The check is a
/// comparison against a thread-local, so it costs next to nothing when there is room, which is nearly always.
#[inline]
pub fn ensure_sufficient_stack<R>(step: impl FnOnce() -> R) -> R {
    stacker::maybe_grow(RED_ZONE, GROW_BY, step)
}
