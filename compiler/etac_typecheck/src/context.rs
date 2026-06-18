//! The typing context

use crate::types::{CoreTy, IdTy};
use std::collections::HashMap;

type Scope = HashMap<String, IdTy>;

pub struct Env {
    scopes: Vec<Scope>,
}

impl Env {
    /// A fresh environment with a single (global) scope.
    pub fn new() -> Self {
        Self { scopes: vec![Scope::new()] }
    }

    pub fn push_scope(&mut self) {
        self.scopes.push(Scope::new());
    }

    pub fn pop_scope(&mut self) {
        debug_assert!(self.scopes.len() > 1, "cannot pop the global scope");
        self.scopes.pop();
    }

    /// Look up a name across all enclosing scopes, innermost first.
    pub fn lookup(&self, ident: &str) -> Option<&IdTy> {
        self.scopes.iter().rev().find_map(|s| s.get(ident))
    }

    /// Look up a name
    /// Returns `None` if unbound *or* bound to a function/return entry.
    pub fn lookup_var(&self, ident: &str) -> Option<&CoreTy> {
        match self.lookup(ident)? {
            IdTy::Var(t) => Some(t),
            _ => None,
        }
    }

    /// Look up a name that must be a function/procedure `fn T -> T'`.
    pub fn lookup_fn(&self, ident: &str) -> Option<(&[CoreTy], &[CoreTy])> {
        match self.lookup(ident)? {
            IdTy::Fn { from, to } => Some((from, to)),
            _ => None,
        }
    }

    /// True if `ident` is already bound anywhere in scope
    pub fn is_bound(&self, ident: &str) -> bool {
        self.lookup(ident).is_some()
    }

    /// Bind a name in the innermost scope. Callers check `is_bound` first and
    /// emit "Duplicate variable" rather than silently overwriting.
    pub fn insert(&mut self, ident: String, ctx: IdTy) {
        self.scopes
            .last_mut()
            .expect("Env always has at least one scope")
            .insert(ident, ctx);
    }
}

impl Default for Env {
    fn default() -> Self {
        Self::new()
    }
}
