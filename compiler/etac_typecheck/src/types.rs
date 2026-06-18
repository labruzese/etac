// --- Types ---
pub(crate) trait EtacType {} 

impl EtacType for Ty {}
pub enum Ty {
    Core(CoreTy),
    Tuple(TupleTy),
    Stmt(StmtTy),
}

impl EtacType for TupleTy {}
pub type TupleTy = Vec<CoreTy>;

impl EtacType for CoreTy {}
pub enum CoreTy {
    Int,
    Bool,
    Array(Box<CoreTy>),
    Err,
}

impl EtacType for StmtTy {}
pub enum StmtTy {
    Unit,
    Void,
}

impl EtacType for FnTy {}
pub struct FnTy {
    pub from: TupleTy,
    pub to: TupleTy,
}

// --- Context ---
impl EtacType for IdTy {}
pub enum IdTy {
    Var(CoreTy),
    Ret(TupleTy),
    Fn(FnTy),
}

// --- Conversions ---
impl From<&etac_ast::TypeKind> for CoreTy {
    fn from(value: &etac_ast::TypeKind) -> Self {
        match value {
            etac_ast::TypeKind::UnsizedArray { of } |
            etac_ast::TypeKind::SizedArray { of, size: _ } => CoreTy::Array(Box::new(CoreTy::from(&of.kind))),
            etac_ast::TypeKind::Int => CoreTy::Int,
            etac_ast::TypeKind::Bool => CoreTy::Bool,
        }
}
}
