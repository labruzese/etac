use crate::visit::{Visitor, VisitorMut, VisitorOwned};

pub trait Walkable<'ast, V: Visitor<'ast>> {
    fn walk(&'ast self, v: &mut V) -> V::Result;
}

pub trait WalkableMut<V: VisitorMut> {
    fn walk_mut(&mut self, v: &mut V) -> V::Result;
}

pub trait WalkableOwned<V: VisitorOwned> {
    fn walk_owned(self, v: &mut V) -> V::Result;
}
