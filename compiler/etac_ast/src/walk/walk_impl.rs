use super::{Walkable, WalkableMut, WalkableOwned};
use crate::visit::{Visitor, VisitorMut, VisitorOwned, VisitorResult};

// `impl_walk! {Ty, |self, v| { ... }}` implements `Walkable`, `WalkableMut` and `WalkableOwned` for `Ty`, all from
// one body. The body is pasted into each impl unchanged, but:
//
// - `self` is `&Ty`, `&mut Ty` or `Ty`, so the same destructuring pattern binds the fields as `&`, `&mut` or owned
//   values.
// - `visit!` is rebound to `visit!`, `visit_mut!` or `visit_owned!`, so the same `visit!(v, a, b)` call visits the
//   fields with the matching flavor of visitor.
//
// When writing a body:
//
// - Destructure the node exhaustively (`let Ty { span: _, a, b } = self;`, without `..`), so that adding a field to the
//   node is a compile error here until someone decides whether the field should be visited.
// - Visit the children in the order a pass should see them. This is the traversal order documented on the `walk`
//   module, so update those docs and leave an `NB:` comment when it differs from the order of the fields.
// - Don't produce a value: `V::Result::output()` is appended after the body.
macro_rules! impl_walk {
    ($ty:ty, |$s:ident, $v:ident| $body:block) => {
        impl<'ast, V: Visitor<'ast>> Walkable<'ast, V> for $ty {
            fn walk(&'ast $s, $v: &mut V) -> V::Result {
                #[allow(unused_imports)]
                use $crate::visit as visit;
                #[allow(unused_variables)]
                $body
                #[allow(unreachable_code)]
                V::Result::output()
            }
        }
        impl<V: VisitorMut> WalkableMut<V> for $ty {
            fn walk_mut(&mut $s, $v: &mut V) -> V::Result {
                #[allow(unused_imports)]
                use $crate::visit_mut as visit;
                #[allow(unused_variables)]
                $body
                #[allow(unreachable_code)]
                V::Result::output()
            }
        }
        impl<V: VisitorOwned> WalkableOwned<V> for $ty {
            fn walk_owned($s, $v: &mut V) -> V::Result {
                #[allow(unused_imports)]
                use $crate::visit_owned as visit;
                #[allow(unused_variables)]
                $body
                #[allow(unreachable_code)]
                V::Result::output()
            }
        }
    };
}

// `atom! {Ty}` implements the three walk traits for a leaf node, one with no children to visit.
macro_rules! atom {
    ($ty:ty) => {
        impl<'ast, V: Visitor<'ast>> Walkable<'ast, V> for $ty {
            fn walk(&'ast self, _v: &mut V) -> V::Result {
                V::Result::output()
            }
        }
        impl<V: VisitorMut> WalkableMut<V> for $ty {
            fn walk_mut(&mut self, _v: &mut V) -> V::Result {
                V::Result::output()
            }
        }
        impl<V: VisitorOwned> WalkableOwned<V> for $ty {
            fn walk_owned(self, _v: &mut V) -> V::Result {
                V::Result::output()
            }
        }
    };
}

// ast specfic impls

impl_walk! {crate::ast::Program, |self, v| {
    let crate::ast::Program { span: _, uses, definitions } = self;
    visit!(v, uses, definitions)
}}

impl_walk! {crate::ast::Use, |self, v| {
    let crate::ast::Use { span: _, interface } = self;
    visit!(v, interface)
}}

// hir impls

use crate::hir::*;

atom! {Ident}
atom! {IntLit}
atom! {BoolLit}
atom! {CharLit}
atom! {StringLit}
atom! {UOpKind}
atom! {BinOpKind}
atom! {Error}

impl_walk! {crate::hir::Program, |self, v| {
    let Program { span: _, uses, definitions } = self;
    visit!(v, uses, definitions)
}}

impl_walk! {Interface, |self, v| {
    let Interface { span: _, items } = self;
    visit!(v, items)
}}

impl_walk! {crate::hir::Use, |self, v| {
    let Use { span: _, interface } = self;
    visit!(v, interface)
}}

impl_walk! {DefinitionKind, |self, v| {
    match self {
        DefinitionKind::Method(method) => visit!(v, method),
        DefinitionKind::GlobDecl(glob_decl) => visit!(v, glob_decl),
        DefinitionKind::Error(error) => visit!(v, error),
    }
}}

impl_walk! {InterfaceItemKind, |self, v| {
    match self {
        InterfaceItemKind::MethodDecl(method_decl) => visit!(v, method_decl),
        InterfaceItemKind::Error(error) => visit!(v, error),
    }
}}

impl_walk! {MethodDecl, |self, v| {
    let MethodDecl { span: _, id, params, ret_types } = self;
    visit!(v, id, params, ret_types)
}}

impl_walk! {Method, |self, v| {
    let Method { span: _, decl, body } = self;
    visit!(v, decl, body)
}}

impl_walk! {MethodBodyKind, |self, v| {
    match self {
        MethodBodyKind::Block(block) => visit!(v, block),
        MethodBodyKind::Error(error) => visit!(v, error),
    }
}}

impl_walk! {GlobDecl, |self, v| {
    let GlobDecl { span: _, id, typ, val } = self;
    visit!(v, typ, id, val) // NB: type before id
}}

impl_walk! {StmtKind, |self, v| {
    match self {
        StmtKind::Assign(assign_stmt)   => visit!(v, assign_stmt),
        StmtKind::Decls(multi_decl_stmt) => visit!(v, multi_decl_stmt),
        StmtKind::If(if_stmt)           => visit!(v, if_stmt),
        StmtKind::While(while_stmt)     => visit!(v, while_stmt),
        StmtKind::Return(return_stmt)   => visit!(v, return_stmt),
        StmtKind::Call(proc_call)       => visit!(v, proc_call),
        StmtKind::Block(block)          => visit!(v, block),
        StmtKind::Error(error)          => visit!(v, error),
    }
}}

impl_walk! {AssignStmt, |self, v| {
    let AssignStmt { span: _, targets, values } = self;
    visit!(v, values, targets) // NB: values before targets
}}

impl_walk! {MultiDeclStmt, |self, v| {
    let MultiDeclStmt { span: _, decls } = self;
    visit!(v, decls)
}}

impl_walk! {Decl, |self, v| {
    let Decl { span: _, id, typ } = self;
    visit!(v, typ, id) // NB: type before id
}}

impl_walk! {IfStmt, |self, v| {
    let IfStmt { span: _, cond, then_branch, else_branch } = self;
    visit!(v, cond, then_branch, else_branch)
}}

impl_walk! {WhileStmt, |self, v| {
    let WhileStmt { span: _, cond, body } = self;
    visit!(v, cond, body)
}}

impl_walk! {ReturnStmt, |self, v| {
    let ReturnStmt { span: _, values } = self;
    visit!(v, values)
}}

impl_walk! {ProcCall, |self, v| {
    let ProcCall { span: _, id, args } = self;
    visit!(v, id, args)
}}

impl_walk! {Block, |self, v| {
    let Block { span: _, stmts } = self;
    visit!(v, stmts)
}}

impl_walk! {TypeKind, |self, v| {
    match self {
        TypeKind::Array(arr_type) => visit!(v, arr_type),
        TypeKind::Int(_span) | TypeKind::Bool(_span) => {}
        TypeKind::Error(error) => visit!(v, error),
    }
}}

impl_walk! {ArrType, |self, v| {
    let ArrType { span: _, of, size } = self;
    visit!(v, of, size)
}}

impl_walk! {TargetKind, |self, v| {
    match self {
        TargetKind::LValue(lvalue_kind) => visit!(v, lvalue_kind),
        TargetKind::Decl(decl)          => visit!(v, decl),
        TargetKind::Discard(_span)      => {}
    }
}}

impl_walk! {LValueKind, |self, v| {
    match self {
        LValueKind::Id(ident)           => visit!(v, ident),
        LValueKind::ProcCall(proc_call) => visit!(v, proc_call),
        LValueKind::Index(expr_index)   => visit!(v, expr_index),
    }
}}

impl_walk! {ExprKind, |self, v| {
    match self {
        ExprKind::Id(ident)             => visit!(v, ident),
        ExprKind::Lit(lit_kind)         => visit!(v, lit_kind),
        ExprKind::Index(expr_index)     => visit!(v, expr_index),
        ExprKind::Call(proc_call)       => visit!(v, proc_call),
        ExprKind::Length(length_call)   => visit!(v, length_call),
        ExprKind::Unary(unary_expr)     => visit!(v, unary_expr),
        ExprKind::Binary(binary_expr)   => visit!(v, binary_expr),
        ExprKind::Error(error)          => visit!(v, error),
    }
}}

impl_walk! {GlobDeclValue, |self, v| {
    match self {
        GlobDeclValue::Literal(lit_kind) => visit!(v, lit_kind),
        GlobDeclValue::Neg{ span: _, value } => visit!(v, value),
    }
}}

impl_walk! {LitKind, |self, v| {
    match self {
        LitKind::Int(int_lit) => visit!(v, int_lit),
        LitKind::Bool(bool_lit) => visit!(v, bool_lit),
        LitKind::Char(char_lit) => visit!(v, char_lit),
        LitKind::Array(array_lit_kind) => visit!(v, array_lit_kind),
    }
}}

impl_walk! {ArrayLitKind, |self, v| {
    match self {
        ArrayLitKind::String(string_lit) => visit!(v, string_lit),
        ArrayLitKind::Array(array_lit)   => visit!(v, array_lit),
    }
}}

impl_walk! {ArrayLit, |self, v| {
    let ArrayLit { span: _, value } = self;
    visit!(v, value)
}}

impl_walk! {ExprIndex, |self, v| {
    let ExprIndex { span: _, array, index } = self;
    visit!(v, array, index)
}}

impl_walk! {LengthCall, |self, v| {
    let LengthCall { span: _, param } = self;
    visit!(v, param)
}}

impl_walk! {UnaryExpr, |self, v| {
    let UnaryExpr { span: _, op, operand } = self;
    visit!(v, op, operand)
}}

impl_walk! {UOp, |self, v| {
    let UOp { span: _, kind } = self;
    visit!(v, kind)
}}

impl_walk! {BinaryExpr, |self, v| {
    let BinaryExpr { span: _, op, lhs, rhs } = self;
    visit!(v, op, lhs, rhs)
}}

impl_walk! {BinOp, |self, v| {
    let BinOp { span: _, kind } = self;
    visit!(v, kind)
}}
