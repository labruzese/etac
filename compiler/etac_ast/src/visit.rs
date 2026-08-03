use std::{convert::Infallible, ops::ControlFlow};
use crate::walk::Walkable;

pub trait VisitorResult {
    type Residual;
    fn output() -> Self;
    fn from_residual(r: Self::Residual) -> Self;
    fn branch(self) -> ControlFlow<Self::Residual>;
}

impl VisitorResult for () {
    type Residual = Infallible;
    fn output() -> Self {}
    fn from_residual(r: Infallible) -> Self { match r {} }
    fn branch(self) -> ControlFlow<Infallible> { ControlFlow::Continue(()) }
}

impl<T> VisitorResult for ControlFlow<T> {
    type Residual = T;
    fn output() -> Self { ControlFlow::Continue(()) }
    fn from_residual(r: T) -> Self { ControlFlow::Break(r) }
    fn branch(self) -> ControlFlow<T> { self }
}

pub trait Visitable<'ast, V: Visitor<'ast>> {
    fn visit(&'ast self, v: &mut V) -> V::Result;
}

#[macro_export]
macro_rules! try_visit {
    ($e:expr) => {
        match $crate::visit::VisitorResult::branch($e) {
            ::core::ops::ControlFlow::Continue(()) => {}
#[allow(unreachable_code)]
            ::core::ops::ControlFlow::Break(r) => {
                return $crate::visit::VisitorResult::from_residual(r);
            }
        }
    };
}

#[macro_export]
macro_rules! visit {
    ($v:expr, $($node:expr),* $(,)?) => {{
        $( $crate::try_visit!($crate::visit::Visitable::visit($node, $v)); )*
    }};
}

macro_rules! ast_visitor {
    ($($ty:ty  => $m:ident),* $(,)?) => {
        pub trait Visitor<'ast>: Sized {
            type Result: VisitorResult;
            $(
                fn $m(&mut self, node: &'ast $ty) -> Self::Result {
                    Walkable::walk(node, self)
                }
            )*
        }
        $(
            impl<'ast, V: Visitor<'ast>> Visitable<'ast, V> for $ty {
                #[inline]
                fn visit(&'ast self, v: &mut V) -> V::Result { v.$m(self) }
            }
        )*
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

impl<'ast, V: Visitor<'ast>, T: Visitable<'ast, V> + ?Sized> Visitable<'ast, V> for Box<T> {
    #[inline]
    fn visit(&'ast self, v: &mut V) -> V::Result { (**self).visit(v) }
}
impl<'ast, V: Visitor<'ast>, T: Visitable<'ast, V>> Visitable<'ast, V> for Option<T> {
    fn visit(&'ast self, v: &mut V) -> V::Result {
        if let Some(x) = self { try_visit!(x.visit(v)); }
        V::Result::output()
    }
}
impl<'ast, V: Visitor<'ast>, T: Visitable<'ast, V>> Visitable<'ast, V> for [T] {
    fn visit(&'ast self, v: &mut V) -> V::Result {
        for x in self { try_visit!(x.visit(v)); }
        V::Result::output()
    }
}
impl<'ast, V: Visitor<'ast>, T: Visitable<'ast, V>> Visitable<'ast, V> for Vec<T> {
    #[inline]
    fn visit(&'ast self, v: &mut V) -> V::Result { self.as_slice().visit(v) }
}
