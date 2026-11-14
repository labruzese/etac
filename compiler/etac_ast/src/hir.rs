pub use crate::ast::{
    ArrType, ArrayLit, ArrayLitKind, AssignStmt, BinOp, BinOpKind, BinaryExpr, Block, BoolLit, CharLit, Decl,
    DefinitionKind, Error, ExprIndex, ExprKind, GlobDecl, GlobDeclValue, Ident, IfStmt, IntLit, Interface,
    InterfaceItemKind, LValueKind, LengthCall, LitKind, Method, MethodBodyKind, MethodDecl, MultiDeclStmt, ProcCall,
    ReturnStmt, StmtKind, StringLit, TargetKind, TypeKind, UOp, UOpKind, UnaryExpr, WhileStmt,
};

use std::sync::Arc;

use etac_source::Span;

use crate::ptr::PVec;

pub struct Program {
    pub span: Span,
    pub uses: PVec<Use>,
    pub definitions: PVec<DefinitionKind>,
}

pub struct Use {
    pub span: Span,
    pub interface: Arc<Interface>,
}
