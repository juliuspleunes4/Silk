/// Parser for Silk language
///
/// Implements a recursive descent parser that converts tokens into an AST.
pub mod error;
pub mod expr;
pub mod stmt;

pub use error::*;

use silk_ast::Program;
use silk_lexer::{Lexer, Token, TokenKind};

/// Parser state
pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    /// Create a new parser from source code
    pub fn new(source: &str) -> ParseResult<Self> {
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().map_err(ParseError::LexError)?;

        Ok(Self {
            tokens,
            position: 0,
        })
    }

    /// Parse source code into an AST
    pub fn parse(source: &str) -> ParseResult<Program> {
        let mut parser = Self::new(source)?;
        parser.parse_program()
    }

    /// Parse a program (sequence of statements)
    fn parse_program(&mut self) -> ParseResult<Program> {
        let start_span = self.current_token().span.clone();
        let mut statements = Vec::new();

        while !self.is_at_end() {
            // Skip newlines at top level
            if self.check(TokenKind::Newline) {
                self.advance();
                continue;
            }

            statements.push(self.parse_statement()?);
        }

        let end_span = if statements.is_empty() {
            start_span.clone()
        } else {
            statements.last().unwrap().span.clone()
        };

        let span = silk_lexer::Span::new(
            start_span.start,
            end_span.end,
            start_span.line,
            start_span.column,
        );

        Ok(Program::new(statements, span))
    }

    // Helper methods

    fn current_token(&self) -> &Token {
        &self.tokens[self.position]
    }

    #[allow(dead_code)] // TODO: Used for lookahead parsing
    fn peek_token(&self, offset: usize) -> Option<&Token> {
        self.tokens.get(self.position + offset)
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.position += 1;
        }
        &self.tokens[self.position - 1]
    }

    fn check(&self, kind: TokenKind) -> bool {
        !self.is_at_end()
            && std::mem::discriminant(&self.current_token().kind) == std::mem::discriminant(&kind)
    }

    fn is_at_end(&self) -> bool {
        matches!(self.current_token().kind, TokenKind::Eof)
    }

    #[allow(dead_code)] // TODO: Used for statement parsing
    fn skip_newlines(&mut self) {
        while self.check(TokenKind::Newline) {
            self.advance();
        }
    }

    fn expect(&mut self, kind: TokenKind, msg: &str) -> ParseResult<Token> {
        if self.check(kind.clone()) {
            Ok(self.advance().clone())
        } else {
            Err(ParseError::UnexpectedToken {
                expected: format!("{:?}", kind),
                found: self.current_token().clone(),
                message: msg.to_string(),
            })
        }
    }
}
