//! Symbol table for tracking declarations and their types

use silk_lexer::Span;
use crate::scope::{Scope, ScopeKind};
use crate::error::{SemanticError, SemanticResult};

/// Kind of symbol
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SymbolKind {
    /// Variable
    Variable,
    /// Function parameter
    Parameter,
    /// Function
    Function,
    /// Class
    Class,
    /// Module
    Module,
}

/// Represents a symbol in the symbol table
#[derive(Debug, Clone)]
pub struct Symbol {
    /// Name of the symbol
    pub name: String,
    /// Kind of symbol
    pub kind: SymbolKind,
    /// Location where the symbol was defined
    pub span: Span,
    /// Type information (to be expanded in future)
    pub type_info: Option<String>,
}

impl Symbol {
    /// Create a new symbol
    pub fn new(name: String, kind: SymbolKind, span: Span) -> Self {
        Self {
            name,
            kind,
            span,
            type_info: None,
        }
    }

    /// Create a new symbol with type information
    pub fn with_type(name: String, kind: SymbolKind, span: Span, type_info: String) -> Self {
        Self {
            name,
            kind,
            span,
            type_info: Some(type_info),
        }
    }
}

/// Symbol table for managing scopes and symbols
#[derive(Debug)]
pub struct SymbolTable {
    /// All scopes (using indices instead of references)
    scopes: Vec<Scope>,
    /// Index of the current scope
    current_scope: usize,
    /// Loop depth counter for break/continue validation
    loop_depth: usize,
}

impl SymbolTable {
    /// Create a new symbol table with a global scope
    pub fn new() -> Self {
        let global_scope = Scope::new(ScopeKind::Global, None);
        Self {
            scopes: vec![global_scope],
            current_scope: 0,
            loop_depth: 0,
        }
    }

    /// Enter a new scope
    pub fn enter_scope(&mut self, kind: ScopeKind) {
        let parent = self.current_scope;
        let new_scope = Scope::new(kind, Some(parent));
        self.scopes.push(new_scope);
        self.current_scope = self.scopes.len() - 1;
    }

    /// Exit the current scope
    pub fn exit_scope(&mut self) -> SemanticResult<()> {
        if self.current_scope == 0 {
            return Err(SemanticError::InvalidScope {
                message: "Cannot exit global scope".to_string(),
            });
        }

        let parent = self.scopes[self.current_scope]
            .parent()
            .expect("Non-global scope must have a parent");
        self.current_scope = parent;
        Ok(())
    }

    /// Define a symbol in the current scope
    pub fn define_symbol(&mut self, symbol: Symbol) -> SemanticResult<()> {
        let name = symbol.name.clone();
        let span = symbol.span.clone();

        // Check if symbol already exists in current scope
        if let Some(existing) = self.scopes[self.current_scope].lookup_local(&name) {
            // Allow variable/parameter redefinition (Python allows reassignment)
            // But don't allow function/class/module redefinition
            match existing.kind {
                SymbolKind::Variable | SymbolKind::Parameter => {
                    // Allow redefinition for variables
                }
                SymbolKind::Function | SymbolKind::Class | SymbolKind::Module => {
                    return Err(SemanticError::RedefinedVariable {
                        name,
                        line: span.line,
                        column: span.column,
                        first_line: existing.span.line,
                        span,
                    });
                }
            }
        }

        self.scopes[self.current_scope].define(name, symbol);
        Ok(())
    }

    /// Resolve a symbol by searching current scope and parent scopes
    pub fn resolve_symbol(&self, name: &str) -> Option<&Symbol> {
        let mut current = self.current_scope;

        loop {
            // Check current scope
            if let Some(symbol) = self.scopes[current].lookup_local(name) {
                return Some(symbol);
            }

            // Move to parent scope
            match self.scopes[current].parent() {
                Some(parent) => current = parent,
                None => return None, // Reached global scope, symbol not found
            }
        }
    }

    /// Get the current scope kind
    pub fn current_scope_kind(&self) -> ScopeKind {
        self.scopes[self.current_scope].kind
    }

    /// Check if currently in a function scope (or nested within one)
    pub fn in_function(&self) -> bool {
        let mut current = self.current_scope;

        loop {
            if self.scopes[current].kind == ScopeKind::Function {
                return true;
            }

            match self.scopes[current].parent() {
                Some(parent) => current = parent,
                None => return false,
            }
        }
    }

    /// Check if currently in a loop scope (for break/continue validation)
    /// Returns true if currently inside a loop (for break/continue validation)
    pub fn in_loop(&self) -> bool {
        self.loop_depth > 0
    }

    /// Enter a loop scope (increment loop depth)
    pub fn enter_loop(&mut self) {
        self.loop_depth += 1;
    }

    /// Exit a loop scope (decrement loop depth)
    pub fn exit_loop(&mut self) {
        if self.loop_depth > 0 {
            self.loop_depth -= 1;
        }
    }
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self::new()
    }
}
