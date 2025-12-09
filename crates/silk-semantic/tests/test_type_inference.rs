//! Tests for type inference from literals

use silk_parser::Parser;
use silk_semantic::{SemanticAnalyzer, Type};

#[test]
fn test_integer_literal_type() {
    let source = "x = 42";
    let program = Parser::parse(source).unwrap();
    
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();
    
    let symbol = analyzer.symbol_table().resolve_symbol("x").unwrap();
    assert_eq!(symbol.ty, Type::Int);
}

#[test]
fn test_float_literal_type() {
    let source = "pi = 3.14";
    let program = Parser::parse(source).unwrap();
    
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();
    
    let symbol = analyzer.symbol_table().resolve_symbol("pi").unwrap();
    assert_eq!(symbol.ty, Type::Float);
}

#[test]
fn test_string_literal_type() {
    let source = "name = 'hello'";
    let program = Parser::parse(source).unwrap();
    
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();
    
    let symbol = analyzer.symbol_table().resolve_symbol("name").unwrap();
    assert_eq!(symbol.ty, Type::Str);
}

#[test]
fn test_raw_string_literal_type() {
    let source = r#"pattern = r'\d+'"#;
    let program = Parser::parse(source).unwrap();
    
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();
    
    let symbol = analyzer.symbol_table().resolve_symbol("pattern").unwrap();
    assert_eq!(symbol.ty, Type::Str);
}

#[test]
fn test_fstring_literal_type() {
    let source = r#"name = 'test'
msg = f'hello {name}'"#;
    let program = Parser::parse(source).unwrap();
    
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();
    
    let symbol = analyzer.symbol_table().resolve_symbol("msg").unwrap();
    assert_eq!(symbol.ty, Type::Str);
}

#[test]
fn test_boolean_true_type() {
    let source = "flag = True";
    let program = Parser::parse(source).unwrap();
    
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();
    
    let symbol = analyzer.symbol_table().resolve_symbol("flag").unwrap();
    assert_eq!(symbol.ty, Type::Bool);
}

#[test]
fn test_boolean_false_type() {
    let source = "flag = False";
    let program = Parser::parse(source).unwrap();
    
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();
    
    let symbol = analyzer.symbol_table().resolve_symbol("flag").unwrap();
    assert_eq!(symbol.ty, Type::Bool);
}

#[test]
fn test_none_literal_type() {
    let source = "result = None";
    let program = Parser::parse(source).unwrap();
    
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();
    
    let symbol = analyzer.symbol_table().resolve_symbol("result").unwrap();
    assert_eq!(symbol.ty, Type::None);
}

#[test]
fn test_multiple_assignments_different_types() {
    let source = r#"
x = 42
y = 3.14
z = 'hello'
flag = True
empty = None
"#;
    let program = Parser::parse(source).unwrap();
    
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();
    
    assert_eq!(analyzer.symbol_table().resolve_symbol("x").unwrap().ty, Type::Int);
    assert_eq!(analyzer.symbol_table().resolve_symbol("y").unwrap().ty, Type::Float);
    assert_eq!(analyzer.symbol_table().resolve_symbol("z").unwrap().ty, Type::Str);
    assert_eq!(analyzer.symbol_table().resolve_symbol("flag").unwrap().ty, Type::Bool);
    assert_eq!(analyzer.symbol_table().resolve_symbol("empty").unwrap().ty, Type::None);
}

#[test]
fn test_walrus_operator_type_inference() {
    let source = "if (x := 42):
    pass";
    let program = Parser::parse(source).unwrap();
    
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();
    
    let symbol = analyzer.symbol_table().resolve_symbol("x").unwrap();
    assert_eq!(symbol.ty, Type::Int);
}

#[test]
fn test_walrus_operator_string_type() {
    let source = "if (name := 'test'):
    pass";
    let program = Parser::parse(source).unwrap();
    
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();
    
    let symbol = analyzer.symbol_table().resolve_symbol("name").unwrap();
    assert_eq!(symbol.ty, Type::Str);
}

#[test]
fn test_variable_reference_preserves_type() {
    let source = r#"
x = 42
y = x
"#;
    let program = Parser::parse(source).unwrap();
    
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();
    
    let x_symbol = analyzer.symbol_table().resolve_symbol("x").unwrap();
    let y_symbol = analyzer.symbol_table().resolve_symbol("y").unwrap();
    assert_eq!(x_symbol.ty, Type::Int);
    assert_eq!(y_symbol.ty, Type::Int);
}

#[test]
fn test_negative_integer_type() {
    let source = "x = -42";
    let program = Parser::parse(source).unwrap();
    
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();
    
    // Negative numbers are parsed as UnaryOp, so they'll be Unknown for now
    // This is expected and will be fixed when we add binary/unary op type inference
    let symbol = analyzer.symbol_table().resolve_symbol("x").unwrap();
    assert_eq!(symbol.ty, Type::Unknown);
}

#[test]
fn test_zero_integer_type() {
    let source = "x = 0";
    let program = Parser::parse(source).unwrap();
    
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();
    
    let symbol = analyzer.symbol_table().resolve_symbol("x").unwrap();
    assert_eq!(symbol.ty, Type::Int);
}

#[test]
fn test_large_integer_type() {
    let source = "x = 999999999999";
    let program = Parser::parse(source).unwrap();
    
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();
    
    let symbol = analyzer.symbol_table().resolve_symbol("x").unwrap();
    assert_eq!(symbol.ty, Type::Int);
}

#[test]
fn test_scientific_notation_float_type() {
    let source = "x = 1.5e10";
    let program = Parser::parse(source).unwrap();
    
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();
    
    let symbol = analyzer.symbol_table().resolve_symbol("x").unwrap();
    assert_eq!(symbol.ty, Type::Float);
}

#[test]
fn test_empty_string_type() {
    let source = "x = ''";
    let program = Parser::parse(source).unwrap();
    
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();
    
    let symbol = analyzer.symbol_table().resolve_symbol("x").unwrap();
    assert_eq!(symbol.ty, Type::Str);
}
