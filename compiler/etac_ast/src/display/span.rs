use std::fmt;
use std::ops::ControlFlow;

use etac_source::Span;

use crate::ast::*;
use crate::visit::Visitor;
use crate::walk::Walkable;

pub struct SpanPrinter<'ast, 'sym, ResT: etac_intern::Resolver, NodeT> {
    span_fmt: Box<dyn Fn(Span) -> String + 'ast>,
    symbols: &'sym ResT,
    ty: &'ast NodeT,
}

impl<'ast, 'sym, ResT: etac_intern::Resolver, NodeT> SpanPrinter<'ast, 'sym, ResT, NodeT> {
    pub const PADDING: usize = 16;
    pub fn new(node: &'ast NodeT, symbols: &'sym ResT, span_fmt: impl Fn(Span) -> String + 'ast) -> Self {
        SpanPrinter {
            span_fmt: Box::new(span_fmt),
            symbols,
            ty: node,
        }
    }
}

impl<'ast, 'sym, ResT: etac_intern::Resolver> std::fmt::Display for SpanPrinter<'ast, 'sym, ResT, Program> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        SpanPrinterWriter {
            out: f,
            printer: self
        }.visit_program(self.ty).continue_ok()
    }
}

impl<'ast, 'sym, ResT: etac_intern::Resolver> std::fmt::Display for SpanPrinter<'ast, 'sym, ResT, Interface> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        SpanPrinterWriter {
            out: f,
            printer: self
        }.visit_interface(self.ty).continue_ok()
    }
}

struct SpanPrinterWriter<'ast, 'sym, 'buf, 'prt, 'fmt, ResT: etac_intern::Resolver, NodeT> {
    out: &'fmt mut fmt::Formatter<'buf>,
    printer: &'prt SpanPrinter<'ast, 'sym, ResT, NodeT>
}

impl<'ast, 'sym, 'buf, 'prt, 'fmt, ResT: etac_intern::Resolver, NodeT> SpanPrinterWriter<'ast, 'sym, 'buf, 'prt, 'fmt, ResT, NodeT> {
    fn write(&mut self, tag: &str, span: &Span) -> ControlFlow<fmt::Error> {
        match writeln!(
            self.out,
            "{:<width$}: {}",
            tag,
            (self.printer.span_fmt)(*span),
            width = SpanPrinter::<'ast, 'sym, ResT, NodeT>::PADDING
        ) {
            Ok(()) => ControlFlow::Continue(()),
            Err(e) => ControlFlow::Break(e),
        }
    }
}

impl<'ast, 'sym, 'buf, 'prt, 'fmt, ResT: etac_intern::Resolver, NodeT> Visitor<'ast> for SpanPrinterWriter<'ast, 'sym, 'buf, 'prt, 'fmt, ResT, NodeT> {
    type Result = ControlFlow<fmt::Error>;

    fn visit_program(&mut self, n: &'ast Program) -> Self::Result {
        let Program { span, .. } = n;
        self.write("program", span)?;
        n.walk(self)
    }

    fn visit_interface(&mut self, n: &'ast Interface) -> Self::Result {
        let Interface { span, .. } = n;
        self.write("interface", span)?;
        n.walk(self)
    }

    fn visit_use(&mut self, n: &'ast Use) -> Self::Result {
        let Use { span, .. } = n;
        self.write("use", span)?;
        n.walk(self)
    }

    fn visit_method_decl(&mut self, n: &'ast MethodDecl) -> Self::Result {
        let MethodDecl { span, .. } = n;
        self.write("mdecl", span)?;
        n.walk(self)
    }

    fn visit_method(&mut self, n: &'ast Method) -> Self::Result {
        let Method { span, .. } = n;
        self.write("method", span)?;
        n.walk(self)
    }


    fn visit_glob_decl(&mut self, n: &'ast GlobDecl) -> Self::Result {
        let GlobDecl { span, .. } = n;
        self.write("gdecl", span)?;
        n.walk(self)
    }

    fn visit_assign_stmt(&mut self, n: &'ast AssignStmt) -> Self::Result {
        let AssignStmt { span, .. } = n;
        self.write("assign", span)?;
        n.walk(self)
    }

    fn visit_multi_decl_stmt(&mut self, n: &'ast MultiDeclStmt) -> Self::Result {
        let MultiDeclStmt { span, .. } = n;
        self.write("decls", span)?;
        n.walk(self)
    }

    fn visit_decl(&mut self, n: &'ast Decl) -> Self::Result {
        let Decl { span, .. } = n;
        self.write("decl", span)?;
        n.walk(self)
    }

    fn visit_if_stmt(&mut self, n: &'ast IfStmt) -> Self::Result {
        let IfStmt { span, .. } = n;
        self.write("if", span)?;
        n.walk(self)
    }

    fn visit_while_stmt(&mut self, n: &'ast WhileStmt) -> Self::Result {
        let WhileStmt { span, .. } = n;
        self.write("while", span)?;
        n.walk(self)
    }

    fn visit_return_stmt(&mut self, n: &'ast ReturnStmt) -> Self::Result {
        let ReturnStmt { span, .. } = n;
        self.write("return", span)?;
        n.walk(self)
    }

    fn visit_proc_call(&mut self, n: &'ast ProcCall) -> Self::Result {
        let ProcCall { span, .. } = n;
        self.write("pcall", span)?;
        n.walk(self)
    }

    fn visit_block(&mut self, n: &'ast Block) -> Self::Result {
        let Block { span, .. } = n;
        self.write("block", span)?;
        n.walk(self)
    }

    fn visit_arr_type(&mut self, n: &'ast ArrType) -> Self::Result {
        let ArrType { span, .. } = n;
        self.write("arr_t", span)?;
        n.walk(self)
    }

    fn visit_ident(&mut self, n: &'ast Ident) -> Self::Result {
        let Ident { span, sym, .. } = n;
        self.write(format!("id {}", self.printer.symbols.resolve(sym)).as_str(), span)?;
        n.walk(self)
    }

    fn visit_int_lit(&mut self, n: &'ast IntLit) -> Self::Result {
        let IntLit { span, .. } = n;
        self.write("int_lit", span)?;
        n.walk(self)
    }
    
    fn visit_bool_lit(&mut self, n: &'ast BoolLit) -> Self::Result {
        let BoolLit { span, .. } = n;
        self.write("bool_lit", span)?;
        n.walk(self)
    }

    fn visit_char_lit(&mut self, n: &'ast CharLit) -> Self::Result {
        let CharLit { span, .. } = n;
        self.write("char_lit", span)?;
        n.walk(self)
    }

    fn visit_string_lit(&mut self, n: &'ast StringLit) -> Self::Result {
        let StringLit { span, .. } = n;
        self.write("string_lit", span)?;
        n.walk(self)
    }

    fn visit_array_lit(&mut self, n: &'ast ArrayLit) -> Self::Result {
        let ArrayLit { span, .. } = n;
        self.write("arr_lit", span)?;
        n.walk(self)
    }

    fn visit_expr_index(&mut self, n: &'ast ExprIndex) -> Self::Result {
        let ExprIndex { span, .. } = n;
        self.write("expr_idx", span)?;
        n.walk(self)
    }

    fn visit_length_call(&mut self, n: &'ast LengthCall) -> Self::Result {
        let LengthCall { span, .. } = n;
        self.write("length", span)?;
        n.walk(self)
    }

    fn visit_unary_expr(&mut self, n: &'ast UnaryExpr) -> Self::Result {
        let UnaryExpr { span, .. } = n;
        self.write("uexpr", span)?;
        n.walk(self)
    }

    fn visit_uop(&mut self, n: &'ast UOp) -> Self::Result {
        let UOp { span, .. } = n;
        self.write("uop", span)?;
        n.walk(self)
    }

    fn visit_binary_expr(&mut self, n: &'ast BinaryExpr) -> Self::Result {
        let BinaryExpr { span, .. } = n;
        self.write("bexpr", span)?;
        n.walk(self)
    }

    fn visit_bin_op(&mut self, n: &'ast BinOp) -> Self::Result {
        let BinOp { span, .. } = n;
        self.write("binop", span)?;
        n.walk(self)
    }
 }
