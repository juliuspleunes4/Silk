use silk_parser::Parser;
use silk_semantic::{types::Type, SemanticAnalyzer};

#[test]
fn test_homogeneous_int_str_dict() {
    let source = r#"x = {1: "a", 2: "b", 3: "c"}"#;
    let program = Parser::parse(source).unwrap();

    let mut analyzer = SemanticAnalyzer::new_without_control_flow();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("x").unwrap();
    assert_eq!(
        symbol.ty,
        Type::Dict {
            key_type: Box::new(Type::Int),
            value_type: Box::new(Type::Str),
        }
    );
    assert_eq!(symbol.ty.to_string(), "dict[int, str]");
}

#[test]
fn test_homogeneous_str_int_dict() {
    let source = r#"x = {"a": 1, "b": 2, "c": 3}"#;
    let program = Parser::parse(source).unwrap();

    let mut analyzer = SemanticAnalyzer::new_without_control_flow();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("x").unwrap();
    assert_eq!(
        symbol.ty,
        Type::Dict {
            key_type: Box::new(Type::Str),
            value_type: Box::new(Type::Int),
        }
    );
    assert_eq!(symbol.ty.to_string(), "dict[str, int]");
}

#[test]
fn test_empty_dict() {
    let source = "x = {}";
    let program = Parser::parse(source).unwrap();

    let mut analyzer = SemanticAnalyzer::new_without_control_flow();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("x").unwrap();
    assert_eq!(
        symbol.ty,
        Type::Dict {
            key_type: Box::new(Type::Unknown),
            value_type: Box::new(Type::Unknown),
        }
    );
    assert_eq!(symbol.ty.to_string(), "dict[<unknown>, <unknown>]");
}

#[test]
fn test_heterogeneous_keys() {
    let source = r#"x = {1: "a", "b": "c"}"#;
    let program = Parser::parse(source).unwrap();

    let mut analyzer = SemanticAnalyzer::new_without_control_flow();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("x").unwrap();
    // Mixed key types should return dict[Unknown, str]
    assert_eq!(
        symbol.ty,
        Type::Dict {
            key_type: Box::new(Type::Unknown),
            value_type: Box::new(Type::Str),
        }
    );
    assert_eq!(symbol.ty.to_string(), "dict[<unknown>, str]");
}

#[test]
fn test_heterogeneous_values() {
    let source = r#"x = {"a": 1, "b": "c"}"#;
    let program = Parser::parse(source).unwrap();

    let mut analyzer = SemanticAnalyzer::new_without_control_flow();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("x").unwrap();
    // Mixed value types should return dict[str, Unknown]
    assert_eq!(
        symbol.ty,
        Type::Dict {
            key_type: Box::new(Type::Str),
            value_type: Box::new(Type::Unknown),
        }
    );
    assert_eq!(symbol.ty.to_string(), "dict[str, <unknown>]");
}

#[test]
fn test_heterogeneous_both() {
    let source = r#"x = {1: "a", "b": 2}"#;
    let program = Parser::parse(source).unwrap();

    let mut analyzer = SemanticAnalyzer::new_without_control_flow();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("x").unwrap();
    // Mixed keys and values should return dict[Unknown, Unknown]
    assert_eq!(
        symbol.ty,
        Type::Dict {
            key_type: Box::new(Type::Unknown),
            value_type: Box::new(Type::Unknown),
        }
    );
    assert_eq!(symbol.ty.to_string(), "dict[<unknown>, <unknown>]");
}

#[test]
fn test_nested_dict_values() {
    let source = r#"x = {"a": {1: "x"}, "b": {2: "y"}}"#;
    let program = Parser::parse(source).unwrap();

    let mut analyzer = SemanticAnalyzer::new_without_control_flow();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("x").unwrap();
    assert_eq!(
        symbol.ty,
        Type::Dict {
            key_type: Box::new(Type::Str),
            value_type: Box::new(Type::Dict {
                key_type: Box::new(Type::Int),
                value_type: Box::new(Type::Str),
            }),
        }
    );
    assert_eq!(symbol.ty.to_string(), "dict[str, dict[int, str]]");
}

#[test]
fn test_dict_with_variables() {
    let source = r#"
a = "key1"
b = "key2"
x = {a: 1, b: 2}
"#;
    let program = Parser::parse(source).unwrap();

    let mut analyzer = SemanticAnalyzer::new_without_control_flow();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("x").unwrap();
    assert_eq!(
        symbol.ty,
        Type::Dict {
            key_type: Box::new(Type::Str),
            value_type: Box::new(Type::Int),
        }
    );
}

#[test]
fn test_dict_with_expressions() {
    let source = r#"x = {1 + 1: 2 * 3, 5 - 2: 7 + 1}"#;
    let program = Parser::parse(source).unwrap();

    let mut analyzer = SemanticAnalyzer::new_without_control_flow();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("x").unwrap();
    assert_eq!(
        symbol.ty,
        Type::Dict {
            key_type: Box::new(Type::Int),
            value_type: Box::new(Type::Int),
        }
    );
}

#[test]
fn test_dict_single_entry() {
    let source = r#"x = {"key": 42}"#;
    let program = Parser::parse(source).unwrap();

    let mut analyzer = SemanticAnalyzer::new_without_control_flow();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("x").unwrap();
    assert_eq!(
        symbol.ty,
        Type::Dict {
            key_type: Box::new(Type::Str),
            value_type: Box::new(Type::Int),
        }
    );
}

#[test]
fn test_dict_bool_keys() {
    let source = "x = {True: 1, False: 2}";
    let program = Parser::parse(source).unwrap();

    let mut analyzer = SemanticAnalyzer::new_without_control_flow();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("x").unwrap();
    assert_eq!(
        symbol.ty,
        Type::Dict {
            key_type: Box::new(Type::Bool),
            value_type: Box::new(Type::Int),
        }
    );
}
