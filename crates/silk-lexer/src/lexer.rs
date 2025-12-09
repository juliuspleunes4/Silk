/// Lexer implementation for Silk
/// 
/// Converts source code text into a stream of tokens.

use crate::error::{LexError, LexResult};
use crate::token::{Span, Token, TokenKind};

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
    indent_stack: Vec<usize>,
    at_line_start: bool,
    pending_dedents: usize,
}

impl Lexer {
    /// Create a new lexer from source code
    pub fn new(source: &str) -> Self {
        Self {
            input: source.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
            indent_stack: vec![0], // Start with 0 indentation
            at_line_start: true,
            pending_dedents: 0,
        }
    }
    
    /// Tokenize the entire source
    pub fn tokenize(&mut self) -> LexResult<Vec<Token>> {
        let mut tokens = Vec::new();
        
        loop {
            // Handle pending dedents first
            if self.pending_dedents > 0 {
                self.pending_dedents -= 1;
                let token = Token {
                    kind: TokenKind::Dedent,
                    lexeme: String::new(),
                    span: Span::new(self.position, self.position, self.line, self.column),
                };
                tokens.push(token);
                continue;
            }
            
            let token = self.next_token()?;
            let is_eof = token.kind == TokenKind::Eof;
            
            // Don't add EOF yet if we have pending dedents
            if is_eof {
                // Generate dedents for remaining indentation levels
                while self.indent_stack.len() > 1 {
                    self.indent_stack.pop();
                    tokens.push(Token {
                        kind: TokenKind::Dedent,
                        lexeme: String::new(),
                        span: Span::new(self.position, self.position, self.line, self.column),
                    });
                }
            }
            
            tokens.push(token);
            
            if is_eof {
                break;
            }
        }
        
        Ok(tokens)
    }
    
    /// Get the next token
    pub fn next_token(&mut self) -> LexResult<Token> {
        // Handle indentation at line start
        if self.at_line_start {
            return self.handle_indentation();
        }
        
        self.skip_whitespace_inline();
        
        if self.is_at_end() {
            return Ok(self.make_token(TokenKind::Eof, ""));
        }
        
        let start_pos = self.position;
        let start_line = self.line;
        let start_col = self.column;
        
        let ch = self.current_char();
        
        // Handle newlines
        if ch == '\n' {
            self.advance();
            self.at_line_start = true;
            return Ok(Token {
                kind: TokenKind::Newline,
                lexeme: "\n".to_string(),
                span: Span::new(start_pos, self.position, start_line, start_col),
            });
        }
        
        // Handle comments
        if ch == '#' {
            return self.lex_comment();
        }
        
        // Handle identifiers and keywords
        if ch.is_alphabetic() || ch == '_' {
            return self.lex_identifier();
        }
        
        // Handle numbers
        if ch.is_ascii_digit() {
            return self.lex_number();
        }
        
        // Handle strings
        if ch == '"' || ch == '\'' {
            return self.lex_string();
        }
        
        // Handle operators and delimiters
        self.lex_operator_or_delimiter()
    }
    
    // Helper methods
    
    fn current_char(&self) -> char {
        self.input[self.position]
    }
    
    fn peek_char(&self, offset: usize) -> Option<char> {
        let pos = self.position + offset;
        if pos < self.input.len() {
            Some(self.input[pos])
        } else {
            std::option::Option::None
        }
    }
    
    fn advance(&mut self) -> char {
        let ch = self.input[self.position];
        self.position += 1;
        
        if ch == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        
        ch
    }
    
    fn is_at_end(&self) -> bool {
        self.position >= self.input.len()
    }
    
    fn skip_whitespace_inline(&mut self) {
        while !self.is_at_end() {
            let ch = self.current_char();
            if ch == ' ' || ch == '\t' || ch == '\r' {
                self.advance();
            } else {
                break;
            }
        }
    }
    
    fn handle_indentation(&mut self) -> LexResult<Token> {
        self.at_line_start = false;
        
        let start_pos = self.position;
        let start_line = self.line;
        let start_col = self.column;
        
        // Skip blank lines and comments
        while !self.is_at_end() {
            let ch = self.current_char();
            
            // Skip whitespace
            if ch == ' ' || ch == '\t' {
                self.advance();
                continue;
            }
            
            // Empty line - skip it
            if ch == '\n' {
                self.advance();
                self.at_line_start = true;
                return Ok(Token {
                    kind: TokenKind::Newline,
                    lexeme: "\n".to_string(),
                    span: Span::new(start_pos, self.position, start_line, start_col),
                });
            }
            
            // Comment line - lex the comment but stay at line start
            if ch == '#' {
                let comment = self.lex_comment()?;
                return Ok(comment);
            }
            
            break;
        }
        
        // End of file
        if self.is_at_end() {
            return Ok(self.make_token(TokenKind::Eof, ""));
        }
        
        // Calculate indentation level (column - 1 because column starts at 1)
        let indent_level = self.column - 1;
        let current_indent = *self.indent_stack.last().unwrap();
        
        if indent_level > current_indent {
            // Indentation increased - generate INDENT
            self.indent_stack.push(indent_level);
            return Ok(Token {
                kind: TokenKind::Indent,
                lexeme: String::new(),
                span: Span::new(start_pos, self.position, start_line, start_col),
            });
        } else if indent_level < current_indent {
            // Indentation decreased - generate DEDENT(s)
            let mut dedent_count = 0;
            
            while let Some(&stack_indent) = self.indent_stack.last() {
                if stack_indent <= indent_level {
                    break;
                }
                self.indent_stack.pop();
                dedent_count += 1;
            }
            
            // Check if indentation matches a level in the stack
            if self.indent_stack.last() != Some(&indent_level) {
                return Err(LexError::IndentationError(
                    start_line,
                    format!("Inconsistent indentation at column {}", indent_level),
                ));
            }
            
            // Queue up dedents
            if dedent_count > 0 {
                self.pending_dedents = dedent_count - 1;
                return Ok(Token {
                    kind: TokenKind::Dedent,
                    lexeme: String::new(),
                    span: Span::new(start_pos, self.position, start_line, start_col),
                });
            }
        }
        
        // Same indentation - continue normally
        self.skip_whitespace_inline();
        
        if self.is_at_end() {
            return Ok(self.make_token(TokenKind::Eof, ""));
        }
        
        let ch = self.current_char();
        
        // Handle comments
        if ch == '#' {
            return self.lex_comment();
        }
        
        // Handle identifiers and keywords
        if ch.is_alphabetic() || ch == '_' {
            return self.lex_identifier();
        }
        
        // Handle numbers
        if ch.is_ascii_digit() {
            return self.lex_number();
        }
        
        // Handle strings
        if ch == '"' || ch == '\'' {
            return self.lex_string();
        }
        
        // Handle newlines
        if ch == '\n' {
            let start_pos = self.position;
            let start_line = self.line;
            let start_col = self.column;
            self.advance();
            self.at_line_start = true;
            return Ok(Token {
                kind: TokenKind::Newline,
                lexeme: "\n".to_string(),
                span: Span::new(start_pos, self.position, start_line, start_col),
            });
        }
        
        // Handle operators and delimiters
        self.lex_operator_or_delimiter()
    }
    
    fn make_token(&self, kind: TokenKind, lexeme: &str) -> Token {
        Token {
            kind,
            lexeme: lexeme.to_string(),
            span: Span::new(
                self.position - lexeme.len(),
                self.position,
                self.line,
                self.column - lexeme.len(),
            ),
        }
    }
    
    fn lex_comment(&mut self) -> LexResult<Token> {
        let start_pos = self.position;
        let start_col = self.column;
        
        // Skip '#'
        self.advance();
        
        // Read until end of line
        while !self.is_at_end() && self.current_char() != '\n' {
            self.advance();
        }
        
        let lexeme: String = self.input[start_pos..self.position].iter().collect();
        
        Ok(Token {
            kind: TokenKind::Comment,
            lexeme,
            span: Span::new(start_pos, self.position, self.line, start_col),
        })
    }
    
    fn lex_identifier(&mut self) -> LexResult<Token> {
        let start_pos = self.position;
        let start_col = self.column;
        
        while !self.is_at_end() {
            let ch = self.current_char();
            if ch.is_alphanumeric() || ch == '_' {
                self.advance();
            } else {
                break;
            }
        }
        
        let lexeme: String = self.input[start_pos..self.position].iter().collect();
        
        // Check if it's a keyword
        let kind = TokenKind::keyword(&lexeme).unwrap_or(TokenKind::Identifier);
        
        Ok(Token {
            kind,
            lexeme,
            span: Span::new(start_pos, self.position, self.line, start_col),
        })
    }
    
    fn lex_number(&mut self) -> LexResult<Token> {
        let start_pos = self.position;
        let start_col = self.column;
        let start_line = self.line;
        
        // TODO: Handle binary (0b), octal (0o), hex (0x) prefixes
        // For now, just handle decimal integers and floats
        
        let mut is_float = false;
        
        // Read digits
        while !self.is_at_end() && self.current_char().is_ascii_digit() {
            self.advance();
        }
        
        // Check for decimal point
        if !self.is_at_end() && self.current_char() == '.' {
            if let Some(next) = self.peek_char(1) {
                if next.is_ascii_digit() {
                    is_float = true;
                    self.advance(); // consume '.'
                    
                    while !self.is_at_end() && self.current_char().is_ascii_digit() {
                        self.advance();
                    }
                }
            }
        }
        
        // Check for scientific notation (e or E)
        if !self.is_at_end() {
            let ch = self.current_char();
            if ch == 'e' || ch == 'E' {
                is_float = true;
                self.advance();
                
                // Optional sign
                if !self.is_at_end() {
                    let sign = self.current_char();
                    if sign == '+' || sign == '-' {
                        self.advance();
                    }
                }
                
                // Exponent digits
                while !self.is_at_end() && self.current_char().is_ascii_digit() {
                    self.advance();
                }
            }
        }
        
        let lexeme: String = self.input[start_pos..self.position].iter().collect();
        
        let kind = if is_float {
            match lexeme.parse::<f64>() {
                Ok(val) => TokenKind::Float(val),
                Err(_) => {
                    return Err(LexError::InvalidNumber(
                        start_line,
                        start_col,
                        lexeme,
                    ));
                }
            }
        } else {
            match lexeme.parse::<i64>() {
                Ok(val) => TokenKind::Integer(val),
                Err(_) => {
                    return Err(LexError::InvalidNumber(
                        start_line,
                        start_col,
                        lexeme,
                    ));
                }
            }
        };
        
        Ok(Token {
            kind,
            lexeme,
            span: Span::new(start_pos, self.position, start_line, start_col),
        })
    }
    
    fn lex_string(&mut self) -> LexResult<Token> {
        let start_pos = self.position;
        let start_col = self.column;
        let start_line = self.line;
        
        let quote = self.advance(); // Consume opening quote
        
        // Check for triple-quoted strings
        let is_triple = if self.peek_char(0) == Some(quote) && self.peek_char(1) == Some(quote) {
            self.advance();
            self.advance();
            true
        } else {
            false
        };
        
        let mut value = String::new();
        
        loop {
            if self.is_at_end() {
                return Err(LexError::UnterminatedString(start_line, start_col));
            }
            
            let ch = self.current_char();
            
            // Check for closing quote(s)
            if ch == quote {
                if is_triple {
                    if self.peek_char(1) == Some(quote) && self.peek_char(2) == Some(quote) {
                        self.advance();
                        self.advance();
                        self.advance();
                        break;
                    } else {
                        value.push(self.advance());
                    }
                } else {
                    self.advance();
                    break;
                }
            } else if ch == '\\' && !is_triple {
                // Handle escape sequences
                self.advance();
                if self.is_at_end() {
                    return Err(LexError::UnterminatedString(start_line, start_col));
                }
                
                let escaped = self.advance();
                let escaped_char = match escaped {
                    'n' => '\n',
                    'r' => '\r',
                    't' => '\t',
                    '\\' => '\\',
                    '\'' => '\'',
                    '"' => '"',
                    '0' => '\0',
                    _ => {
                        return Err(LexError::InvalidEscape(escaped, self.line, self.column));
                    }
                };
                value.push(escaped_char);
            } else if ch == '\n' && !is_triple {
                return Err(LexError::UnterminatedString(start_line, start_col));
            } else {
                value.push(self.advance());
            }
        }
        
        let lexeme: String = self.input[start_pos..self.position].iter().collect();
        
        Ok(Token {
            kind: TokenKind::String(value),
            lexeme,
            span: Span::new(start_pos, self.position, start_line, start_col),
        })
    }
    
    fn lex_operator_or_delimiter(&mut self) -> LexResult<Token> {
        let start_pos = self.position;
        let start_col = self.column;
        
        let ch = self.advance();
        
        let kind = match ch {
            '(' => TokenKind::LeftParen,
            ')' => TokenKind::RightParen,
            '[' => TokenKind::LeftBracket,
            ']' => TokenKind::RightBracket,
            '{' => TokenKind::LeftBrace,
            '}' => TokenKind::RightBrace,
            ',' => TokenKind::Comma,
            ';' => TokenKind::Semicolon,
            ':' => {
                if self.peek_char(0) == Some('=') {
                    self.advance();
                    TokenKind::ColonEqual
                } else {
                    TokenKind::Colon
                }
            }
            '~' => TokenKind::Tilde,
            
            '+' => {
                if self.peek_char(0) == Some('=') {
                    self.advance();
                    TokenKind::PlusAssign
                } else {
                    TokenKind::Plus
                }
            }
            
            '-' => {
                if self.peek_char(0) == Some('=') {
                    self.advance();
                    TokenKind::MinusAssign
                } else if self.peek_char(0) == Some('>') {
                    self.advance();
                    TokenKind::Arrow
                } else {
                    TokenKind::Minus
                }
            }
            
            '*' => {
                if self.peek_char(0) == Some('*') {
                    self.advance();
                    if self.peek_char(0) == Some('=') {
                        self.advance();
                        TokenKind::DoubleStarAssign
                    } else {
                        TokenKind::DoubleStar
                    }
                } else if self.peek_char(0) == Some('=') {
                    self.advance();
                    TokenKind::StarAssign
                } else {
                    TokenKind::Star
                }
            }
            
            '/' => {
                if self.peek_char(0) == Some('/') {
                    self.advance();
                    if self.peek_char(0) == Some('=') {
                        self.advance();
                        TokenKind::DoubleSlashAssign
                    } else {
                        TokenKind::DoubleSlash
                    }
                } else if self.peek_char(0) == Some('=') {
                    self.advance();
                    TokenKind::SlashAssign
                } else {
                    TokenKind::Slash
                }
            }
            
            '%' => {
                if self.peek_char(0) == Some('=') {
                    self.advance();
                    TokenKind::PercentAssign
                } else {
                    TokenKind::Percent
                }
            }
            
            '&' => {
                if self.peek_char(0) == Some('=') {
                    self.advance();
                    TokenKind::AmpersandAssign
                } else {
                    TokenKind::Ampersand
                }
            }
            
            '|' => {
                if self.peek_char(0) == Some('=') {
                    self.advance();
                    TokenKind::PipeAssign
                } else {
                    TokenKind::Pipe
                }
            }
            
            '^' => {
                if self.peek_char(0) == Some('=') {
                    self.advance();
                    TokenKind::CaretAssign
                } else {
                    TokenKind::Caret
                }
            }
            
            '<' => {
                if self.peek_char(0) == Some('<') {
                    self.advance();
                    if self.peek_char(0) == Some('=') {
                        self.advance();
                        TokenKind::LeftShiftAssign
                    } else {
                        TokenKind::LeftShift
                    }
                } else if self.peek_char(0) == Some('=') {
                    self.advance();
                    TokenKind::LessEqual
                } else {
                    TokenKind::Less
                }
            }
            
            '>' => {
                if self.peek_char(0) == Some('>') {
                    self.advance();
                    if self.peek_char(0) == Some('=') {
                        self.advance();
                        TokenKind::RightShiftAssign
                    } else {
                        TokenKind::RightShift
                    }
                } else if self.peek_char(0) == Some('=') {
                    self.advance();
                    TokenKind::GreaterEqual
                } else {
                    TokenKind::Greater
                }
            }
            
            '=' => {
                if self.peek_char(0) == Some('=') {
                    self.advance();
                    TokenKind::Equal
                } else {
                    TokenKind::Assign
                }
            }
            
            '!' => {
                if self.peek_char(0) == Some('=') {
                    self.advance();
                    TokenKind::NotEqual
                } else {
                    return Err(LexError::UnexpectedCharacter(ch, self.line, start_col));
                }
            }
            
            '.' => {
                if self.peek_char(0) == Some('.') && self.peek_char(1) == Some('.') {
                    self.advance();
                    self.advance();
                    TokenKind::Ellipsis
                } else {
                    TokenKind::Dot
                }
            }
            
            '@' => TokenKind::At,
            
            _ => {
                return Err(LexError::UnexpectedCharacter(ch, self.line, start_col));
            }
        };
        
        let lexeme: String = self.input[start_pos..self.position].iter().collect();
        
        Ok(Token {
            kind,
            lexeme,
            span: Span::new(start_pos, self.position, self.line, start_col),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_keywords() {
        let source = "def if else while for";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens[0].kind, TokenKind::Def);
        assert_eq!(tokens[1].kind, TokenKind::If);
        assert_eq!(tokens[2].kind, TokenKind::Else);
        assert_eq!(tokens[3].kind, TokenKind::While);
        assert_eq!(tokens[4].kind, TokenKind::For);
    }
    
    #[test]
    fn test_identifiers() {
        let source = "foo bar_baz test123";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens[0].kind, TokenKind::Identifier);
        assert_eq!(tokens[0].lexeme, "foo");
        assert_eq!(tokens[1].lexeme, "bar_baz");
        assert_eq!(tokens[2].lexeme, "test123");
    }
    
    #[test]
    fn test_integers() {
        let source = "42 0 123456";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens[0].kind, TokenKind::Integer(42));
        assert_eq!(tokens[1].kind, TokenKind::Integer(0));
        assert_eq!(tokens[2].kind, TokenKind::Integer(123456));
    }
    
    #[test]
    fn test_floats() {
        let source = "3.14 0.5 1e10 2.5e-3";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens[0].kind, TokenKind::Float(3.14));
        assert_eq!(tokens[1].kind, TokenKind::Float(0.5));
    }
    
    #[test]
    fn test_strings() {
        let source = r#""hello" 'world' "with\nnewline""#;
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        
        assert!(matches!(tokens[0].kind, TokenKind::String(_)));
        if let TokenKind::String(ref s) = tokens[0].kind {
            assert_eq!(s, "hello");
        }
    }
    
    #[test]
    fn test_operators() {
        let source = "+ - * / ** // % == != < > <= >= += ->";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens[0].kind, TokenKind::Plus);
        assert_eq!(tokens[1].kind, TokenKind::Minus);
        assert_eq!(tokens[2].kind, TokenKind::Star);
        assert_eq!(tokens[3].kind, TokenKind::Slash);
        assert_eq!(tokens[4].kind, TokenKind::DoubleStar);
        assert_eq!(tokens[5].kind, TokenKind::DoubleSlash);
    }
    
    #[test]
    fn test_delimiters() {
        let source = "( ) [ ] { } , : . ...";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens[0].kind, TokenKind::LeftParen);
        assert_eq!(tokens[1].kind, TokenKind::RightParen);
        assert_eq!(tokens[2].kind, TokenKind::LeftBracket);
        assert_eq!(tokens[3].kind, TokenKind::RightBracket);
    }
    
    #[test]
    fn test_comments() {
        let source = "x = 1 # this is a comment\ny = 2";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        
        // Should have: x, =, 1, comment, newline, y, =, 2, eof
        assert_eq!(tokens[0].kind, TokenKind::Identifier);
        assert_eq!(tokens[1].kind, TokenKind::Assign);
        assert_eq!(tokens[2].kind, TokenKind::Integer(1));
        assert_eq!(tokens[3].kind, TokenKind::Comment);
    }
    
    #[test]
    fn test_indentation_simple() {
        let source = "def foo():\n    return 42";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        
        // Find INDENT and DEDENT tokens
        let indent_found = tokens.iter().any(|t| t.kind == TokenKind::Indent);
        let dedent_found = tokens.iter().any(|t| t.kind == TokenKind::Dedent);
        
        assert!(indent_found, "Should have INDENT token");
        assert!(dedent_found, "Should have DEDENT token");
    }
    
    #[test]
    fn test_indentation_nested() {
        let source = "def outer():\n    if True:\n        x = 1\n    y = 2";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        
        let indent_count = tokens.iter().filter(|t| t.kind == TokenKind::Indent).count();
        let dedent_count = tokens.iter().filter(|t| t.kind == TokenKind::Dedent).count();
        
        assert_eq!(indent_count, 2, "Should have 2 INDENT tokens");
        assert_eq!(dedent_count, 2, "Should have 2 DEDENT tokens");
    }
    
    #[test]
    fn test_indentation_multiple_dedents() {
        let source = "def foo():\n    if True:\n        x = 1\ny = 2";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        
        let dedent_count = tokens.iter().filter(|t| t.kind == TokenKind::Dedent).count();
        
        assert_eq!(dedent_count, 2, "Should have 2 DEDENT tokens when going from nested to top level");
    }
}

