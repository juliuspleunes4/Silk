/// Expression AST nodes

use silk_lexer::Span;
use crate::{Type, Pattern};

/// Expression node with source location
#[derive(Debug, Clone, PartialEq)]
pub struct Expression {
    pub kind: ExpressionKind,
    pub span: Span,
}

impl Expression {
    pub fn new(kind: ExpressionKind, span: Span) -> Self {
        Self { kind, span }
    }
}

/// All expression kinds in Silk
#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionKind {
    // Literals
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    None,
    
    // Identifiers
    Identifier(String),
    
    // Binary operations
    BinaryOp {
        left: Box<Expression>,
        op: BinaryOperator,
        right: Box<Expression>,
    },
    
    // Unary operations
    UnaryOp {
        op: UnaryOperator,
        operand: Box<Expression>,
    },
    
    // Comparison (can chain: a < b < c)
    Compare {
        left: Box<Expression>,
        ops: Vec<CompareOperator>,
        comparators: Vec<Expression>,
    },
    
    // Logical operations
    LogicalOp {
        left: Box<Expression>,
        op: LogicalOperator,
        right: Box<Expression>,
    },
    
    // Function call
    Call {
        func: Box<Expression>,
        args: Vec<Expression>,
        keywords: Vec<CallKeyword>,
    },
    
    // Attribute access (obj.attr)
    Attribute {
        value: Box<Expression>,
        attr: String,
    },
    
    // Subscript (obj[index])
    Subscript {
        value: Box<Expression>,
        index: Box<Expression>,
    },
    
    // Slice (start:stop:step)
    Slice {
        lower: Option<Box<Expression>>,
        upper: Option<Box<Expression>>,
        step: Option<Box<Expression>>,
    },
    
    // List literal
    List {
        elements: Vec<Expression>,
    },
    
    // Tuple literal
    Tuple {
        elements: Vec<Expression>,
    },
    
    // Dictionary literal
    Dict {
        keys: Vec<Expression>,
        values: Vec<Expression>,
    },
    
    // Set literal
    Set {
        elements: Vec<Expression>,
    },
    
    // List comprehension
    ListComp {
        element: Box<Expression>,
        generators: Vec<Comprehension>,
    },
    
    // Dictionary comprehension
    DictComp {
        key: Box<Expression>,
        value: Box<Expression>,
        generators: Vec<Comprehension>,
    },
    
    // Set comprehension
    SetComp {
        element: Box<Expression>,
        generators: Vec<Comprehension>,
    },
    
    // Generator expression
    GeneratorExp {
        element: Box<Expression>,
        generators: Vec<Comprehension>,
    },
    
    // Lambda expression
    Lambda {
        params: Vec<Parameter>,
        body: Box<Expression>,
    },
    
    // Conditional expression (ternary)
    IfExp {
        test: Box<Expression>,
        body: Box<Expression>,
        orelse: Box<Expression>,
    },
    
    // Await expression
    Await {
        value: Box<Expression>,
    },
    
    // Yield expression
    Yield {
        value: Option<Box<Expression>>,
    },
    
    // Yield from expression
    YieldFrom {
        value: Box<Expression>,
    },
    
    // Named expression (walrus operator :=)
    NamedExpr {
        target: Box<Expression>,
        value: Box<Expression>,
    },
}

/// Binary operators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOperator {
    Add,        // +
    Sub,        // -
    Mult,       // *
    Div,        // /
    FloorDiv,   // //
    Mod,        // %
    Pow,        // **
    BitOr,      // |
    BitXor,     // ^
    BitAnd,     // &
    LShift,     // <<
    RShift,     // >>
}

/// Unary operators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOperator {
    Not,        // not
    UAdd,       // +
    USub,       // -
    Invert,     // ~
}

/// Comparison operators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompareOperator {
    Eq,         // ==
    NotEq,      // !=
    Lt,         // <
    LtE,        // <=
    Gt,         // >
    GtE,        // >=
    Is,         // is
    IsNot,      // is not
    In,         // in
    NotIn,      // not in
}

/// Logical operators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogicalOperator {
    And,        // and
    Or,         // or
}

/// Keyword argument in function call
#[derive(Debug, Clone, PartialEq)]
pub struct CallKeyword {
    pub arg: Option<String>,  // None for **kwargs
    pub value: Expression,
    pub span: Span,
}

/// Comprehension clause
#[derive(Debug, Clone, PartialEq)]
pub struct Comprehension {
    pub target: Pattern,
    pub iter: Expression,
    pub ifs: Vec<Expression>,
    pub is_async: bool,
}

/// Function parameter
#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    pub name: String,
    pub annotation: Option<Type>,
    pub default: Option<Expression>,
    pub span: Span,
}
