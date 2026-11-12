use etac_intern::Symbol;
use etac_source::Span;

// Top level
pub struct Error(pub Span);

pub struct Program {
    pub span: Span,
    pub uses: Vec<Use>,
    pub definitions: Vec<DefinitionKind>,
}

pub struct Interface {
    pub span: Span,
    pub items: Vec<InterfaceItemKind>,
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
    pub params: Vec<Decl>,
    pub ret_types: Vec<TypeKind>,
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
    pub targets: Vec<TargetKind>,
    pub values: Vec<ExprKind>,
}

pub struct MultiDeclStmt {
    pub span: Span,
    pub decls: Vec<Decl>,
}

pub struct Decl {
    pub span: Span,
    pub id: Ident,
    pub typ: TypeKind,
}

pub struct IfStmt {
    pub span: Span,
    pub cond: Box<ExprKind>,
    pub then_branch: Box<StmtKind>,
    pub else_branch: Option<Box<StmtKind>>,
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
    pub args: Vec<ExprKind>,
}

pub struct Block {
    pub span: Span,
    pub stmts: Vec<StmtKind>,
}

// Types

pub enum TypeKind {
    Array(ArrType),
    Int(Span),
    Bool(Span),
    Error(Error),
}

/// An array of `size` elements of type `of`, or of unknown length if there is no `size`.
///
/// `int[3][4]` is an array of 3 `int[4]`s: the outer node has size `3`, and its `of` is the array with size `4`.
pub struct ArrType {
    /// The whole type as written, for every level of a multi-dimensional array.
    pub span: Span,
    pub of: Box<TypeKind>,
    pub size: Option<Box<ExprKind>>,
}

// Targets & LValues

pub enum TargetKind {
    LValue(LValueKind),
    Decl(Decl),
    Discard(Span),
    /// Something that can't be assigned to. Already reported.
    Error(Error),
}

/// Something that can be assigned to: a variable, or an element of an array.
///
/// The array being indexed can be any expression, including a call (`f()[0] = 1`), but a call on its own is not
/// assignable.
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
    pub value: Vec<ExprKind>,
}

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
