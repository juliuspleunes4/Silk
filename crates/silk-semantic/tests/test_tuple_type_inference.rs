use silk_parser::Parser;
use silk_semantic::{types::Type, SemanticAnalyzer};

#[test]
fn test_empty_tuple() {
    let source = "x = ()";
    let program = Parser::parse(source).unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("x").unwrap();
    assert_eq!(symbol.ty, Type::Tuple(vec![]));
    assert_eq!(symbol.ty.to_string(), "tuple[]");
}

#[test]
fn test_single_element_tuple() {
    let source = "x = (42,)";
    let program = Parser::parse(source).unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("x").unwrap();
    assert_eq!(symbol.ty, Type::Tuple(vec![Type::Int]));
    assert_eq!(symbol.ty.to_string(), "tuple[int]");
}

#[test]
fn test_homogeneous_tuple() {
    let source = "x = (1, 2, 3)";
    let program = Parser::parse(source).unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("x").unwrap();
    assert_eq!(
        symbol.ty,
        Type::Tuple(vec![Type::Int, Type::Int, Type::Int])
    );
    assert_eq!(symbol.ty.to_string(), "tuple[int, int, int]");
}

#[test]
fn test_heterogeneous_tuple() {
    let source = r#"x = (1, "a", 3.0)"#;
    let program = Parser::parse(source).unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("x").unwrap();
    // Tuples preserve individual element types (this is expected!)
    assert_eq!(
        symbol.ty,
        Type::Tuple(vec![Type::Int, Type::Str, Type::Float])
    );
    assert_eq!(symbol.ty.to_string(), "tuple[int, str, float]");
}

#[test]
fn test_tuple_two_elements() {
    let source = r#"x = (42, "hello")"#;
    let program = Parser::parse(source).unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("x").unwrap();
    assert_eq!(symbol.ty, Type::Tuple(vec![Type::Int, Type::Str]));
    assert_eq!(symbol.ty.to_string(), "tuple[int, str]");
}

#[test]
fn test_nested_tuple() {
    let source = "x = ((1, 2), (3, 4))";
    let program = Parser::parse(source).unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("x").unwrap();
    assert_eq!(
        symbol.ty,
        Type::Tuple(vec![
            Type::Tuple(vec![Type::Int, Type::Int]),
            Type::Tuple(vec![Type::Int, Type::Int]),
        ])
    );
    assert_eq!(
        symbol.ty.to_string(),
        "tuple[tuple[int, int], tuple[int, int]]"
    );
}

#[test]
fn test_tuple_with_variables() {
    let source = r#"
a = 42
b = "test"
x = (a, b, 3.14)
"#;
    let program = Parser::parse(source).unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("x").unwrap();
    assert_eq!(
        symbol.ty,
        Type::Tuple(vec![Type::Int, Type::Str, Type::Float])
    );
}

#[test]
fn test_tuple_with_expressions() {
    let source = "x = (1 + 1, 2 * 3, 5 - 1)";
    let program = Parser::parse(source).unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("x").unwrap();
    assert_eq!(
        symbol.ty,
        Type::Tuple(vec![Type::Int, Type::Int, Type::Int])
    );
}

#[test]
fn test_tuple_mixed_collections() {
    let source = "x = ([1, 2], (3, 4))";
    let program = Parser::parse(source).unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("x").unwrap();
    assert_eq!(
        symbol.ty,
        Type::Tuple(vec![
            Type::List(Box::new(Type::Int)),
            Type::Tuple(vec![Type::Int, Type::Int]),
        ])
    );
    assert_eq!(symbol.ty.to_string(), "tuple[list[int], tuple[int, int]]");
}

#[test]
fn test_tuple_with_function_calls() {
    let source = r#"
def get_num() -> int:
    return 42

def get_str() -> str:
    return "hello"

x = (get_num(), get_str())
"#;
    let program = Parser::parse(source).unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("x").unwrap();
    assert_eq!(symbol.ty, Type::Tuple(vec![Type::Int, Type::Str]));
}

#[test]
fn test_tuple_bool_and_none() {
    let source = "x = (True, False, None)";
    let program = Parser::parse(source).unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("x").unwrap();
    assert_eq!(
        symbol.ty,
        Type::Tuple(vec![Type::Bool, Type::Bool, Type::None])
    );
}
