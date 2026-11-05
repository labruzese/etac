use super::control_flow::*;
use super::visitable::*;
use crate::walk::{Walkable, WalkableMut, WalkableOwned};
use paste::paste;

macro_rules! ast_visitor {
    ($($ty:ty => $m:ident),* $(,)?) => {
        paste! {
            pub trait Visitor<'ast>: Sized {
                type Result: VisitorResult;
                $(
                    fn $m(&mut self, node: &'ast $ty) -> Self::Result {
                        Walkable::walk(node, self)
                    }
                )*
            }
            pub trait VisitorMut: Sized {
                type Result: VisitorResult;
                $(
                    fn [<$m _mut>](&mut self, node: &mut $ty) -> Self::Result {
                        WalkableMut::walk_mut(node, self)
                    }
                )*
            }
            pub trait VisitorOwned: Sized {
                type Result: VisitorResult;
                $(
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


use crate::ast::*;
ast_visitor! {
    Program           => visit_program,
    Interface         => visit_interface,
    Use               => visit_use,
    DefinitionKind    => visit_definition_kind,
    InterfaceItemKind => visit_interface_item_kind,
    MethodDecl        => visit_method_decl,
    Method            => visit_method,
    MethodBodyKind    => visit_method_body_kind,
    GlobDecl          => visit_glob_decl,
    StmtKind          => visit_stmt_kind,
    AssignStmt        => visit_assign_stmt,
    MultiDeclStmt     => visit_multi_decl_stmt,
    Decl              => visit_decl,
    IfStmt            => visit_if_stmt,
    WhileStmt         => visit_while_stmt,
    ReturnStmt        => visit_return_stmt,
    ProcCall          => visit_proc_call,
    Block             => visit_block,
    TypeKind          => visit_type_kind,
    ArrType           => visit_arr_type,
    TargetKind        => visit_target_kind,
    LValueKind        => visit_lvalue_kind,
    ExprKind          => visit_expr_kind,
    Ident             => visit_ident,
    GlobDeclValue     => visit_glob_decl_value,
    LitKind           => visit_lit_kind,
    IntLit            => visit_int_lit,
    BoolLit           => visit_bool_lit,
    CharLit           => visit_char_lit,
    ArrayLitKind      => visit_array_lit_kind,
    StringLit         => visit_string_lit,
    ArrayLit          => visit_array_lit,
    ExprIndex         => visit_expr_index,
    LengthCall        => visit_length_call,
    UnaryExpr         => visit_unary_expr,
    UOp               => visit_uop,
    UOpKind           => visit_uop_kind,
    BinaryExpr        => visit_binary_expr,
    BinOp             => visit_bin_op,
    BinOpKind         => visit_bin_op_kind,
    Error             => visit_error,
}

