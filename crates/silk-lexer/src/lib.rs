pub mod error;
pub mod lexer;
/// Silk programming language lexer
///
/// This module provides lexical analysis (tokenization) for Silk source code.
/// It transforms raw source text into a stream of tokens that can be parsed.
pub mod token;

pub use error::{LexError, LexResult};
pub use lexer::Lexer;
pub use token::{FStringPart, Span, Token, TokenKind};
