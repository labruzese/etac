use super::{Visitor, VisitorMut, VisitorOwned, VisitorResult};
use crate::try_visit;

pub trait Visitable<'ast, V: Visitor<'ast>> {
    fn visit(&'ast self, v: &mut V) -> V::Result;
}

pub trait VisitableMut<V: VisitorMut> {
    fn visit_mut(&mut self, v: &mut V) -> V::Result;
}

pub trait VisitableOwned<V: VisitorOwned> {
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
