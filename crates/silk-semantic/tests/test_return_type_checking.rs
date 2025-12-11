use silk_parser::Parser;
use silk_semantic::SemanticAnalyzer;

// ========== VALID RETURN TYPES ==========

#[test]
fn test_valid_int_return() {
    let source = r#"
def f() -> int:
    return 42
"#;
    let program = Parser::parse(source).expect("Failed to parse");
    
    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&program);
    
    assert!(result.is_ok(), "Expected no errors, got: {:?}", result.err());
}

#[test]
fn test_valid_str_return() {
    let source = r#"
def f() -> str:
    return "hello"
"#;
    let program = Parser::parse(source).expect("Failed to parse");
    
    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&program);
    
    assert!(result.is_ok(), "Expected no errors, got: {:?}", result.err());
}

#[test]
fn test_valid_float_return() {
    let source = r#"
def f() -> float:
    return 3.14
"#;
    let program = Parser::parse(source).expect("Failed to parse");
    
    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&program);
    
    assert!(result.is_ok(), "Expected no errors, got: {:?}", result.err());
}

#[test]
fn test_valid_bool_return() {
    let source = r#"
def f() -> bool:
    return True
"#;
    let program = Parser::parse(source).expect("Failed to parse");
    
    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&program);
    
    assert!(result.is_ok(), "Expected no errors, got: {:?}", result.err());
}

#[test]
fn test_valid_int_to_float_return() {
    // int can be returned where float is expected (widening)
    let source = r#"
def f() -> float:
    return 42
"#;
    let program = Parser::parse(source).expect("Failed to parse");
    
    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&program);
    
    assert!(result.is_ok(), "Expected no errors, got: {:?}", result.err());
}

#[test]
fn test_no_return_annotation_allows_anything() {
    // Function without return type annotation should accept any return type
    let source = r#"
def f():
    return "hello"

def g():
    return 42
"#;
    let program = Parser::parse(source).expect("Failed to parse");
    
    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&program);
    
    assert!(result.is_ok(), "Expected no errors, got: {:?}", result.err());
}

// ========== INVALID RETURN TYPES ==========

#[test]
fn test_invalid_int_return() {
    let source = r#"
def f() -> int:
    return "hello"
"#;
    let program = Parser::parse(source).expect("Failed to parse");
    
    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&program);
    
    assert!(result.is_err(), "Expected error for returning str from int function");
    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 1, "Got errors: {:?}", errors);
    assert!(errors[0].to_string().contains("Type mismatch in return statement"), "Error was: {}", errors[0]);
    assert!(errors[0].to_string().contains("function expects"), "Error was: {}", errors[0]);
}

#[test]
fn test_invalid_str_return() {
    let source = r#"
def f() -> str:
    return 3.14
"#;
    let program = Parser::parse(source).expect("Failed to parse");
    
    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&program);
    
    assert!(result.is_err(), "Expected error for returning float from str function");
    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 1);
    assert!(errors[0].to_string().contains("Type mismatch in return statement"));
}

#[test]
fn test_invalid_float_to_int_return() {
    // float cannot be returned where int is expected (narrowing)
    let source = r#"
def f() -> int:
    return 3.14
"#;
    let program = Parser::parse(source).expect("Failed to parse");
    
    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&program);
    
    assert!(result.is_err(), "Expected error for returning float from int function");
    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 1);
    assert!(errors[0].to_string().contains("Type mismatch in return statement"));
}

#[test]
fn test_invalid_bool_return() {
    let source = r#"
def f() -> bool:
    return 42
"#;
    let program = Parser::parse(source).expect("Failed to parse");
    
    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&program);
    
    assert!(result.is_err(), "Expected error for returning int from bool function");
    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 1);
    assert!(errors[0].to_string().contains("Type mismatch in return statement"));
}

// ========== EMPTY RETURN STATEMENTS ==========

#[test]
fn test_empty_return_in_function_with_return_type() {
    let source = r#"
def f() -> int:
    return
"#;
    let program = Parser::parse(source).expect("Failed to parse");
    
    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&program);
    
    assert!(result.is_err(), "Expected error for empty return in int function");
    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 1, "Got errors: {:?}", errors);
    assert!(errors[0].to_string().contains("Type mismatch in return statement"), "Error was: {}", errors[0]);
    assert!(errors[0].to_string().contains("function expects"), "Error was: {}", errors[0]);
}

#[test]
fn test_empty_return_in_function_without_return_type() {
    let source = r#"
def f():
    return
"#;
    let program = Parser::parse(source).expect("Failed to parse");
    
    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&program);
    
    assert!(result.is_ok(), "Empty return should be valid in function without return type");
}

// ========== EXPRESSION RETURNS ==========

#[test]
fn test_return_expression_valid() {
    let source = r#"
def f() -> int:
    x: int = 40
    return x + 2
"#;
    let program = Parser::parse(source).expect("Failed to parse");
    
    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&program);
    
    assert!(result.is_ok(), "Expected no errors for valid expression return");
}

#[test]
fn test_return_expression_invalid() {
    let source = r#"
def f() -> int:
    x: str = "hello"
    y: str = "world"
    return x + y
"#;
    let program = Parser::parse(source).expect("Failed to parse");
    
    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&program);
    
    assert!(result.is_err(), "Expected error for returning str expression from int function");
    let errors = result.unwrap_err();
    assert!(errors.iter().any(|e| e.to_string().contains("Type mismatch in return statement")));
}

// ========== MULTIPLE RETURNS ==========

#[test]
fn test_multiple_valid_returns() {
    let source = r#"
def f(x: int) -> int:
    if x > 0:
        return x
    else:
        return -x
"#;
    let program = Parser::parse(source).expect("Failed to parse");
    
    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&program);
    
    assert!(result.is_ok(), "Expected no errors for multiple valid returns");
}

#[test]
fn test_multiple_returns_with_error() {
    let source = r#"
def f(x: int) -> int:
    if x > 0:
        return x
    else:
        return "negative"
"#;
    let program = Parser::parse(source).expect("Failed to parse");
    
    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&program);
    
    assert!(result.is_err(), "Expected error for invalid return in one branch");
    let errors = result.unwrap_err();
    assert!(errors.iter().any(|e| e.to_string().contains("Type mismatch in return statement")));
}

// ========== NESTED FUNCTIONS ==========

#[test]
fn test_nested_function_returns() {
    let source = r#"
def outer() -> int:
    def inner() -> str:
        return "hello"
    return 42
"#;
    let program = Parser::parse(source).expect("Failed to parse");
    
    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&program);
    
    assert!(result.is_ok(), "Expected no errors for nested functions with different return types");
}

#[test]
fn test_nested_function_return_type_mismatch() {
    let source = r#"
def outer() -> int:
    def inner() -> str:
        return 123
    return 42
"#;
    let program = Parser::parse(source).expect("Failed to parse");
    
    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&program);
    
    assert!(result.is_err(), "Expected error for invalid return in nested function");
    let errors = result.unwrap_err();
    assert!(errors.iter().any(|e| e.to_string().contains("Type mismatch in return statement")));
}

// ========== FUNCTION CALL RETURNS ==========

#[test]
fn test_return_function_call_valid() {
    let source = r#"
def get_int() -> int:
    return 42

def f() -> int:
    return get_int()
"#;
    let program = Parser::parse(source).expect("Failed to parse");
    
    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&program);
    
    assert!(result.is_ok(), "Expected no errors for returning function call result");
}

#[test]
fn test_return_function_call_invalid() {
    let source = r#"
def get_str() -> str:
    return "hello"

def f() -> int:
    return get_str()
"#;
    let program = Parser::parse(source).expect("Failed to parse");
    
    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&program);
    
    assert!(result.is_err(), "Expected error for returning wrong type from function call");
    let errors = result.unwrap_err();
    assert!(errors.iter().any(|e| e.to_string().contains("Type mismatch in return statement")));
}
