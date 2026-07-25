use etac_intern::Symbol;
use etac_source::Span;

// Top level

pub struct Program {
    pub span: Span,
    pub uses: Vec<Use>,
    pub definitions: Vec<DefinitionKind>,
}

pub struct Interface {
    pub span: Span,
    pub items: Vec<InterfaceItemKind>
}

pub struct Use {
    pub span: Span,
    pub interface: Ident,
}

// Definitions

pub enum DefinitionKind {
    Method(Method),
    GlobDecl(GlobDecl),
    Error(Span),
}

pub enum InterfaceItemKind {
    MethodDecl(MethodDecl),
    Error(Span),
}

pub struct MethodDecl {
    pub span: Span,
    pub id: Ident,
    pub params: Vec<Decl>,
    pub ret_types: Vec<TypeKind>
}

pub struct Method {
    pub span: Span,
    pub decl: MethodDecl,
    pub body: MethodBodyKind,
}

pub enum MethodBodyKind {
    Block(Block),
    Error(Span),
}

pub struct GlobDecl {
    pub span: Span,
    pub id: Ident,
    pub typ: TypeKind,
    pub val: Option<LitKind>
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
    Error(Span),
}

pub struct AssignStmt {
    pub span: Span,
    pub targets: Vec<TargetKind>,
    pub values: Vec<ExprKind>,
}

pub struct MultiDeclStmt {
    pub span: Span,
    pub decls: Vec<Decl>
}

pub struct Decl {
    pub span: Span,
    pub id: Ident,
    pub typ: TypeKind
}

pub struct IfStmt {
    pub span: Span,
    pub cond: Box<ExprKind>,
    pub then_branch: Box<StmtKind>,
    pub else_branch: Option<Box<StmtKind>>
}

pub struct WhileStmt {
    pub span: Span,
    pub cond: Box<ExprKind>,
    pub body: Box<StmtKind>,
}

pub struct ReturnStmt {
    pub span: Span,
    pub values: Vec<ExprKind>,
}

pub struct ProcCall {
    pub span: Span,
    pub id: Ident,
    pub args: Vec<ExprKind>
}

pub struct Block {
    pub span: Span,
    pub stmts: Vec<StmtKind>
}


// Types

pub enum TypeKind {
    Array(ArrType),
    Int(Span),
    Bool(Span),
    Error(Span),
}

pub struct ArrType { 
    pub span: Span,
    pub of: Box<TypeKind>,
    pub size: Option<Box<ExprKind>> 
}

// Targets & LValues

pub enum TargetKind {
    LValue(LValueKind),
    Decl(Decl),
    Discard(Span),
}

pub enum LValueKind {
    Id(Ident),
    ProcCall(ProcCall),
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
    Error(Span),
}

pub struct Ident {
    pub span: Span,
    pub sym: Symbol,
}

pub enum LitKind {
    Int { span: Span, value: i64 },
    Bool { span: Span, value: bool },
    Char { span: Span, value: char },
    Array(ArrayLitKind),
}

pub enum ArrayLitKind {
    String(StringLit),
    Array(ArrayLit),
}

pub struct StringLit{ pub span: Span, pub value: Vec<char> }

pub struct ArrayLit{ pub span: Span, pub value: Vec<ExprKind> }

pub struct ExprIndex {
    pub span: Span,
    pub array: Box<ExprKind>,
    pub index: Box<ExprKind>,
}

pub struct LengthCall {
    pub span: Span,
    pub param: Box<ExprKind>,
}

// Operators
pub struct UnaryExpr {
    pub span: Span,
    pub op: UOp,
    pub operand: Box<ExprKind>,
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
    pub lhs: Box<ExprKind>,
    pub rhs: Box<ExprKind>,
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
