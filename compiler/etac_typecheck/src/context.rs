//! The typing context

use etac_ast::{NodeId, SpanTable};
use etac_errors::DiagCtxt;
use etac_span::SourceCache;

use crate::types::*;
use std::any::Any;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

#[derive(Debug)]
pub struct VarEntry {
    pub ty: VarTy,
    pub declared: NodeId,
}

#[derive(Debug)]
pub struct FnEntry {
    pub ty: FnTy,
    pub declared: NodeId,
}

#[derive(Debug)]
pub struct RetEntry {
    pub ty: TupleTy,
    pub declared: NodeId,
}

#[derive(Debug, Default)]
pub struct Scope {
    vars: HashMap<String, VarEntry>,
    fns: HashMap<String, FnEntry>,
    ret: Option<RetEntry>
}

#[derive(Debug, Default)]
pub(crate) struct Scopes(Vec<Scope>);

#[derive(Debug, Default)]
pub(crate) struct Types(HashMap<NodeId, Box<dyn EtaType>>);

#[derive(Debug)]
pub struct Env<'dcx, C: SourceCache> {
    pub dcx: &'dcx DiagCtxt<C>,
    pub span_table: &'dcx SpanTable,
    pub scopes: Scopes,
    pub types: Types,
}

impl<'dcx, C: SourceCache> Env<'dcx, C> {
    pub fn new(dcx: &'dcx DiagCtxt<C>, span_table: &'dcx SpanTable) -> Self {
        Self { dcx, span_table, scopes: Scopes(vec![Scope::default()]), types: Types(HashMap::default()) }
    }
}

impl Scopes {
    pub fn current_mut(&mut self) -> &mut Scope {
        self.0.last_mut().expect("at least 1 scope")
    }
    pub fn current(&self) -> &Scope {
        self.0.last().expect("at least 1 scope")
    }

    pub fn push(&mut self) {
        self.0.push(Scope::default());
    }
    pub fn pop(&mut self) {
        debug_assert!(self.0.len() > 1, "cannot pop the global scope");
        self.0.pop();
    }

    pub fn lookup_var(&self, bind: &str) -> Option<&VarEntry> {
        self.0.iter().rev().find_map(|s| s.vars.get(bind))
    }
    pub fn lookup_fn(&self, bind: &str) -> Option<&FnEntry> {
        self.0.iter().rev().find_map(|s| s.fns.get(bind))
    }
    pub fn lookup_ret(&self) -> Option<&RetEntry> {
        self.0.iter().rev().find_map(|s| s.ret.as_ref())
    }

    pub fn declare_var(
        &mut self,
        declaration: NodeId,
        binding: String,
        ty: VarTy,
    ) -> Result<&VarEntry, &VarEntry> {
        match self.current_mut().vars.entry(binding) {
            Entry::Occupied(entry) => Err(entry.into_mut()),
            Entry::Vacant(entry) => {
                Ok(entry.insert(VarEntry { ty, declared: declaration }))
            }
        }
    }

    pub fn declare_fn(
        &mut self,
        declaration: NodeId,
        binding: String,
        ty: FnTy,
    ) -> Result<&FnEntry, &FnEntry> {
        match self.current_mut().fns.entry(binding) {
            Entry::Occupied(entry) => Err(entry.into_mut()),
            Entry::Vacant(entry) => {
                Ok(entry.insert(FnEntry { ty, declared: declaration }))
            }
        }
    }

    pub fn declare_ret_type(&mut self, declaration: NodeId, ty: TupleTy) {
        self.current_mut().ret = Some(RetEntry { ty, declared: declaration });
    }
}

impl Types {
    pub fn assign_type<T: EtaType>(&mut self, node: NodeId, ty: Box<T>) -> &T {
        let _previous = self.0.insert(node.clone(), ty);
        debug_assert!(_previous.is_none(), "type should only be assigned once");
        self.lookup_type(node).unwrap()
    }

    pub fn lookup_type<T: EtaType>(&self, node: NodeId) -> Option<&T> {
        let any: &dyn Any = self.0.get(&node).unwrap().as_ref();
        any.downcast_ref::<T>()
    }
}
