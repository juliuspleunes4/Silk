use silk_parser::Parser;
use silk_semantic::{types::Type, SemanticAnalyzer};

#[test]
fn test_homogeneous_int_list() {
    let source = "x = [1, 2, 3]";
    let program = Parser::parse(source).unwrap();

    let mut analyzer = SemanticAnalyzer::new_without_control_flow();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("x").unwrap();
    assert_eq!(symbol.ty, Type::List(Box::new(Type::Int)));
    assert_eq!(symbol.ty.to_string(), "list[int]");
}

#[test]
fn test_homogeneous_str_list() {
    let source = r#"x = ["a", "b", "c"]"#;
    let program = Parser::parse(source).unwrap();

    let mut analyzer = SemanticAnalyzer::new_without_control_flow();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("x").unwrap();
    assert_eq!(symbol.ty, Type::List(Box::new(Type::Str)));
    assert_eq!(symbol.ty.to_string(), "list[str]");
}

#[test]
fn test_homogeneous_float_list() {
    let source = "x = [1.0, 2.5, 3.14]";
    let program = Parser::parse(source).unwrap();

    let mut analyzer = SemanticAnalyzer::new_without_control_flow();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("x").unwrap();
    assert_eq!(symbol.ty, Type::List(Box::new(Type::Float)));
    assert_eq!(symbol.ty.to_string(), "list[float]");
}

#[test]
fn test_homogeneous_bool_list() {
    let source = "x = [True, False, True]";
    let program = Parser::parse(source).unwrap();

    let mut analyzer = SemanticAnalyzer::new_without_control_flow();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("x").unwrap();
    assert_eq!(symbol.ty, Type::List(Box::new(Type::Bool)));
    assert_eq!(symbol.ty.to_string(), "list[bool]");
}

#[test]
fn test_empty_list() {
    let source = "x = []";
    let program = Parser::parse(source).unwrap();

    let mut analyzer = SemanticAnalyzer::new_without_control_flow();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("x").unwrap();
    assert_eq!(symbol.ty, Type::List(Box::new(Type::Unknown)));
    assert_eq!(symbol.ty.to_string(), "list[<unknown>]");
}

#[test]
fn test_heterogeneous_list() {
    let source = r#"x = [1, "a", 3.0]"#;
    let program = Parser::parse(source).unwrap();

    let mut analyzer = SemanticAnalyzer::new_without_control_flow();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("x").unwrap();
    // Mixed types should return list[Unknown] (no union support yet)
    assert_eq!(symbol.ty, Type::List(Box::new(Type::Unknown)));
    assert_eq!(symbol.ty.to_string(), "list[<unknown>]");
}

#[test]
fn test_nested_list() {
    let source = "x = [[1, 2], [3, 4]]";
    let program = Parser::parse(source).unwrap();

    let mut analyzer = SemanticAnalyzer::new_without_control_flow();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("x").unwrap();
    assert_eq!(
        symbol.ty,
        Type::List(Box::new(Type::List(Box::new(Type::Int))))
    );
    assert_eq!(symbol.ty.to_string(), "list[list[int]]");
}

#[test]
fn test_deeply_nested_list() {
    let source = "x = [[[1, 2], [3, 4]], [[5, 6], [7, 8]]]";
    let program = Parser::parse(source).unwrap();

    let mut analyzer = SemanticAnalyzer::new_without_control_flow();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("x").unwrap();
    assert_eq!(
        symbol.ty,
        Type::List(Box::new(Type::List(Box::new(Type::List(Box::new(
            Type::Int
        ))))))
    );
    assert_eq!(symbol.ty.to_string(), "list[list[list[int]]]");
}

#[test]
fn test_list_with_variables() {
    let source = r#"
a = 42
b = 100
x = [a, b, 7]
"#;
    let program = Parser::parse(source).unwrap();

    let mut analyzer = SemanticAnalyzer::new_without_control_flow();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("x").unwrap();
    assert_eq!(symbol.ty, Type::List(Box::new(Type::Int)));
    assert_eq!(symbol.ty.to_string(), "list[int]");
}

#[test]
fn test_list_with_expressions() {
    let source = "x = [1 + 2, 3 * 4, 5 - 1]";
    let program = Parser::parse(source).unwrap();

    let mut analyzer = SemanticAnalyzer::new_without_control_flow();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("x").unwrap();
    assert_eq!(symbol.ty, Type::List(Box::new(Type::Int)));
    assert_eq!(symbol.ty.to_string(), "list[int]");
}

#[test]
fn test_list_single_element() {
    let source = "x = [42]";
    let program = Parser::parse(source).unwrap();

    let mut analyzer = SemanticAnalyzer::new_without_control_flow();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("x").unwrap();
    assert_eq!(symbol.ty, Type::List(Box::new(Type::Int)));
    assert_eq!(symbol.ty.to_string(), "list[int]");
}

#[test]
fn test_list_with_function_calls() {
    let source = r#"
def get_num() -> int:
    return 42

x = [get_num(), get_num()]
"#;
    let program = Parser::parse(source).unwrap();

    let mut analyzer = SemanticAnalyzer::new_without_control_flow();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("x").unwrap();
    assert_eq!(symbol.ty, Type::List(Box::new(Type::Int)));
    assert_eq!(symbol.ty.to_string(), "list[int]");
}

#[test]
fn test_nested_empty_lists() {
    let source = "x = [[], []]";
    let program = Parser::parse(source).unwrap();

    let mut analyzer = SemanticAnalyzer::new_without_control_flow();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("x").unwrap();
    assert_eq!(
        symbol.ty,
        Type::List(Box::new(Type::List(Box::new(Type::Unknown))))
    );
    assert_eq!(symbol.ty.to_string(), "list[list[<unknown>]]");
}
