/// Silk programming language lexer
/// 
/// This module provides lexical analysis (tokenization) for Silk source code.
/// It transforms raw source text into a stream of tokens that can be parsed.

pub mod token;
pub mod lexer;
pub mod error;

pub use token::{Token, TokenKind, Span};
pub use lexer::Lexer;
pub use error::{LexError, LexResult};
