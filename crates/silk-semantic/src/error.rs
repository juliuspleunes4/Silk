//! Error types for semantic analysis

use silk_lexer::Span;
use thiserror::Error;

/// Result type for semantic analysis operations
pub type SemanticResult<T> = Result<T, SemanticError>;

/// Errors that can occur during semantic analysis
#[derive(Debug, Error, Clone, PartialEq)]
pub enum SemanticError {
    /// Variable used before being defined
    #[error("Undefined variable '{name}' at line {line}, column {column}")]
    UndefinedVariable {
        name: String,
        line: usize,
        column: usize,
        span: Span,
    },

    /// Variable defined more than once in the same scope
    #[error("Variable '{name}' redefined at line {line}, column {column} (first defined at line {first_line})")]
    RedefinedVariable {
        name: String,
        line: usize,
        column: usize,
        first_line: usize,
        span: Span,
    },

    /// Function used before being defined
    #[error("Undefined function '{name}' at line {line}, column {column}")]
    UndefinedFunction {
        name: String,
        line: usize,
        column: usize,
        span: Span,
    },

    /// Class used before being defined
    #[error("Undefined class '{name}' at line {line}, column {column}")]
    UndefinedClass {
        name: String,
        line: usize,
        column: usize,
        span: Span,
    },

    /// Invalid scope operation
    #[error("Invalid scope operation: {message}")]
    InvalidScope {
        message: String,
    },

    /// Type mismatch error (basic, for future type checking)
    #[error("Type mismatch at line {line}, column {column}: {message}")]
    TypeMismatch {
        message: String,
        line: usize,
        column: usize,
        span: Span,
    },

    /// Break statement outside of loop
    #[error("'break' statement outside of loop at line {line}, column {column}")]
    BreakOutsideLoop {
        line: usize,
        column: usize,
        span: Span,
    },

    /// Continue statement outside of loop
    #[error("'continue' statement outside of loop at line {line}, column {column}")]
    ContinueOutsideLoop {
        line: usize,
        column: usize,
        span: Span,
    },

    /// Return statement outside of function
    #[error("'return' statement outside of function at line {line}, column {column}")]
    ReturnOutsideFunction {
        line: usize,
        column: usize,
        span: Span,
    },
}
