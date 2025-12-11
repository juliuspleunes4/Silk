//! Tests for function type storage in the symbol table

use silk_parser::Parser;
use silk_semantic::SemanticAnalyzer;
use silk_semantic::SymbolKind;
use silk_semantic::types::Type;

#[test]
fn test_function_with_return_type_creates_symbol() {
    // Function with return type annotation creates a function symbol
    let source = r#"
def foo() -> int:
    return 42
"#;
    let program = Parser::parse(source).expect("Failed to parse");
    
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).expect("Analysis failed");
    
    let symbol_table = analyzer.symbol_table();
    let symbol = symbol_table.resolve_symbol("foo").expect("foo not found");
    
    assert_eq!(symbol.kind, SymbolKind::Function);
    assert_eq!(symbol.name, "foo");
}

#[test]
fn test_function_without_return_type_unknown() {
    // Function without return type annotation gets Unknown
    let source = r#"
def bar():
    pass
"#;
    let program = Parser::parse(source).expect("Failed to parse");
    
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).expect("Analysis failed");
    
    let symbol_table = analyzer.symbol_table();
    let symbol = symbol_table.resolve_symbol("bar").expect("bar not found");
    
    assert_eq!(symbol.kind, SymbolKind::Function);
    
    // Check that it's a Function type with Unknown return type
    match &symbol.ty {
        Type::Function { return_type } => {
            assert_eq!(**return_type, Type::Unknown);
        }
        _ => panic!("Expected Function type, got {:?}", symbol.ty),
    }
}

#[test]
fn test_function_symbol_has_function_type() {
    // Verify the symbol has Type::Function
    let source = r#"
def add(x: int, y: int) -> int:
    return x + y
"#;
    let program = Parser::parse(source).expect("Failed to parse");
    
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).expect("Analysis failed");
    
    let symbol_table = analyzer.symbol_table();
    let symbol = symbol_table.resolve_symbol("add").expect("add not found");
    
    // Verify it's a Function type
    match &symbol.ty {
        Type::Function { .. } => {
            // Success - it's a function type
        }
        _ => panic!("Expected Type::Function, got {:?}", symbol.ty),
    }
}

#[test]
fn test_function_return_type_resolution() {
    // Check that different return types are correctly resolved
    let source = r#"
def get_int() -> int:
    return 42

def get_str() -> str:
    return "hello"

def get_float() -> float:
    return 3.14

def get_bool() -> bool:
    return True
"#;
    let program = Parser::parse(source).expect("Failed to parse");
    
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).expect("Analysis failed");
    
    let symbol_table = analyzer.symbol_table();
    
    // Check get_int
    let symbol = symbol_table.resolve_symbol("get_int").expect("get_int not found");
    match &symbol.ty {
        Type::Function { return_type } => {
            assert_eq!(**return_type, Type::Int);
        }
        _ => panic!("Expected Function type"),
    }
    
    // Check get_str
    let symbol = symbol_table.resolve_symbol("get_str").expect("get_str not found");
    match &symbol.ty {
        Type::Function { return_type } => {
            assert_eq!(**return_type, Type::Str);
        }
        _ => panic!("Expected Function type"),
    }
    
    // Check get_float
    let symbol = symbol_table.resolve_symbol("get_float").expect("get_float not found");
    match &symbol.ty {
        Type::Function { return_type } => {
            assert_eq!(**return_type, Type::Float);
        }
        _ => panic!("Expected Function type"),
    }
    
    // Check get_bool
    let symbol = symbol_table.resolve_symbol("get_bool").expect("get_bool not found");
    match &symbol.ty {
        Type::Function { return_type } => {
            assert_eq!(**return_type, Type::Bool);
        }
        _ => panic!("Expected Function type"),
    }
}
