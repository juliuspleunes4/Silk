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

// ============== ADVANCED TESTS (Step 8) ==============

#[test]
fn test_nested_function_calls() {
    let source = r#"
def get_int() -> int:
    return 42

def process(x: int) -> str:
    return "processed"

result = process(get_int())
"#;
    let program = Parser::parse(source).unwrap();
    
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();
    
    // result should have type str (from process's return type)
    let symbol = analyzer.symbol_table().resolve_symbol("result").unwrap();
    assert_eq!(symbol.ty, Type::Str);
}

#[test]
fn test_function_call_in_expression() {
    let source = r#"
def get_x() -> int:
    return 10

def get_y() -> int:
    return 20

result = get_x() + get_y()
"#;
    let program = Parser::parse(source).unwrap();
    
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();
    
    // result should have type int (int + int = int)
    let symbol = analyzer.symbol_table().resolve_symbol("result").unwrap();
    assert_eq!(symbol.ty, Type::Int);
}

#[test]
fn test_function_call_as_argument() {
    let source = r#"
def get_message() -> str:
    return "hello"

result = print(get_message())
"#;
    let program = Parser::parse(source).unwrap();
    
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();
    
    // result should have type None (print returns None)
    let symbol = analyzer.symbol_table().resolve_symbol("result").unwrap();
    assert_eq!(symbol.ty, Type::None);
}

#[test]
fn test_multiple_calls_to_same_function() {
    let source = r#"
def create() -> int:
    return 42

x = create()
y = create()
z = create()
"#;
    let program = Parser::parse(source).unwrap();
    
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();
    
    // All should have type int
    let x = analyzer.symbol_table().resolve_symbol("x").unwrap();
    let y = analyzer.symbol_table().resolve_symbol("y").unwrap();
    let z = analyzer.symbol_table().resolve_symbol("z").unwrap();
    assert_eq!(x.ty, Type::Int);
    assert_eq!(y.ty, Type::Int);
    assert_eq!(z.ty, Type::Int);
}

#[test]
fn test_recursive_function_call() {
    let source = r#"
def factorial(n: int) -> int:
    if n <= 1:
        return 1
    return n * factorial(n - 1)

result = factorial(5)
"#;
    let program = Parser::parse(source).unwrap();
    
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();
    
    // result should have type int
    let symbol = analyzer.symbol_table().resolve_symbol("result").unwrap();
    assert_eq!(symbol.ty, Type::Int);
}

#[test]
fn test_method_call_returns_unknown() {
    let source = r#"
class MyClass:
    def method(self) -> int:
        return 42

obj = MyClass()
result = obj.method()
"#;
    let program = Parser::parse(source).unwrap();
    
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();
    
    // Method calls not yet supported - returns Unknown
    let symbol = analyzer.symbol_table().resolve_symbol("result").unwrap();
    assert_eq!(symbol.ty, Type::Unknown);
}

#[test]
fn test_calling_non_function_variable() {
    let source = r#"
x = 42
result = x()
"#;
    let program = Parser::parse(source).unwrap();
    
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();
    
    // Calling a non-function (int) returns Unknown
    let symbol = analyzer.symbol_table().resolve_symbol("result").unwrap();
    assert_eq!(symbol.ty, Type::Unknown);
}

#[test]
fn test_undefined_function_call_returns_unknown() {
    let source = r#"
result = undefined_function()
"#;
    let program = Parser::parse(source).unwrap();
    
    let mut analyzer = SemanticAnalyzer::new();
    // Will have error about undefined function
    let analysis_result = analyzer.analyze(&program);
    assert!(analysis_result.is_err(), "Should have undefined variable error");
    
    // But result still gets a type (Unknown)
    let symbol = analyzer.symbol_table().resolve_symbol("result").unwrap();
    assert_eq!(symbol.ty, Type::Unknown);
}

#[test]
fn test_builtin_input_returns_str() {
    let source = r#"
user_input = input("Enter name: ")
"#;
    let program = Parser::parse(source).unwrap();
    
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();
    
    // input returns str
    let symbol = analyzer.symbol_table().resolve_symbol("user_input").unwrap();
    assert_eq!(symbol.ty, Type::Str);
}

#[test]
fn test_builtin_int_returns_int() {
    let source = r#"
number = int("42")
"#;
    let program = Parser::parse(source).unwrap();
    
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();
    
    // int() returns int
    let symbol = analyzer.symbol_table().resolve_symbol("number").unwrap();
    assert_eq!(symbol.ty, Type::Int);
}

#[test]
fn test_builtin_float_returns_float() {
    let source = r#"
number = float("3.14")
"#;
    let program = Parser::parse(source).unwrap();
    
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();
    
    // float() returns float
    let symbol = analyzer.symbol_table().resolve_symbol("number").unwrap();
    assert_eq!(symbol.ty, Type::Float);
}

#[test]
fn test_builtin_bool_returns_bool() {
    let source = r#"
flag = bool(1)
"#;
    let program = Parser::parse(source).unwrap();
    
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();
    
    // bool() returns bool
    let symbol = analyzer.symbol_table().resolve_symbol("flag").unwrap();
    assert_eq!(symbol.ty, Type::Bool);
}
