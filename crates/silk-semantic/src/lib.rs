//! Semantic Analysis for Silk
//!
//! This crate implements semantic analysis including:
//! - Symbol table management
//! - Scope tracking and resolution
//! - Name binding analysis
//! - Type checking and inference
//! - Control flow analysis
//! - Semantic validation

pub mod analyzer;
pub mod control_flow;
pub mod error;
pub mod scope;
pub mod symbol_table;
pub mod types;

pub use analyzer::SemanticAnalyzer;
pub use control_flow::ControlFlowAnalyzer;
pub use error::{SemanticError, SemanticResult};
pub use scope::{Scope, ScopeKind};
pub use symbol_table::{Symbol, SymbolKind, SymbolTable};
pub use types::Type;
