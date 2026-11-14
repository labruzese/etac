use etac_errors::guarantee::ErrorGuaranteed;
use etac_intern::Symbol;
use etac_source::Span;

use crate::ptr::{P, PVec};

/// Stands in for a piece of the tree that could not be built.
///
/// An error node is proof that the problem was reported: it cannot be made without the [`ErrorGuaranteed`] that only
/// reporting an error gives. So a pass that comes across one has nothing to say about it, and can carry on with the
/// rest of the tree knowing the compilation has already failed.
pub struct Error {
    /// What the node stands in for.
    pub span: Span,
    /// Proof that the error behind this node was reported.
    pub reported: ErrorGuaranteed,
}

// Top level

pub struct Program {
    pub span: Span,
    pub uses: PVec<Use>,
    pub definitions: PVec<DefinitionKind>,
}

pub struct Interface {
    pub span: Span,
    pub items: PVec<InterfaceItemKind>,
}

pub struct Use {
    pub span: Span,
    pub interface: Ident,
}

// Definitions

pub enum DefinitionKind {
    Method(Method),
    GlobDecl(GlobDecl),
    Error(Error),
}

pub enum InterfaceItemKind {
    MethodDecl(MethodDecl),
    Error(Error),
}

pub struct MethodDecl {
    pub span: Span,
    pub id: Ident,
    pub params: PVec<Decl>,
    pub ret_types: PVec<TypeKind>,
}

pub struct Method {
    pub span: Span,
    pub decl: MethodDecl,
    pub body: MethodBodyKind,
}

pub enum MethodBodyKind {
    Block(Block),
    Error(Error),
}

pub struct GlobDecl {
    pub span: Span,
    pub id: Ident,
    pub typ: TypeKind,
    pub val: Option<GlobDeclValue>,
}

// Statements

pub enum StmtKind {
    Assign(AssignStmt),
    Decls(MultiDeclStmt),
    If(IfStmt),
    While(WhileStmt),
    Return(ReturnStmt),
    Call(ProcCall),
    Block(Block),
    Error(Error),
}

pub struct AssignStmt {
    pub span: Span,
    pub targets: PVec<TargetKind>,
    pub values: PVec<ExprKind>,
}

pub struct MultiDeclStmt {
    pub span: Span,
    pub decls: PVec<Decl>,
}

pub struct Decl {
    pub span: Span,
    pub id: Ident,
    pub typ: TypeKind,
}

pub struct IfStmt {
    pub span: Span,
    pub cond: P<ExprKind>,
    pub then_branch: P<StmtKind>,
    pub else_branch: Option<P<StmtKind>>,
}

pub struct WhileStmt {
    pub span: Span,
    pub cond: P<ExprKind>,
    pub body: P<StmtKind>,
}

pub struct ReturnStmt {
    pub span: Span,
    pub values: PVec<ExprKind>,
}

pub struct ProcCall {
    pub span: Span,
    pub id: Ident,
    pub args: PVec<ExprKind>,
}

pub struct Block {
    pub span: Span,
    pub stmts: PVec<StmtKind>,
}

// Types

pub enum TypeKind {
    Array(ArrType),
    Int(Span),
    Bool(Span),
    Error(Error),
}

pub struct ArrType {
    pub span: Span,
    pub of: P<TypeKind>,
    pub size: Option<P<ExprKind>>,
}

// Targets & LValues

pub enum TargetKind {
    LValue(LValueKind),
    Decl(Decl),
    Discard(Span),
    Error(Error),
}

pub enum LValueKind {
    Id(Ident),
    Index(ExprIndex),
}

// ---- Expressions ----

pub enum ExprKind {
    Id(Ident),
    Lit(LitKind),
    Index(ExprIndex),
    Call(ProcCall),
    Length(LengthCall),
    Unary(UnaryExpr),
    Binary(BinaryExpr),
    Error(Error),
}

pub struct Ident {
    pub span: Span,
    pub sym: Symbol,
}

pub enum GlobDeclValue {
    Literal(LitKind),
    Neg { span: Span, value: IntLit },
}

pub enum LitKind {
    Int(IntLit),
    Bool(BoolLit),
    Char(CharLit),
    Array(ArrayLitKind),
}

pub struct IntLit {
    pub span: Span,
    pub value: u64,
}

pub struct BoolLit {
    pub span: Span,
    pub value: bool,
}

pub struct CharLit {
    pub span: Span,
    pub value: char,
}

pub enum ArrayLitKind {
    String(StringLit),
    Array(ArrayLit),
}

pub struct StringLit {
    pub span: Span,
    pub value: Symbol,
}

pub struct ArrayLit {
    pub span: Span,
    pub value: PVec<ExprKind>,
}

pub struct ExprIndex {
    pub span: Span,
    pub array: P<ExprKind>,
    pub index: P<ExprKind>,
}

pub struct LengthCall {
    pub span: Span,
    pub param: P<ExprKind>,
}

// Operators
pub struct UnaryExpr {
    pub span: Span,
    pub op: UOp,
    pub operand: P<ExprKind>,
}

pub struct UOp {
    pub span: Span,
    pub kind: UOpKind,
}

pub enum UOpKind {
    Neg,
    Not,
}

pub struct BinaryExpr {
    pub span: Span,
    pub op: BinOp,
    pub lhs: P<ExprKind>,
    pub rhs: P<ExprKind>,
}

pub struct BinOp {
    pub span: Span,
    pub kind: BinOpKind,
}

pub enum BinOpKind {
    Add,
    Sub,
    Mul,
    HighMul,
    Div,
    Mod,
    Eq,
    Neq,
    Lt,
    Gt,
    Le,
    Ge,
    LAnd,
    LOr,
}
