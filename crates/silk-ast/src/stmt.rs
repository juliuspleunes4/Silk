use crate::{Expression, Pattern, Type};
/// Statement AST nodes
use silk_lexer::Span;

/// Statement node with source location
#[derive(Debug, Clone, PartialEq)]
pub struct Statement {
    pub kind: StatementKind,
    pub span: Span,
}

impl Statement {
    pub fn new(kind: StatementKind, span: Span) -> Self {
        Self { kind, span }
    }
}

/// All statement kinds in Silk
#[derive(Debug, Clone, PartialEq)]
pub enum StatementKind {
    // Expression statement
    Expr(Expression),

    // Assignment
    Assign {
        targets: Vec<Expression>,
        value: Expression,
        type_annotation: Option<Type>,
    },

    // Augmented assignment (+=, -=, etc.)
    AugAssign {
        target: Expression,
        op: AugAssignOperator,
        value: Expression,
    },

    // Annotated assignment (x: int = 5)
    AnnAssign {
        target: Expression,
        annotation: Type,
        value: Option<Expression>,
    },

    // Assert statement
    Assert {
        test: Expression,
        msg: Option<Expression>,
    },

    // Pass statement
    Pass,

    // Delete statement
    Delete {
        targets: Vec<Expression>,
    },

    // Return statement
    Return {
        value: Option<Expression>,
    },

    // Raise statement
    Raise {
        exc: Option<Expression>,
        cause: Option<Expression>,
    },

    // Break statement
    Break,

    // Continue statement
    Continue,

    // Import statement
    Import {
        names: Vec<Alias>,
    },

    // Import from statement
    ImportFrom {
        module: Option<String>,
        names: Vec<Alias>,
        level: usize, // Relative import level (number of dots)
    },

    // Global statement
    Global {
        names: Vec<String>,
    },

    // Nonlocal statement
    Nonlocal {
        names: Vec<String>,
    },

    // If statement
    If {
        test: Expression,
        body: Vec<Statement>,
        orelse: Vec<Statement>,
    },

    // While loop
    While {
        test: Expression,
        body: Vec<Statement>,
        orelse: Vec<Statement>,
    },

    // For loop
    For {
        target: Pattern,
        iter: Expression,
        body: Vec<Statement>,
        orelse: Vec<Statement>,
        is_async: bool,
    },

    // With statement
    With {
        items: Vec<WithItem>,
        body: Vec<Statement>,
        is_async: bool,
    },

    // Match statement
    Match {
        subject: Expression,
        cases: Vec<MatchCase>,
    },

    // Try/except statement
    Try {
        body: Vec<Statement>,
        handlers: Vec<ExceptHandler>,
        orelse: Vec<Statement>,
        finalbody: Vec<Statement>,
    },

    // Function definition
    FunctionDef {
        name: String,
        params: FunctionParams,
        body: Vec<Statement>,
        decorator_list: Vec<Expression>,
        returns: Option<Type>,
        is_async: bool,
    },

    // Class definition
    ClassDef {
        name: String,
        bases: Vec<Expression>,
        keywords: Vec<Keyword>,
        body: Vec<Statement>,
        decorator_list: Vec<Expression>,
    },
}

/// Augmented assignment operators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AugAssignOperator {
    Add,      // +=
    Sub,      // -=
    Mult,     // *=
    Div,      // /=
    FloorDiv, // //=
    Mod,      // %=
    Pow,      // **=
    BitOr,    // |=
    BitXor,   // ^=
    BitAnd,   // &=
    LShift,   // <<=
    RShift,   // >>=
}

/// Import alias (name as asname)
#[derive(Debug, Clone, PartialEq)]
pub struct Alias {
    pub name: String,
    pub asname: Option<String>,
    pub span: Span,
}

/// With item
#[derive(Debug, Clone, PartialEq)]
pub struct WithItem {
    pub context_expr: Expression,
    pub optional_vars: Option<Expression>,
    pub span: Span,
}

/// Match case
#[derive(Debug, Clone, PartialEq)]
pub struct MatchCase {
    pub pattern: Pattern,
    pub guard: Option<Expression>,
    pub body: Vec<Statement>,
    pub span: Span,
}

/// Exception handler
#[derive(Debug, Clone, PartialEq)]
pub struct ExceptHandler {
    pub typ: Option<Expression>,
    pub name: Option<String>,
    pub body: Vec<Statement>,
    pub span: Span,
}

/// Function parameters
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionParams {
    pub args: Vec<FunctionArg>,
    pub vararg: Option<FunctionArg>,
    pub kwonlyargs: Vec<FunctionArg>,
    pub kwarg: Option<FunctionArg>,
}

/// Function argument
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionArg {
    pub name: String,
    pub annotation: Option<Type>,
    pub default: Option<Expression>,
    pub span: Span,
}

/// Keyword argument in class definition
#[derive(Debug, Clone, PartialEq)]
pub struct Keyword {
    pub arg: Option<String>,
    pub value: Expression,
    pub span: Span,
}
