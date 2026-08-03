use std::fmt;
use std::ops::ControlFlow;

use crate::ast::*;
use crate::visit::{Visitable, Visitor};

pub struct SexpPrinter<'ast, 'sym, ResT: etac_intern::Resolver, NodeT> {
    symbols: &'sym ResT,
    ty: &'ast NodeT,
}

impl<'ast, 'sym, ResT: etac_intern::Resolver, NodeT> SexpPrinter<'ast, 'sym, ResT, NodeT> {
    pub fn new(resolver: &'sym ResT, node: &'ast NodeT) -> Self {
        SexpPrinter {
            symbols: resolver,
            ty: node,
        }
    }
}

pub struct SexpPrinterWriter<'ast, 'sym, 'buf, 'prt, 'fmt, ResT: etac_intern::Resolver, NodeT> {
    printer: &'prt SexpPrinter<'ast, 'sym, ResT, NodeT>,
    out: &'fmt mut fmt::Formatter<'buf>,
    depth: usize,
}

impl<'ast, 'sym, R: etac_intern::Resolver> std::fmt::Display for SexpPrinter<'ast, 'sym, R, Program> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        SexpPrinterWriter {
            printer: self,
            out: f,
            depth: 0,
        }
        .visit_program(self.ty)
        .continue_ok()
    }
}

impl<'ast, 'sym, R: etac_intern::Resolver> std::fmt::Display for SexpPrinter<'ast, 'sym, R, Interface> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        SexpPrinterWriter {
            printer: self,
            out: f,
            depth: 0,
        }
        .visit_interface(self.ty)
        .continue_ok()
    }
}

impl<'ast, 'sym, 'buf, 'prt, 'fmt, ResT: etac_intern::Resolver, NodeT>
    SexpPrinterWriter<'ast, 'sym, 'buf, 'prt, 'fmt, ResT, NodeT>
{
    fn raw(&mut self, args: fmt::Arguments<'_>) -> ControlFlow<fmt::Error> {
        match self.out.write_fmt(args) {
            Ok(()) => ControlFlow::Continue(()),
            Err(e) => ControlFlow::Break(e),
        }
    }
    fn atom(&mut self, s: &str) -> ControlFlow<fmt::Error> {
        self.raw(format_args!("{s}"))
    }
    fn open(&mut self) -> ControlFlow<fmt::Error> {
        self.raw(format_args!("("))
    }
    fn close(&mut self) -> ControlFlow<fmt::Error> {
        self.raw(format_args!(")"))
    }
    fn sp(&mut self) -> ControlFlow<fmt::Error> {
        self.raw(format_args!(" "))
    }
    fn indent(&mut self) {
        self.depth += 1;
    }
    fn dedent(&mut self) {
        self.depth -= 1;
    }
    fn br(&mut self) -> ControlFlow<fmt::Error> {
        self.raw(format_args!("\n"))?;
        for _ in 0..self.depth {
            self.raw(format_args!("  "))?;
        }
        ControlFlow::Continue(())
    }
}

/// `(tag child child …)` on one line.
macro_rules! sexp {
    ($p:expr, $tag:expr $(, $child:expr)* $(,)?) => {{
        $p.open()?;
        $p.atom($tag)?;
        $( $p.sp()?; $child.visit($p)?; )*
        $p.close()
    }};
}

/// `(tag a b c)` from a slice, one line.
macro_rules! sexp_seq {
    ($p:expr, $tag:expr, $items:expr) => {{
        $p.open()?;
        $p.atom($tag)?;
        for it in $items {
            $p.sp()?;
            it.visit($p)?;
        }
        $p.close()
    }};
}

/// `(tag\n  a\n  b)` — one child per line.
macro_rules! sexp_block {
    ($p:expr, $tag:expr, $items:expr) => {{
        $p.open()?;
        $p.atom($tag)?;
        $p.indent();
        for it in $items {
            $p.br()?;
            it.visit($p)?;
        }
        $p.dedent();
        $p.close()
    }};
}
impl<'ast, 'sym, 'buf, 'prt, 'fmt, ResT: etac_intern::Resolver, NodeT> Visitor<'ast>
    for SexpPrinterWriter<'ast, 'sym, 'buf, 'prt, 'fmt, ResT, NodeT>
{
    type Result = ControlFlow<fmt::Error>;

    // ---- Top level ----

    fn visit_program(&mut self, n: &'ast Program) -> Self::Result {
        let Program {
            span: _,
            uses,
            definitions,
        } = n;
        self.open()?;
        self.atom("program")?;
        self.indent();
        self.br()?;
        sexp_block!(self, "uses", uses)?;
        self.br()?;
        sexp_block!(self, "defs", definitions)?;
        self.dedent();
        self.close()
    }

    fn visit_interface(&mut self, n: &'ast Interface) -> Self::Result {
        let Interface { span: _, items } = n;
        sexp_block!(self, "interface", items)
    }

    fn visit_use(&mut self, n: &'ast Use) -> Self::Result {
        let Use { span: _, interface } = n;
        sexp!(self, "use", interface)
    }

    fn visit_error(&mut self, n: &'ast Error) -> Self::Result {
        let Error(_span) = n;
        self.atom("(error)")
    }

    // ---- Definitions ----

    fn visit_definition_kind(&mut self, n: &'ast DefinitionKind) -> Self::Result {
        match n {
            DefinitionKind::Method(m) => m.visit(self),
            DefinitionKind::GlobDecl(g) => g.visit(self),
            DefinitionKind::Error(e) => e.visit(self),
        }
    }

    fn visit_interface_item_kind(&mut self, n: &'ast InterfaceItemKind) -> Self::Result {
        match n {
            InterfaceItemKind::MethodDecl(d) => d.visit(self),
            InterfaceItemKind::Error(e) => e.visit(self),
        }
    }

    fn visit_method_decl(&mut self, n: &'ast MethodDecl) -> Self::Result {
        let MethodDecl {
            span: _,
            id,
            params,
            ret_types,
        } = n;
        self.open()?;
        self.atom("decl")?;
        self.sp()?;
        id.visit(self)?;
        self.sp()?;
        sexp_seq!(self, "params", params)?;
        self.sp()?;
        sexp_seq!(self, "returns", ret_types)?;
        self.close()
    }

    fn visit_method(&mut self, n: &'ast Method) -> Self::Result {
        let Method { span: _, decl, body } = n;
        self.open()?;
        self.atom("method")?;
        self.sp()?;
        decl.visit(self)?;
        self.indent();
        self.br()?;
        body.visit(self)?;
        self.dedent();
        self.close()
    }

    fn visit_method_body_kind(&mut self, n: &'ast MethodBodyKind) -> Self::Result {
        match n {
            MethodBodyKind::Block(b) => b.visit(self),
            MethodBodyKind::Error(e) => e.visit(self),
        }
    }

    fn visit_glob_decl(&mut self, n: &'ast GlobDecl) -> Self::Result {
        let GlobDecl { span: _, id, typ, val } = n;
        self.open()?;
        self.atom("global")?;
        self.sp()?;
        id.visit(self)?;
        self.sp()?;
        typ.visit(self)?;
        if let Some(v) = val {
            self.sp()?;
            v.visit(self)?;
        }
        self.close()
    }

    // ---- Statements ----

    fn visit_stmt_kind(&mut self, n: &'ast StmtKind) -> Self::Result {
        match n {
            StmtKind::Assign(s) => s.visit(self),
            StmtKind::Decls(s) => s.visit(self),
            StmtKind::If(s) => s.visit(self),
            StmtKind::While(s) => s.visit(self),
            StmtKind::Return(s) => s.visit(self),
            StmtKind::Call(s) => s.visit(self),
            StmtKind::Block(s) => s.visit(self),
            StmtKind::Error(e) => e.visit(self),
        }
    }

    fn visit_assign_stmt(&mut self, n: &'ast AssignStmt) -> Self::Result {
        let AssignStmt {
            span: _,
            targets,
            values,
        } = n;
        self.open()?;
        self.atom("assign")?;
        self.sp()?;
        sexp_seq!(self, "targets", targets)?;
        self.sp()?;
        sexp_seq!(self, "values", values)?;
        self.close()
    }

    fn visit_multi_decl_stmt(&mut self, n: &'ast MultiDeclStmt) -> Self::Result {
        let MultiDeclStmt { span: _, decls } = n;
        sexp_seq!(self, "decls", decls)
    }

    fn visit_decl(&mut self, n: &'ast Decl) -> Self::Result {
        let Decl { span: _, id, typ } = n;
        sexp!(self, ":", id, typ)
    }

    fn visit_if_stmt(&mut self, n: &'ast IfStmt) -> Self::Result {
        let IfStmt {
            span: _,
            cond,
            then_branch,
            else_branch,
        } = n;
        self.open()?;
        self.atom("if")?;
        self.sp()?;
        cond.visit(self)?;
        self.indent();
        self.br()?;
        then_branch.visit(self)?;
        if let Some(e) = else_branch {
            self.br()?;
            e.visit(self)?;
        }
        self.dedent();
        self.close()
    }

    fn visit_while_stmt(&mut self, n: &'ast WhileStmt) -> Self::Result {
        let WhileStmt { span: _, cond, body } = n;
        self.open()?;
        self.atom("while")?;
        self.sp()?;
        cond.visit(self)?;
        self.indent();
        self.br()?;
        body.visit(self)?;
        self.dedent();
        self.close()
    }

    fn visit_return_stmt(&mut self, n: &'ast ReturnStmt) -> Self::Result {
        let ReturnStmt { span: _, values } = n;
        sexp_seq!(self, "return", values)
    }

    fn visit_proc_call(&mut self, n: &'ast ProcCall) -> Self::Result {
        let ProcCall { span: _, id, args } = n;
        self.open()?;
        self.atom("call")?;
        self.sp()?;
        id.visit(self)?;
        for a in args {
            self.sp()?;
            a.visit(self)?;
        }
        self.close()
    }

    fn visit_block(&mut self, n: &'ast Block) -> Self::Result {
        let Block { span: _, stmts } = n;
        sexp_block!(self, "block", stmts)
    }

    // ---- Types ----

    fn visit_type_kind(&mut self, n: &'ast TypeKind) -> Self::Result {
        match n {
            TypeKind::Array(a) => a.visit(self),
            TypeKind::Int(_) => self.atom("int"),
            TypeKind::Bool(_) => self.atom("bool"),
            TypeKind::Error(e) => e.visit(self),
        }
    }

    fn visit_arr_type(&mut self, n: &'ast ArrType) -> Self::Result {
        let ArrType { span: _, of, size } = n;
        self.open()?;
        self.atom("array")?;
        self.sp()?;
        of.visit(self)?;
        if let Some(s) = size {
            self.sp()?;
            s.visit(self)?;
        }
        self.close()
    }

    // ---- Targets & LValues ----

    fn visit_target_kind(&mut self, n: &'ast TargetKind) -> Self::Result {
        match n {
            TargetKind::LValue(l) => l.visit(self),
            TargetKind::Decl(d) => d.visit(self),
            TargetKind::Discard(_) => self.atom("_"),
        }
    }

    fn visit_lvalue_kind(&mut self, n: &'ast LValueKind) -> Self::Result {
        match n {
            LValueKind::Id(i) => i.visit(self),
            LValueKind::ProcCall(c) => c.visit(self),
            LValueKind::Index(x) => x.visit(self),
        }
    }

    // ---- Expressions ----

    fn visit_expr_kind(&mut self, n: &'ast ExprKind) -> Self::Result {
        match n {
            ExprKind::Id(i) => i.visit(self),
            ExprKind::Lit(l) => l.visit(self),
            ExprKind::Index(x) => x.visit(self),
            ExprKind::Call(c) => c.visit(self),
            ExprKind::Length(l) => l.visit(self),
            ExprKind::Unary(u) => u.visit(self),
            ExprKind::Binary(b) => b.visit(self),
            ExprKind::Error(e) => e.visit(self),
        }
    }

    fn visit_ident(&mut self, n: &'ast Ident) -> Self::Result {
        let Ident { span: _, sym } = n;
        self.raw(format_args!("{}", self.printer.symbols.resolve(sym)))
    }

    fn visit_glob_decl_value(&mut self, n: &'ast GlobDeclValue) -> Self::Result {
        match n {
            GlobDeclValue::Literal(l) => l.visit(self),
            GlobDeclValue::Neg { span: _, value } => {
                self.atom("-")?;
                value.visit(self)
            }
        }
    }

    fn visit_lit_kind(&mut self, n: &'ast LitKind) -> Self::Result {
        match n {
            LitKind::Int(i) => i.visit(self),
            LitKind::Bool(b) => b.visit(self),
            LitKind::Char(c) => c.visit(self),
            LitKind::Array(a) => a.visit(self),
        }
    }

    fn visit_int_lit(&mut self, n: &'ast IntLit) -> Self::Result {
        let IntLit { span: _, value } = n;
        self.raw(format_args!("{value}"))
    }

    fn visit_bool_lit(&mut self, n: &'ast BoolLit) -> Self::Result {
        let BoolLit { span: _, value } = n;
        self.raw(format_args!("{value}"))
    }

    fn visit_char_lit(&mut self, n: &'ast CharLit) -> Self::Result {
        let CharLit { span: _, value } = n;
        self.raw(format_args!("{value}"))
    }

    fn visit_array_lit_kind(&mut self, n: &'ast ArrayLitKind) -> Self::Result {
        match n {
            ArrayLitKind::String(s) => s.visit(self),
            ArrayLitKind::Array(a) => a.visit(self),
        }
    }

    fn visit_string_lit(&mut self, n: &'ast StringLit) -> Self::Result {
        let StringLit { span: _, value } = n;
        self.raw(format_args!("{value:?}"))
    }

    fn visit_array_lit(&mut self, n: &'ast ArrayLit) -> Self::Result {
        let ArrayLit { span: _, value } = n;
        sexp_seq!(self, "array", value)
    }

    fn visit_expr_index(&mut self, n: &'ast ExprIndex) -> Self::Result {
        let ExprIndex { span: _, array, index } = n;
        sexp!(self, "index", array, index)
    }

    fn visit_length_call(&mut self, n: &'ast LengthCall) -> Self::Result {
        let LengthCall { span: _, param } = n;
        sexp!(self, "length", param)
    }

    // ---- Operators ----

    fn visit_unary_expr(&mut self, n: &'ast UnaryExpr) -> Self::Result {
        let UnaryExpr { span: _, op, operand } = n;
        self.open()?;
        op.visit(self)?;
        self.sp()?;
        operand.visit(self)?;
        self.close()
    }

    fn visit_uop(&mut self, n: &'ast UOp) -> Self::Result {
        let UOp { span: _, kind } = n;
        kind.visit(self)
    }

    fn visit_uop_kind(&mut self, n: &'ast UOpKind) -> Self::Result {
        self.atom(match n {
            UOpKind::Neg => "-",
            UOpKind::Not => "!",
        })
    }

    fn visit_binary_expr(&mut self, n: &'ast BinaryExpr) -> Self::Result {
        let BinaryExpr { span: _, op, lhs, rhs } = n;
        self.open()?;
        op.visit(self)?;
        self.sp()?;
        lhs.visit(self)?;
        self.sp()?;
        rhs.visit(self)?;
        self.close()
    }

    fn visit_bin_op(&mut self, n: &'ast BinOp) -> Self::Result {
        let BinOp { span: _, kind } = n;
        kind.visit(self)
    }

    fn visit_bin_op_kind(&mut self, n: &'ast BinOpKind) -> Self::Result {
        self.atom(match n {
            BinOpKind::Add => "+",
            BinOpKind::Sub => "-",
            BinOpKind::Mul => "*",
            BinOpKind::HighMul => "*>>",
            BinOpKind::Div => "/",
            BinOpKind::Mod => "%",
            BinOpKind::Eq => "==",
            BinOpKind::Neq => "!=",
            BinOpKind::Lt => "<",
            BinOpKind::Gt => ">",
            BinOpKind::Le => "<=",
            BinOpKind::Ge => ">=",
            BinOpKind::LAnd => "&",
            BinOpKind::LOr => "|",
        })
    }
}
