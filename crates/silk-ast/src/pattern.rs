/// Pattern AST nodes for match statements and destructuring

use silk_lexer::Span;
use crate::Expression;

/// Pattern with source location
#[derive(Debug, Clone, PartialEq)]
pub struct Pattern {
    pub kind: PatternKind,
    pub span: Span,
}

impl Pattern {
    pub fn new(kind: PatternKind, span: Span) -> Self {
        Self { kind, span }
    }
}

/// All pattern kinds in Silk
#[derive(Debug, Clone, PartialEq)]
pub enum PatternKind {
    // Match any value and bind to name
    Name(String),
    
    // Literal pattern (match exact value)
    Literal(Expression),
    
    // Wildcard pattern (_)
    Wildcard,
    
    // Sequence pattern ([a, b, c] or (a, b, c))
    Sequence {
        patterns: Vec<Pattern>,
    },
    
    // Mapping pattern ({key: value, ...})
    Mapping {
        keys: Vec<Expression>,
        patterns: Vec<Pattern>,
        rest: Option<String>,  // **rest
    },
    
    // Class pattern (Point(x=1, y=2))
    Class {
        cls: Expression,
        patterns: Vec<Pattern>,
        kwd_patterns: Vec<(String, Pattern)>,
    },
    
    // Or pattern (pattern1 | pattern2)
    Or {
        patterns: Vec<Pattern>,
    },
    
    // As pattern (pattern as name)
    As {
        pattern: Box<Pattern>,
        name: String,
    },
}
