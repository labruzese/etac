use etac_errors::dcx::Diag;
use etac_intern::{EtacInterner};

#[derive(Debug)]
pub enum Parsed<Out> {
    Ok(Out),
    Recovered(Out),
    Failed,
}

impl<Out> Parsed<Out> {
    pub fn output(&self) -> Option<&Out> {
        match self {
            Parsed::Ok(ast) | Parsed::Recovered(ast) => Some(ast),
            Parsed::Failed => None,
        }
    }
}

pub struct ParseResult<'src, 'dcx, Out> {
    pub parse: Parsed<Out>,
    pub diags: Vec<Diag<'dcx>>,
    pub symbols: &'src mut EtacInterner,
}

