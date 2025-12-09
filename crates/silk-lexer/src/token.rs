/// Token types for the Silk programming language
/// 
/// Represents all possible token types in Python/Silk syntax.

use std::fmt;

/// Parts of an f-string: literal text or embedded expression
#[derive(Debug, Clone, PartialEq)]
pub enum FStringPart {
    /// Literal string text
    Text(String),
    /// Expression code to be evaluated (stored as string, parsed later)
    Expression { 
        code: String,
        format_spec: Option<String>,  // e.g., ".2f" in {value:.2f}
    },
}

/// A token with its kind, lexeme, and source location
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub span: Span,
}

/// Source location span (start and end positions)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub line: usize,
    pub column: usize,
}

impl Span {
    pub fn new(start: usize, end: usize, line: usize, column: usize) -> Self {
        Self { start, end, line, column }
    }
}

/// All token kinds in Silk
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Keywords
    And,
    As,
    Assert,
    Async,
    Await,
    Break,
    Class,
    Continue,
    Def,
    Del,
    Elif,
    Else,
    Except,
    False,
    Finally,
    For,
    From,
    Global,
    If,
    Import,
    In,
    Is,
    Lambda,
    None,
    Nonlocal,
    Not,
    Or,
    Pass,
    Raise,
    Return,
    True,
    Try,
    While,
    With,
    Yield,
    Match,
    Case,

    // Identifiers and literals
    Identifier,
    Integer(i64),
    Float(f64),
    String(String),
    RawString(String),  // r"text\n" - escape sequences not processed
    ByteString(Vec<u8>),  // b"bytes" - byte literal
    FString(Vec<FStringPart>),  // f"text {expr} text"
    
    // Operators
    Plus,           // +
    Minus,          // -
    Star,           // *
    Slash,          // /
    DoubleSlash,    // //
    Percent,        // %
    DoubleStar,     // **
    
    Ampersand,      // &
    Pipe,           // |
    Caret,          // ^
    Tilde,          // ~
    LeftShift,      // <<
    RightShift,     // >>
    
    Equal,          // ==
    NotEqual,       // !=
    Less,           // <
    Greater,        // >
    LessEqual,      // <=
    GreaterEqual,   // >=
    
    Assign,         // =
    PlusAssign,     // +=
    MinusAssign,    // -=
    StarAssign,     // *=
    SlashAssign,    // /=
    DoubleSlashAssign, // //=
    PercentAssign,  // %=
    DoubleStarAssign, // **=
    AmpersandAssign, // &=
    PipeAssign,     // |=
    CaretAssign,    // ^=
    LeftShiftAssign, // <<=
    RightShiftAssign, // >>=
    
    // Delimiters
    LeftParen,      // (
    RightParen,     // )
    LeftBracket,    // [
    RightBracket,   // ]
    LeftBrace,      // {
    RightBrace,     // }
    
    Comma,          // ,
    Colon,          // :
    ColonEqual,     // :=
    Semicolon,      // ;
    Dot,            // .
    Arrow,          // ->
    Ellipsis,       // ...
    At,             // @
    
    // Special tokens
    Newline,
    Indent,
    Dedent,
    Eof,
    
    // Comments (usually ignored but tracked for completeness)
    Comment,
}

impl TokenKind {
    /// Check if this token is a keyword
    pub fn is_keyword(&self) -> bool {
        matches!(
            self,
            TokenKind::And
                | TokenKind::As
                | TokenKind::Assert
                | TokenKind::Async
                | TokenKind::Await
                | TokenKind::Break
                | TokenKind::Class
                | TokenKind::Continue
                | TokenKind::Def
                | TokenKind::Del
                | TokenKind::Elif
                | TokenKind::Else
                | TokenKind::Except
                | TokenKind::False
                | TokenKind::Finally
                | TokenKind::For
                | TokenKind::From
                | TokenKind::Global
                | TokenKind::If
                | TokenKind::Import
                | TokenKind::In
                | TokenKind::Is
                | TokenKind::Lambda
                | TokenKind::None
                | TokenKind::Nonlocal
                | TokenKind::Not
                | TokenKind::Or
                | TokenKind::Pass
                | TokenKind::Raise
                | TokenKind::Return
                | TokenKind::True
                | TokenKind::Try
                | TokenKind::While
                | TokenKind::With
                | TokenKind::Yield
                | TokenKind::Match
                | TokenKind::Case
        )
    }
    
    /// Get keyword from string, if it exists
    pub fn keyword(s: &str) -> Option<TokenKind> {
        match s {
            "and" => Some(TokenKind::And),
            "as" => Some(TokenKind::As),
            "assert" => Some(TokenKind::Assert),
            "async" => Some(TokenKind::Async),
            "await" => Some(TokenKind::Await),
            "break" => Some(TokenKind::Break),
            "class" => Some(TokenKind::Class),
            "continue" => Some(TokenKind::Continue),
            "def" => Some(TokenKind::Def),
            "del" => Some(TokenKind::Del),
            "elif" => Some(TokenKind::Elif),
            "else" => Some(TokenKind::Else),
            "except" => Some(TokenKind::Except),
            "False" => Some(TokenKind::False),
            "finally" => Some(TokenKind::Finally),
            "for" => Some(TokenKind::For),
            "from" => Some(TokenKind::From),
            "global" => Some(TokenKind::Global),
            "if" => Some(TokenKind::If),
            "import" => Some(TokenKind::Import),
            "in" => Some(TokenKind::In),
            "is" => Some(TokenKind::Is),
            "lambda" => Some(TokenKind::Lambda),
            "None" => Some(TokenKind::None),
            "nonlocal" => Some(TokenKind::Nonlocal),
            "not" => Some(TokenKind::Not),
            "or" => Some(TokenKind::Or),
            "pass" => Some(TokenKind::Pass),
            "raise" => Some(TokenKind::Raise),
            "return" => Some(TokenKind::Return),
            "True" => Some(TokenKind::True),
            "try" => Some(TokenKind::Try),
            "while" => Some(TokenKind::While),
            "with" => Some(TokenKind::With),
            "yield" => Some(TokenKind::Yield),
            "match" => Some(TokenKind::Match),
            "case" => Some(TokenKind::Case),
            _ => std::option::Option::None,
        }
    }
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenKind::Identifier => write!(f, "identifier"),
            TokenKind::Integer(_) => write!(f, "integer"),
            TokenKind::Float(_) => write!(f, "float"),
            TokenKind::String(_) => write!(f, "string"),
            TokenKind::Eof => write!(f, "end of file"),
            _ => write!(f, "{:?}", self),
        }
    }
}
