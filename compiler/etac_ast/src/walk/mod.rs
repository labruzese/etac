//! Recursing into a node's children.
//!
//! [`Walkable::walk`] visits each of a node's children, without calling the hook for the node itself. It is what the
//! default [`Visitor`](crate::visit::Visitor) hooks do, and what your own hooks call to keep descending into the tree.
//! [`WalkableMut`] and [`WalkableOwned`] do the same for the other two visitor flavors.
//!
//! See the [`visit`](mod@crate::visit) module docs for how this fits together with visitors, and for examples.
//!
//! # Traversal order
//!
//! All three flavors visit a node's children in the same order: the order its fields are declared in
//! [`ast`](crate::ast), with two exceptions.
//!
//! - [`Decl`] and [`GlobDecl`] visit their type before their name (`typ`, then `id`).
//! - [`AssignStmt`] visits its values before its targets.
//! - [`ArrType`] visits its size before its element type, so the sizes of `int[n][m]` are visited as they are written:
//!   `n`, then `m`.
//!
//! So a pass that tracks scopes sees the expressions a declaration depends on before the name it declares: in
//! `x: int = x + 1`, the `x` in `x + 1` is visited before the declaration of `x`, and in `a: int[n]`, `n` is visited
//! before `a`.
//!
//! The interface of a lowered [`hir::Use`](crate::hir::Use) is shared between every `use` of the same file, so only
//! [`Walkable`] descends into it. [`WalkableMut`] and [`WalkableOwned`] treat that `use` as a leaf.
//!
//! A child in an `Option` is skipped when the option is `None`, and the elements of a `Vec` are visited front to
//! back. The leaf nodes ([`Ident`], [`IntLit`], [`BoolLit`], [`CharLit`], [`StringLit`], [`UOpKind`], [`BinOpKind`]
//! and [`Error`]) have no children, so walking them does nothing. Spans are never visited.
//!
//! [`Decl`]: crate::ast::Decl
//! [`GlobDecl`]: crate::ast::GlobDecl
//! [`AssignStmt`]: crate::ast::AssignStmt
//! [`ArrType`]: crate::ast::ArrType
//! [`Ident`]: crate::ast::Ident
//! [`IntLit`]: crate::ast::IntLit
//! [`BoolLit`]: crate::ast::BoolLit
//! [`CharLit`]: crate::ast::CharLit
//! [`StringLit`]: crate::ast::StringLit
//! [`UOpKind`]: crate::ast::UOpKind
//! [`BinOpKind`]: crate::ast::BinOpKind
//! [`Error`]: crate::ast::Error
mod walk_impl;
mod walk_trait;

pub use walk_trait::*;
