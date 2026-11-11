use crate::visit::{Visitor, VisitorMut, VisitorOwned};

/// Recursion for [`Visitor`]: `node.walk(v)` visits each of `node`'s children.
///
/// Each child is visited with [`Visitable::visit`](crate::visit::Visitable::visit), in [traversal
/// order](crate::walk#traversal-order), stopping at the first one that returns `Break`. Walking a node does *not*
/// call the hook for the node itself, and walking a leaf (such as an [`Ident`](crate::ast::Ident)) does nothing.
///
/// Every default [`Visitor`] hook just walks its node. Call `node.walk(self)` from your own hooks to keep descending
/// into the tree, or leave it out to skip the node's subtree.
pub trait Walkable<'ast, V: Visitor<'ast>> {
    /// Visit each of this node's children with `v`.
    fn walk(&'ast self, v: &mut V) -> V::Result;
}

/// Recursion for [`VisitorMut`]: `node.walk_mut(v)` visits each of `node`'s children, with mutable access.
///
/// Works the same way as [`Walkable`], in the same order.
pub trait WalkableMut<V: VisitorMut> {
    /// Visit each of this node's children with `v`.
    fn walk_mut(&mut self, v: &mut V) -> V::Result;
}

/// Recursion for [`VisitorOwned`]: `node.walk_owned(v)` consumes `node` and moves each of its children into `v`.
///
/// Works the same way as [`Walkable`], in the same order.
pub trait WalkableOwned<V: VisitorOwned> {
    /// Visit each of this node's children with `v`, consuming the node.
    fn walk_owned(self, v: &mut V) -> V::Result;
}
