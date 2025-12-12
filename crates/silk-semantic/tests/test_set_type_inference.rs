use silk_parser::Parser;
use silk_semantic::{types::Type, SemanticAnalyzer};

#[test]
fn test_homogeneous_int_set() {
    let source = "x = {1, 2, 3}";
    let program = Parser::parse(source).unwrap();

    let mut analyzer = SemanticAnalyzer::new_without_control_flow();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("x").unwrap();
    assert_eq!(symbol.ty, Type::Set(Box::new(Type::Int)));
    assert_eq!(symbol.ty.to_string(), "set[int]");
}

#[test]
fn test_homogeneous_str_set() {
    let source = r#"x = {"a", "b", "c"}"#;
    let program = Parser::parse(source).unwrap();

    let mut analyzer = SemanticAnalyzer::new_without_control_flow();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("x").unwrap();
    assert_eq!(symbol.ty, Type::Set(Box::new(Type::Str)));
    assert_eq!(symbol.ty.to_string(), "set[str]");
}

#[test]
fn test_homogeneous_float_set() {
    let source = "x = {1.0, 2.5, 3.14}";
    let program = Parser::parse(source).unwrap();

    let mut analyzer = SemanticAnalyzer::new_without_control_flow();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("x").unwrap();
    assert_eq!(symbol.ty, Type::Set(Box::new(Type::Float)));
    assert_eq!(symbol.ty.to_string(), "set[float]");
}

#[test]
fn test_heterogeneous_set() {
    let source = r#"x = {1, "a", 3.0}"#;
    let program = Parser::parse(source).unwrap();

    let mut analyzer = SemanticAnalyzer::new_without_control_flow();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("x").unwrap();
    // Mixed types should return set[Unknown]
    assert_eq!(symbol.ty, Type::Set(Box::new(Type::Unknown)));
    assert_eq!(symbol.ty.to_string(), "set[<unknown>]");
}

#[test]
fn test_set_single_element() {
    let source = "x = {42}";
    let program = Parser::parse(source).unwrap();

    let mut analyzer = SemanticAnalyzer::new_without_control_flow();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("x").unwrap();
    assert_eq!(symbol.ty, Type::Set(Box::new(Type::Int)));
}

#[test]
fn test_set_with_variables() {
    let source = r#"
a = 1
b = 2
x = {a, b, 3}
"#;
    let program = Parser::parse(source).unwrap();

    let mut analyzer = SemanticAnalyzer::new_without_control_flow();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("x").unwrap();
    assert_eq!(symbol.ty, Type::Set(Box::new(Type::Int)));
}

#[test]
fn test_set_with_expressions() {
    let source = "x = {1 + 1, 2 * 3, 5 - 1}";
    let program = Parser::parse(source).unwrap();

    let mut analyzer = SemanticAnalyzer::new_without_control_flow();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("x").unwrap();
    assert_eq!(symbol.ty, Type::Set(Box::new(Type::Int)));
}

#[test]
fn test_set_bool_elements() {
    let source = "x = {True, False}";
    let program = Parser::parse(source).unwrap();

    let mut analyzer = SemanticAnalyzer::new_without_control_flow();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("x").unwrap();
    assert_eq!(symbol.ty, Type::Set(Box::new(Type::Bool)));
}
