use super::control_flow::*;
use super::visitable::*;
use crate::walk::{Walkable, WalkableMut, WalkableOwned};
use paste::paste;

// Generates the three visitor traits from one list of `NodeType => hook_name` entries, plus the `Visitable*` impls
// that dispatch each node type to its hook. The `VisitorMut` and `VisitorOwned` hooks are named after the `Visitor`
// one, with a `_mut` or `_owned` suffix.
//
// To add a node type, add it to the list at the bottom of this file, and give it a `Walkable*` impl in
// `walk/walk_impl.rs` (the default hook bodies call it).
macro_rules! ast_visitor {
    ($($ty:ty => $m:ident),* $(,)?) => {
        paste! {
            /// A pass over the AST that reads it.
            ///
            /// There is one hook per AST node type. Each hook's default body walks the node's children with
            /// [`Walkable::walk`], so a visitor that overrides nothing visits the whole tree and does nothing.
            /// Override the hooks for the nodes you care about, and call `node.walk(self)` from them to keep
            /// descending into the tree.
            ///
            /// Nodes are borrowed for `'ast`, the lifetime of the tree, so a visitor can keep references to the
            /// nodes it is handed.
            ///
            /// See the [module docs](mod@crate::visit) for a guide and examples.
            pub trait Visitor<'ast>: Sized {
                /// What every hook returns: `()` for a pass that always visits the whole tree, or
                /// [`ControlFlow<B>`](std::ops::ControlFlow) for one that can stop early by returning
                /// `ControlFlow::Break(B)`. See [`VisitorResult`].
                type Result: VisitorResult;
                $(
                    #[doc = concat!(
                        "Called for each [`", stringify!($ty), "`](crate::", stringify!($ty), "). ",
                        "By default, walks its children with [`Walkable::walk`]."
                    )]
                    fn $m(&mut self, node: &'ast $ty) -> Self::Result {
                        Walkable::walk(node, self)
                    }
                )*
            }

            /// A pass over the AST that modifies it in place.
            ///
            /// The `&mut` twin of [`Visitor`]: the same hooks with a `_mut` suffix, each handed mutable access to
            /// its node. A hook can change the node's fields, or replace the whole node with `*node = ...`. Each
            /// hook's default body walks the node's children with [`WalkableMut::walk_mut`]; call
            /// `node.walk_mut(self)` from your own hooks to keep descending into the tree.
            ///
            /// Unlike with [`Visitor`], a node is only borrowed for the duration of its hook, so a `VisitorMut` can't
            /// keep references into the tree.
            ///
            /// See the [module docs](mod@crate::visit#borrowed-mutable-or-owned) for an example.
            pub trait VisitorMut: Sized {
                /// What every hook returns. See [`Visitor::Result`].
                type Result: VisitorResult;
                $(
                    #[doc = concat!(
                        "Called for each [`", stringify!($ty), "`](crate::", stringify!($ty), "), ",
                        "with mutable access to it. By default, walks its children with [`WalkableMut::walk_mut`]."
                    )]
                    fn [<$m _mut>](&mut self, node: &mut $ty) -> Self::Result {
                        WalkableMut::walk_mut(node, self)
                    }
                )*
            }

            /// A pass over the AST that consumes it.
            ///
            /// The by-value twin of [`Visitor`]: the same hooks with an `_owned` suffix, each handed ownership of its
            /// node. Use it for passes that tear the tree down, such as lowering it into another representation, and
            /// want to move data out of it without cloning.
            ///
            /// Each hook's default body walks the node's children with [`WalkableOwned::walk_owned`], which consumes
            /// the node. A hook that takes its node apart can't walk it afterwards, so it has to visit the parts it
            /// wants itself, with [`VisitableOwned::visit_owned`].
            ///
            /// See the [module docs](mod@crate::visit#borrowed-mutable-or-owned) for an example.
            pub trait VisitorOwned: Sized {
                /// What every hook returns. See [`Visitor::Result`].
                type Result: VisitorResult;
                $(
                    #[doc = concat!(
                        "Called for each [`", stringify!($ty), "`](crate::", stringify!($ty), "), ",
                        "taking ownership of it. By default, walks its children with [`WalkableOwned::walk_owned`]."
                    )]
                    fn [<$m _owned>](&mut self, node: $ty) -> Self::Result {
                        WalkableOwned::walk_owned(node, self)
                    }
                )*
            }
            $(
                impl<'ast, V: Visitor<'ast>> Visitable<'ast, V> for $ty {
                    #[inline]
                    fn visit(&'ast self, v: &mut V) -> V::Result { v.$m(self) }
                }
                impl<V: VisitorMut> VisitableMut<V> for $ty {
                    #[inline]
                    fn visit_mut(&mut self, v: &mut V) -> V::Result { v.[<$m _mut>](self) }
                }
                impl<V: VisitorOwned> VisitableOwned<V> for $ty {
                    #[inline]
                    fn visit_owned(self, v: &mut V) -> V::Result { v.[<$m _owned>](self) }
                }
            )*
        }
    }
}

use crate::ast;
use crate::hir;
ast_visitor! {
    ast::Program           => visit_program_ast,
    ast::Use               => visit_use_ast,

    hir::Program           => visit_program_hir,
    hir::Interface         => visit_interface,
    hir::Use               => visit_use_hir,
    hir::DefinitionKind    => visit_definition_kind,
    hir::InterfaceItemKind => visit_interface_item_kind,
    hir::MethodDecl        => visit_method_decl,
    hir::Method            => visit_method,
    hir::MethodBodyKind    => visit_method_body_kind,
    hir::GlobDecl          => visit_glob_decl,
    hir::StmtKind          => visit_stmt_kind,
    hir::AssignStmt        => visit_assign_stmt,
    hir::MultiDeclStmt     => visit_multi_decl_stmt,
    hir::Decl              => visit_decl,
    hir::IfStmt            => visit_if_stmt,
    hir::WhileStmt         => visit_while_stmt,
    hir::ReturnStmt        => visit_return_stmt,
    hir::ProcCall          => visit_proc_call,
    hir::Block             => visit_block,
    hir::TypeKind          => visit_type_kind,
    hir::ArrType           => visit_arr_type,
    hir::TargetKind        => visit_target_kind,
    hir::LValueKind        => visit_lvalue_kind,
    hir::ExprKind          => visit_expr_kind,
    hir::Ident             => visit_ident,
    hir::GlobDeclValue     => visit_glob_decl_value,
    hir::LitKind           => visit_lit_kind,
    hir::IntLit            => visit_int_lit,
    hir::BoolLit           => visit_bool_lit,
    hir::CharLit           => visit_char_lit,
    hir::ArrayLitKind      => visit_array_lit_kind,
    hir::StringLit         => visit_string_lit,
    hir::ArrayLit          => visit_array_lit,
    hir::ExprIndex         => visit_expr_index,
    hir::LengthCall        => visit_length_call,
    hir::UnaryExpr         => visit_unary_expr,
    hir::UOp               => visit_uop,
    hir::UOpKind           => visit_uop_kind,
    hir::BinaryExpr        => visit_binary_expr,
    hir::BinOp             => visit_bin_op,
    hir::BinOpKind         => visit_bin_op_kind,
    hir::Error             => visit_error,
}
