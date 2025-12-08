/// Silk compiler library
/// 
/// Main entry point for the Silk compiler.

pub use silk_lexer::{Lexer, Token, TokenKind, LexError};

pub struct Compiler {
    // TODO: Add parser, semantic analyzer, etc.
}

impl Compiler {
    pub fn new() -> Self {
        Self {}
    }
    
    /// Compile Silk source code to tokens (Phase 1: Lexer only)
    pub fn lex(&self, source: &str) -> Result<Vec<Token>, LexError> {
        let mut lexer = Lexer::new(source);
        lexer.tokenize()
    }
}

impl Default for Compiler {
    fn default() -> Self {
        Self::new()
    }
}
