//! Tests for function call type inference

use silk_parser::Parser;
use silk_semantic::SemanticAnalyzer;
use silk_semantic::types::Type;

#[test]
fn test_call_to_function_with_int_return() {
    let source = r#"
def get_number() -> int:
    return 42

result = get_number()
"#;
    let program = Parser::parse(source).unwrap();
    
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();
    
    // result should have type int
    let symbol = analyzer.symbol_table().resolve_symbol("result").unwrap();
    assert_eq!(symbol.ty, Type::Int);
}

#[test]
fn test_call_to_function_with_str_return() {
    let source = r#"
def get_message() -> str:
    return "hello"

msg = get_message()
"#;
    let program = Parser::parse(source).unwrap();
    
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();
    
    // msg should have type str
    let symbol = analyzer.symbol_table().resolve_symbol("msg").unwrap();
    assert_eq!(symbol.ty, Type::Str);
}

#[test]
fn test_call_to_function_without_return_type() {
    let source = r#"
def some_func():
    pass

x = some_func()
"#;
    let program = Parser::parse(source).unwrap();
    
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();
    
    // x should have Unknown type (no return type annotation)
    let symbol = analyzer.symbol_table().resolve_symbol("x").unwrap();
    assert_eq!(symbol.ty, Type::Unknown);
}

#[test]
fn test_assignment_from_function_call() {
    let source = r#"
def calculate() -> float:
    return 3.14

pi = calculate()
"#;
    let program = Parser::parse(source).unwrap();
    
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();
    
    // pi should have type float
    let symbol = analyzer.symbol_table().resolve_symbol("pi").unwrap();
    assert_eq!(symbol.ty, Type::Float);
}

#[test]
fn test_builtin_len_returns_int() {
    let source = r#"
length = len([1, 2, 3])
"#;
    let program = Parser::parse(source).unwrap();
    
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();
    
    // length should have type int
    let symbol = analyzer.symbol_table().resolve_symbol("length").unwrap();
    assert_eq!(symbol.ty, Type::Int);
}

#[test]
fn test_builtin_str_returns_str() {
    let source = r#"
text = str(123)
"#;
    let program = Parser::parse(source).unwrap();
    
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();
    
    // text should have type str
    let symbol = analyzer.symbol_table().resolve_symbol("text").unwrap();
    assert_eq!(symbol.ty, Type::Str);
}

#[test]
fn test_builtin_print_returns_none() {
    let source = r#"
result = print("hello")
"#;
    let program = Parser::parse(source).unwrap();
    
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();
    
    // result should have type None
    let symbol = analyzer.symbol_table().resolve_symbol("result").unwrap();
    assert_eq!(symbol.ty, Type::None);
}
