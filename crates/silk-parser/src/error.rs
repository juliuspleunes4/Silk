/// Parser error types
use silk_lexer::{LexError, Token};
use thiserror::Error;

pub type ParseResult<T> = Result<T, ParseError>;

#[derive(Error, Debug, Clone)]
pub enum ParseError {
    #[error("Lexer error: {0}")]
    LexError(#[from] LexError),

    #[error("Unexpected token: expected {expected}, found {found:?} - {message}")]
    UnexpectedToken {
        expected: String,
        found: Token,
        message: String,
    },

    #[error("Unexpected end of file")]
    UnexpectedEof,

    #[error("Invalid syntax: {0} at line {1}, column {2}")]
    InvalidSyntax(String, usize, usize),

    #[error("Invalid expression at line {0}, column {1}")]
    InvalidExpression(usize, usize),

    #[error("Invalid statement at line {0}, column {1}")]
    InvalidStatement(usize, usize),

    #[error("Indentation error at line {0}: {1}")]
    IndentationError(usize, String),

    #[error("Invalid pattern at line {0}, column {1}")]
    InvalidPattern(usize, usize),

    #[error("Non-default parameter follows default parameter at line {0}, column {1}")]
    NonDefaultParamAfterDefault(usize, usize),
}
