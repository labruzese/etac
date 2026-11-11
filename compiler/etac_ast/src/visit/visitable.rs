use super::{Visitor, VisitorMut, VisitorOwned, VisitorResult};
use crate::try_visit;

/// Dispatch for [`Visitor`]: `node.visit(v)` calls the hook for the type of `node`.
///
/// For an AST node of type `Foo`, `node.visit(v)` is `v.visit_foo(node)`. It is also implemented for containers of
/// nodes, so it works on any field of a node without unwrapping it first:
///
/// - `Box<T>` visits the `T`.
/// - `Option<T>` visits the `T` if there is one, and does nothing for `None`.
/// - `Vec<T>` and `[T]` visit each element in order, stopping at the first one that returns `Break`.
///
/// Inside a hook, call `.visit(self)` on the node's *children*. Calling it on the node the hook was given calls the
/// same hook again and recurses forever; use [`Walkable::walk`](crate::walk::Walkable::walk) to visit the node's
/// children instead.
pub trait Visitable<'ast, V: Visitor<'ast>> {
    /// Call `v`'s hook for this node, or for each node in this container.
    fn visit(&'ast self, v: &mut V) -> V::Result;
}

/// Dispatch for [`VisitorMut`]: `node.visit_mut(v)` calls the `_mut` hook for the type of `node`.
///
/// Works the same way as [`Visitable`], including on `Box`, `Option`, `Vec` and slices of nodes.
pub trait VisitableMut<V: VisitorMut> {
    /// Call `v`'s hook for this node, or for each node in this container.
    fn visit_mut(&mut self, v: &mut V) -> V::Result;
}

/// Dispatch for [`VisitorOwned`]: `node.visit_owned(v)` calls the `_owned` hook for the type of `node`, passing it
/// ownership of the node.
///
/// Works the same way as [`Visitable`], on `Box`, `Option` and `Vec` of nodes. It is not implemented for slices,
/// since their elements can't be moved out.
pub trait VisitableOwned<V: VisitorOwned> {
    /// Call `v`'s hook for this node, or for each node in this container, consuming it.
    fn visit_owned(self, v: &mut V) -> V::Result;
}

// Box
impl<'ast, V: Visitor<'ast>, T: Visitable<'ast, V> + ?Sized> Visitable<'ast, V> for Box<T> {
    #[inline]
    fn visit(&'ast self, v: &mut V) -> V::Result {
        (**self).visit(v)
    }
}
impl<V: VisitorMut, T: VisitableMut<V> + ?Sized> VisitableMut<V> for Box<T> {
    #[inline]
    fn visit_mut(&mut self, v: &mut V) -> V::Result {
        (**self).visit_mut(v)
    }
}
impl<V: VisitorOwned, T: VisitableOwned<V>> VisitableOwned<V> for Box<T> {
    #[inline]
    fn visit_owned(self, v: &mut V) -> V::Result {
        (*self).visit_owned(v)
    }
}

// Option
impl<'ast, V: Visitor<'ast>, T: Visitable<'ast, V>> Visitable<'ast, V> for Option<T> {
    fn visit(&'ast self, v: &mut V) -> V::Result {
        if let Some(x) = self {
            try_visit!(x.visit(v));
        }
        V::Result::output()
    }
}
impl<V: VisitorMut, T: VisitableMut<V>> VisitableMut<V> for Option<T> {
    fn visit_mut(&mut self, v: &mut V) -> V::Result {
        if let Some(x) = self {
            try_visit!(x.visit_mut(v));
        }
        V::Result::output()
    }
}
impl<V: VisitorOwned, T: VisitableOwned<V>> VisitableOwned<V> for Option<T> {
    fn visit_owned(self, v: &mut V) -> V::Result {
        if let Some(x) = self {
            try_visit!(x.visit_owned(v));
        }
        V::Result::output()
    }
}

// Slices (no owned impl: you can't move out of a `[T]`)
impl<'ast, V: Visitor<'ast>, T: Visitable<'ast, V>> Visitable<'ast, V> for [T] {
    fn visit(&'ast self, v: &mut V) -> V::Result {
        for x in self {
            try_visit!(x.visit(v));
        }
        V::Result::output()
    }
}
impl<V: VisitorMut, T: VisitableMut<V>> VisitableMut<V> for [T] {
    fn visit_mut(&mut self, v: &mut V) -> V::Result {
        for x in self {
            try_visit!(x.visit_mut(v));
        }
        V::Result::output()
    }
}

// Vec
impl<'ast, V: Visitor<'ast>, T: Visitable<'ast, V>> Visitable<'ast, V> for Vec<T> {
    #[inline]
    fn visit(&'ast self, v: &mut V) -> V::Result {
        self.as_slice().visit(v)
    }
}
impl<V: VisitorMut, T: VisitableMut<V>> VisitableMut<V> for Vec<T> {
    #[inline]
    fn visit_mut(&mut self, v: &mut V) -> V::Result {
        self.as_mut_slice().visit_mut(v)
    }
}
impl<V: VisitorOwned, T: VisitableOwned<V>> VisitableOwned<V> for Vec<T> {
    fn visit_owned(self, v: &mut V) -> V::Result {
        for x in self {
            try_visit!(x.visit_owned(v));
        }
        V::Result::output()
    }
}
