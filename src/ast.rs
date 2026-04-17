pub type Id = String;
pub type IntLit = i64;
pub type BoolLit = bool;
pub type CharLit = char;
pub type StringLit = String;

#[derive(Debug)]
pub enum Program {
    Prog {
        uses: Vec<Use>,
        definitions: Vec<Definition>,
    },
}

#[derive(Debug)]
pub enum Use {
    Id(Id),
}

#[derive(Debug)]
pub enum Definition {
    Method(Method),
    GlobDecl(GlobDecl),
}

#[derive(Debug)]
pub enum Method {
    Method {
        id: Id,
        params: Vec<Decl>,
        ret_types: Vec<Type>,
        body: Block,
    },
}

#[derive(Debug)]
pub enum GlobDecl {
    GlobDecl {
        id: Id,
        typ: Type,
        val: Option<Value>,
    },
}

#[derive(Debug)]
pub enum Value {
    IntLit(IntLit),
    BoolLit(BoolLit),
}

#[derive(Debug)]
pub enum Decl {
    Decl { id: Id, typ: Type },
}

#[derive(Debug)]
pub enum Block {
    Block { stmts: Vec<Stmt> },
}

#[derive(Debug)]
pub enum Type {
    Type {
        base: BaseType,
        static_dims: Vec<u32>, // const_f_arr* — integer literals only
        empty_dims: u32,       // count of e_arr ([]) markers
    },
}

#[derive(Debug)]
pub enum BaseType {
    Int,
    Bool,
}

#[derive(Debug)]
pub enum Stmt {
    Assignment(Assignment),
    IfStmt(IfStmt),
    WhileStmt(WhileStmt),
    ReturnStmt(ReturnStmt),
    ProcCall(ProcCall),
    Block(Block),
    Decls {
        decls: Vec<Decl>,
        assignment: Option<Expr>,
    },
}

#[derive(Debug)]
pub enum Assignment {
    Assignment {
        targets: Vec<AssignLeft>,
        value: Expr,
    },
}

#[derive(Debug)]
pub enum AssignLeft {
    Id { id: Id, indices: Vec<Expr> }, // indices may be empty
    Ignore,                             // `_`
}

#[derive(Debug)]
pub enum IfStmt {
    IfStmt {
        cond: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
}

#[derive(Debug)]
pub enum WhileStmt {
    WhileStmt {
        cond: Expr,
        body: Box<Stmt>,
    },
}

#[derive(Debug)]
pub enum ReturnStmt {
    ReturnStmt { values: Vec<Expr> }, // empty Vec represents bare `return`
}

#[derive(Debug)]
pub enum ProcCall {
    ProcCall { id: Id, args: Vec<Expr> },
}

#[derive(Debug)]
pub enum Expr {
    Id(Id),
    Lit(Lit),
    Index { id: Id, indices: Vec<Expr> },  // from postfix_expr with dyn_f_arr+
    Call(ProcCall),                        // func_call == proc_call shape
    Unary { op: UOp, expr: Box<Expr> },
    Binary { op: BinOp, left: Box<Expr>, right: Box<Expr> },
}

#[derive(Debug)]
pub enum Lit {
    IntLit(IntLit),
    BoolLit(BoolLit),
    CharLit(CharLit),
    ArrLit(ArrLit),
}

#[derive(Debug)]
pub enum ArrLit {
    Array(Vec<Expr>),
    StringLit(StringLit),
}

#[derive(Debug)]
pub enum UOp {
    Neg, // -
    Not, // !
}

#[derive(Debug)]
pub enum BinOp {
    Add, Sub, Mul, Div, Mod,
    Eq, Neq, Lt, Gt, Le, Ge,
    And, Or,
}
