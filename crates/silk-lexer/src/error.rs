/// Error types for lexical analysis

use crate::Span;
use thiserror::Error;

pub type LexResult<T> = Result<T, LexError>;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum LexError {
    #[error("Unexpected character '{0}' at line {1}, column {2}")]
    UnexpectedCharacter(char, usize, usize),
    
    #[error("Unterminated string starting at line {0}, column {1}")]
    UnterminatedString(usize, usize),
    
    #[error("Invalid number format at line {0}, column {1}: {2}")]
    InvalidNumber(usize, usize, String),
    
    #[error("Indentation error at line {0}: {1}")]
    IndentationError(usize, String),
    
    #[error("Invalid escape sequence '\\{0}' at line {1}, column {2}")]
    InvalidEscape(char, usize, usize),
    
    #[error("Invalid Unicode escape at line {0}, column {1}")]
    InvalidUnicodeEscape(usize, usize),
    
    #[error("Unexpected end of file")]
    UnexpectedEof,
}

impl LexError {
    pub fn span(&self) -> Option<Span> {
        match self {
            LexError::UnexpectedCharacter(_, line, col) => {
                Some(Span::new(0, 1, *line, *col))
            }
            LexError::UnterminatedString(line, col) => {
                Some(Span::new(0, 1, *line, *col))
            }
            LexError::InvalidNumber(line, col, _) => {
                Some(Span::new(0, 1, *line, *col))
            }
            LexError::IndentationError(line, _) => {
                Some(Span::new(0, 1, *line, 1))
            }
            LexError::InvalidEscape(_, line, col) => {
                Some(Span::new(0, 1, *line, *col))
            }
            LexError::InvalidUnicodeEscape(line, col) => {
                Some(Span::new(0, 1, *line, *col))
            }
            LexError::UnexpectedEof => std::option::Option::None,
        }
    }
}
