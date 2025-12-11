use crate::Expression;
/// Type annotation AST nodes
use silk_lexer::Span;

/// Type annotation with source location
#[derive(Debug, Clone, PartialEq)]
pub struct Type {
    pub kind: TypeKind,
    pub span: Span,
}

impl Type {
    pub fn new(kind: TypeKind, span: Span) -> Self {
        Self { kind, span }
    }
}

/// All type kinds in Silk
#[derive(Debug, Clone, PartialEq)]
pub enum TypeKind {
    // Named type (e.g., int, str, MyClass)
    Name(String),

    // Generic type (e.g., List[int], Dict[str, int])
    Generic {
        base: Box<Type>,
        args: Vec<Type>,
    },

    // Tuple type (e.g., tuple[int, str, bool])
    Tuple {
        elements: Vec<Type>,
    },

    // Union type (e.g., int | str or Union[int, str])
    Union {
        types: Vec<Type>,
    },

    // Optional type (e.g., Optional[int] or int | None)
    Optional {
        inner: Box<Type>,
    },

    // Callable type (e.g., Callable[[int, str], bool])
    Callable {
        params: Vec<Type>,
        return_type: Box<Type>,
    },

    // Literal type (e.g., Literal[42, "hello"])
    Literal {
        values: Vec<Expression>,
    },

    // Any type
    Any,

    // None type
    None,
}
