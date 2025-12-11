//! Scope management for semantic analysis

use std::collections::HashMap;
use crate::symbol_table::Symbol;

/// Types of scopes in Silk
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScopeKind {
    /// Global/module scope
    Global,
    /// Function scope
    Function,
    /// Class scope
    Class,
    /// Local scope (e.g., inside if, while, for blocks)
    Local,
}

/// Represents a single scope containing symbols
#[derive(Debug, Clone)]
pub struct Scope {
    /// The kind of this scope
    pub kind: ScopeKind,
    /// Symbols defined in this scope
    symbols: HashMap<String, Symbol>,
    /// Index of parent scope (None for global scope)
    parent: Option<usize>,
}

impl Scope {
    /// Create a new scope
    pub fn new(kind: ScopeKind, parent: Option<usize>) -> Self {
        Self {
            kind,
            symbols: HashMap::new(),
            parent,
        }
    }

    /// Define a symbol in this scope
    pub fn define(&mut self, name: String, symbol: Symbol) -> Option<Symbol> {
        self.symbols.insert(name, symbol)
    }

    /// Look up a symbol in this scope only (does not search parent scopes)
    pub fn lookup_local(&self, name: &str) -> Option<&Symbol> {
        self.symbols.get(name)
    }

    /// Look up a symbol mutably in this scope only (does not search parent scopes)
    pub fn lookup_local_mut(&mut self, name: &str) -> Option<&mut Symbol> {
        self.symbols.get_mut(name)
    }

    /// Get parent scope index
    pub fn parent(&self) -> Option<usize> {
        self.parent
    }

    /// Get all symbols in this scope
    pub fn symbols(&self) -> &HashMap<String, Symbol> {
        &self.symbols
    }
}
