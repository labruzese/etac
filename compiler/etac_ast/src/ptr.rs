//! How a node owns its children.
//!
//! A tree is as deep as the program it was parsed from: `1 + 1 + ... + 1` is as deep as it is long, and so is a
//! chain of `else if`. Dropping a tree visits every level of it, and the drop the compiler generates for a `Box` or a
//! `Vec` recurses on the thread's own stack, which a deep enough tree overflows.
//!
//! So nodes hold their children in [`P`] and [`PVec`] instead. They are a `Box` and a `Vec` that drop what they own
//! through [`ensure_sufficient_stack`], and every cycle of node types goes through one or the other, so a tree of
//! any depth can be dropped.
//!
//! Traversals get the same protection from the [visitor dispatch](crate::visit::Visitable).

use std::ops::{Deref, DerefMut};

use etac_stack::ensure_sufficient_stack;

/// An owned child node: a `Box` that is safe to drop however deep the tree under it is.
pub struct P<T>(
    /// Always `Some`, until the value is moved out by [`into_inner`](P::into_inner) or by `drop`.
    Option<Box<T>>,
);

impl<T> P<T> {
    pub fn new(value: T) -> Self {
        P(Some(Box::new(value)))
    }

    /// Move the node out of its allocation.
    pub fn into_inner(mut self) -> T {
        *self.0.take().expect("a `P` holds its node until it is consumed")
    }
}

impl<T> Deref for P<T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.0.as_deref().expect("a `P` holds its node until it is consumed")
    }
}

impl<T> DerefMut for P<T> {
    fn deref_mut(&mut self) -> &mut T {
        self.0
            .as_deref_mut()
            .expect("a `P` holds its node until it is consumed")
    }
}

impl<T> Drop for P<T> {
    fn drop(&mut self) {
        if let Some(node) = self.0.take() {
            ensure_sufficient_stack(|| drop(node));
        }
    }
}

/// A list of owned child nodes: a `Vec` that is safe to drop however deep the trees in it are.
///
/// It derefs to the `Vec`, so it is used like one.
pub struct PVec<T>(Vec<T>);

impl<T> PVec<T> {
    pub const fn new() -> Self {
        PVec(Vec::new())
    }

    /// Take the nodes out as a plain `Vec`.
    pub fn into_vec(mut self) -> Vec<T> {
        std::mem::take(&mut self.0)
    }
}

impl<T> Default for PVec<T> {
    fn default() -> Self {
        PVec::new()
    }
}

impl<T> From<Vec<T>> for PVec<T> {
    fn from(nodes: Vec<T>) -> Self {
        PVec(nodes)
    }
}

impl<T> FromIterator<T> for PVec<T> {
    fn from_iter<I: IntoIterator<Item = T>>(nodes: I) -> Self {
        PVec(Vec::from_iter(nodes))
    }
}

impl<T> Deref for PVec<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Vec<T> {
        &self.0
    }
}

impl<T> DerefMut for PVec<T> {
    fn deref_mut(&mut self) -> &mut Vec<T> {
        &mut self.0
    }
}

impl<T> IntoIterator for PVec<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_vec().into_iter()
    }
}

impl<'a, T> IntoIterator for &'a PVec<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut PVec<T> {
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

impl<T> Drop for PVec<T> {
    fn drop(&mut self) {
        if !self.0.is_empty() {
            let nodes = std::mem::take(&mut self.0);
            ensure_sufficient_stack(|| drop(nodes));
        }
    }
}
