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
    InvalidScope { message: String },

    /// Type mismatch error (basic, for future type checking)
    #[error("Type mismatch at line {line}, column {column}: {message}")]
    TypeMismatch {
        message: String,
        line: usize,
        column: usize,
        span: Span,
    },

    /// Assignment type mismatch - value doesn't match annotated type
    #[error("Type mismatch in assignment at line {line}, column {column}: cannot assign '{value_type}' to variable of type '{expected_type}'")]
    AssignmentTypeMismatch {
        expected_type: String,
        value_type: String,
        line: usize,
        column: usize,
        span: Span,
    },

    /// Function argument type mismatch
    #[error("Type mismatch in function call at line {line}, column {column}: argument {arg_index} has type '{actual_type}' but parameter '{param_name}' expects type '{expected_type}'")]
    ArgumentTypeMismatch {
        param_name: String,
        arg_index: usize,
        expected_type: String,
        actual_type: String,
        line: usize,
        column: usize,
        span: Span,
    },

    /// Return type mismatch
    #[error("Type mismatch in return statement at line {line}, column {column}: returning '{actual_type}' but function expects '{expected_type}'")]
    ReturnTypeMismatch {
        expected_type: String,
        actual_type: String,
        line: usize,
        column: usize,
        span: Span,
    },

    /// Invalid binary operation - operand types incompatible
    #[error("Invalid operation at line {line}, column {column}: cannot apply operator '{operator}' to types '{left_type}' and '{right_type}'")]
    InvalidBinaryOperation {
        operator: String,
        left_type: String,
        right_type: String,
        line: usize,
        column: usize,
        span: Span,
    },

    /// Invalid unary operation - operand type incompatible
    #[error("Invalid operation at line {line}, column {column}: cannot apply operator '{operator}' to type '{operand_type}'")]
    InvalidUnaryOperation {
        operator: String,
        operand_type: String,
        line: usize,
        column: usize,
        span: Span,
    },

    /// Invalid subscript operation - wrong index type or non-subscriptable type
    #[error("Invalid subscript at line {line}, column {column}: cannot index '{collection_type}' with '{index_type}'")]
    InvalidSubscript {
        collection_type: String,
        index_type: String,
        line: usize,
        column: usize,
        span: Span,
    },

    /// Argument count mismatch
    #[error("Argument count mismatch at line {line}, column {column}: function '{function_name}' expects {expected} argument(s) but got {actual}")]
    ArgumentCountMismatch {
        function_name: String,
        expected: usize,
        actual: usize,
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

    // ========== CONTROL FLOW ANALYSIS ERRORS ==========

    /// Unreachable code after return/break/continue/raise
    #[error("Unreachable code at line {line}, column {column}: code after '{statement_type}' will never execute")]
    UnreachableCode {
        statement_type: String,
        line: usize,
        column: usize,
        span: Span,
    },

    /// Variable used before being initialized
    #[error("Variable '{name}' may be used before being initialized at line {line}, column {column}")]
    UninitializedVariable {
        name: String,
        line: usize,
        column: usize,
        span: Span,
    },

    /// Function missing return statement on some paths
    #[error("Function '{function_name}' is missing a return statement on some execution paths (line {line}, column {column})")]
    MissingReturn {
        function_name: String,
        line: usize,
        column: usize,
        span: Span,
    },

    /// Infinite loop detected
    #[error("Infinite loop detected at line {line}, column {column}: loop will never terminate")]
    InfiniteLoop {
        line: usize,
        column: usize,
        span: Span,
    },

    /// Dead code that can never be executed
    #[error("Dead code at line {line}, column {column}: {reason}")]
    DeadCode {
        reason: String,
        line: usize,
        column: usize,
        span: Span,
    },

    /// Variable assigned but never used
    #[error("Unused variable '{name}' at line {line}, column {column}")]
    UnusedVariable {
        name: String,
        line: usize,
        column: usize,
        span: Span,
    },

    /// Function defined but never called
    #[error("Unused function '{name}' at line {line}, column {column}")]
    UnusedFunction {
        name: String,
        line: usize,
        column: usize,
        span: Span,
    },
}
