/// Abstract Syntax Tree definitions for Silk
/// 
/// Represents the parsed structure of Silk source code.
/// Each node preserves source location information for error reporting.

pub mod expr;
pub mod stmt;
pub mod types;
pub mod pattern;

pub use expr::*;
pub use stmt::*;
pub use types::*;
pub use pattern::*;

use silk_lexer::Span;

/// A program is a sequence of statements
#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub statements: Vec<Statement>,
    pub span: Span,
}

impl Program {
    pub fn new(statements: Vec<Statement>, span: Span) -> Self {
        Self { statements, span }
    }
}
