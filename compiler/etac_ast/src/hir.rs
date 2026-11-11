pub use crate::ast::{
    ArrType, ArrayLit, ArrayLitKind, AssignStmt, BinOp, BinOpKind, BinaryExpr, Block, BoolLit, CharLit, Decl,
    DefinitionKind, Error, ExprIndex, ExprKind, GlobDecl, GlobDeclValue, Ident, IfStmt, IntLit, Interface,
    InterfaceItemKind, LValueKind, LengthCall, LitKind, Method, MethodBodyKind, MethodDecl, MultiDeclStmt, ProcCall,
    ReturnStmt, StmtKind, StringLit, TargetKind, TypeKind, UOp, UOpKind, UnaryExpr, WhileStmt,
};

use std::sync::Arc;

use etac_source::Span;

pub struct Program {
    pub span: Span,
    pub uses: Vec<Use>,
    pub definitions: Vec<DefinitionKind>,
}

/// A `use` whose interface has been found and parsed.
///
/// The interface is shared: every `use` of the same file, in any program, points at the one parse of it.
pub struct Use {
    pub span: Span,
    pub interface: Arc<Interface>,
}
