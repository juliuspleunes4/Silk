use silk_parser::Parser;
use silk_semantic::SemanticAnalyzer;

// ========== VALID ARITHMETIC OPERATIONS ==========

#[test]
fn test_valid_int_addition() {
    let source = r#"
x: int = 1 + 2
"#;
    let program = Parser::parse(source).expect("Failed to parse");
    
    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&program);
    
    assert!(result.is_ok(), "Expected no errors, got: {:?}", result.err());
}

#[test]
fn test_valid_float_addition() {
    let source = r#"
x: float = 1.5 + 2.5
"#;
    let program = Parser::parse(source).expect("Failed to parse");
    
    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&program);
    
    assert!(result.is_ok(), "Expected no errors, got: {:?}", result.err());
}

#[test]
fn test_valid_mixed_numeric_addition() {
    // int + float is valid
    let source = r#"
x: float = 1 + 2.5
y: float = 3.5 + 4
"#;
    let program = Parser::parse(source).expect("Failed to parse");
    
    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&program);
    
    assert!(result.is_ok(), "Expected no errors, got: {:?}", result.err());
}

#[test]
fn test_valid_string_concatenation() {
    let source = r#"
x: str = "hello" + "world"
"#;
    let program = Parser::parse(source).expect("Failed to parse");
    
    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&program);
    
    assert!(result.is_ok(), "Expected no errors, got: {:?}", result.err());
}

#[test]
fn test_valid_subtraction() {
    let source = r#"
a: int = 10 - 3
b: float = 5.5 - 2.2
c: float = 7 - 1.5
"#;
    let program = Parser::parse(source).expect("Failed to parse");
    
    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&program);
    
    assert!(result.is_ok(), "Expected no errors, got: {:?}", result.err());
}

#[test]
fn test_valid_multiplication() {
    let source = r#"
a: int = 5 * 3
b: float = 2.5 * 4.0
c: float = 3 * 1.5
"#;
    let program = Parser::parse(source).expect("Failed to parse");
    
    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&program);
    
    assert!(result.is_ok(), "Expected no errors, got: {:?}", result.err());
}

#[test]
fn test_valid_division() {
    let source = r#"
a: int = 10 / 2
b: float = 7.5 / 1.5
c: float = 8 / 2.0
"#;
    let program = Parser::parse(source).expect("Failed to parse");
    
    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&program);
    
    assert!(result.is_ok(), "Expected no errors, got: {:?}", result.err());
}

// ========== VALID BITWISE OPERATIONS ==========

#[test]
fn test_valid_bitwise_operations() {
    let source = r#"
a: int = 5 | 3
b: int = 12 & 7
c: int = 9 ^ 6
d: int = 4 << 2
e: int = 16 >> 2
"#;
    let program = Parser::parse(source).expect("Failed to parse");
    
    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&program);
    
    assert!(result.is_ok(), "Expected no errors, got: {:?}", result.err());
}

// ========== INVALID OPERATIONS - TYPE MISMATCHES ==========

#[test]
fn test_invalid_int_plus_string() {
    let source = r#"
x = 1 + "hello"
"#;
    let program = Parser::parse(source).expect("Failed to parse");
    
    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&program);
    
    assert!(result.is_err(), "Expected error for int + str");
    let errors = result.unwrap_err();
    assert!(errors.iter().any(|e| e.to_string().contains("Invalid operation")));
    assert!(errors.iter().any(|e| e.to_string().contains("cannot apply operator '+'")));
}

#[test]
fn test_invalid_string_plus_int() {
    let source = r#"
x = "hello" + 42
"#;
    let program = Parser::parse(source).expect("Failed to parse");
    
    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&program);
    
    assert!(result.is_err(), "Expected error for str + int");
    let errors = result.unwrap_err();
    assert!(errors.iter().any(|e| e.to_string().contains("Invalid operation")));
}

#[test]
fn test_invalid_string_subtraction() {
    let source = r#"
x = "hello" - "world"
"#;
    let program = Parser::parse(source).expect("Failed to parse");
    
    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&program);
    
    assert!(result.is_err(), "Expected error for str - str");
    let errors = result.unwrap_err();
    assert!(errors.iter().any(|e| e.to_string().contains("Invalid operation")));
    assert!(errors.iter().any(|e| e.to_string().contains("'-'")));
}

#[test]
fn test_invalid_string_multiplication_by_float() {
    let source = r#"
x = "hello" * 3.14
"#;
    let program = Parser::parse(source).expect("Failed to parse");
    
    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&program);
    
    assert!(result.is_err(), "Expected error for str * float");
    let errors = result.unwrap_err();
    assert!(errors.iter().any(|e| e.to_string().contains("Invalid operation")));
}

#[test]
fn test_invalid_bool_arithmetic() {
    let source = r#"
x = True + False
"#;
    let program = Parser::parse(source).expect("Failed to parse");
    
    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&program);
    
    assert!(result.is_err(), "Expected error for bool + bool");
    let errors = result.unwrap_err();
    assert!(errors.iter().any(|e| e.to_string().contains("Invalid operation")));
}

// ========== INVALID BITWISE OPERATIONS ==========

#[test]
fn test_invalid_bitwise_on_float() {
    let source = r#"
x = 3.14 | 2.71
"#;
    let program = Parser::parse(source).expect("Failed to parse");
    
    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&program);
    
    assert!(result.is_err(), "Expected error for float | float");
    let errors = result.unwrap_err();
    assert!(errors.iter().any(|e| e.to_string().contains("Invalid operation")));
    assert!(errors.iter().any(|e| e.to_string().contains("'|'")));
}

#[test]
fn test_invalid_bitwise_on_string() {
    let source = r#"
x = "hello" & "world"
"#;
    let program = Parser::parse(source).expect("Failed to parse");
    
    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&program);
    
    assert!(result.is_err(), "Expected error for str & str");
    let errors = result.unwrap_err();
    assert!(errors.iter().any(|e| e.to_string().contains("Invalid operation")));
}

#[test]
fn test_invalid_shift_on_float() {
    let source = r#"
x = 1.5 << 2
"#;
    let program = Parser::parse(source).expect("Failed to parse");
    
    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&program);
    
    assert!(result.is_err(), "Expected error for float << int");
    let errors = result.unwrap_err();
    assert!(errors.iter().any(|e| e.to_string().contains("Invalid operation")));
    assert!(errors.iter().any(|e| e.to_string().contains("'<<'")));
}

// ========== UNKNOWN TYPE HANDLING ==========

#[test]
fn test_unknown_operand_passes_validation() {
    // Operations with Unknown types should pass (gradual typing)
    let source = r#"
def f():
    x = y + 5
"#;
    let program = Parser::parse(source).expect("Failed to parse");
    
    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&program);
    
    // Should only error on undefined 'y', not on the operation
    if let Err(errors) = result {
        assert!(errors.iter().any(|e| e.to_string().contains("Undefined")));
        assert!(!errors.iter().any(|e| e.to_string().contains("Invalid operation")));
    }
}

// ========== NESTED OPERATIONS ==========

#[test]
fn test_nested_valid_operations() {
    let source = r#"
x: int = (1 + 2) * (3 - 1)
y: float = (5.0 / 2.0) + (3.0 * 1.5)
"#;
    let program = Parser::parse(source).expect("Failed to parse");
    
    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&program);
    
    assert!(result.is_ok(), "Expected no errors for nested operations, got: {:?}", result.err());
}

#[test]
fn test_nested_invalid_operations() {
    let source = r#"
x = (1 + 2) * ("hello" + 3)
"#;
    let program = Parser::parse(source).expect("Failed to parse");
    
    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&program);
    
    assert!(result.is_err(), "Expected error for invalid nested operation");
    let errors = result.unwrap_err();
    assert!(errors.iter().any(|e| e.to_string().contains("Invalid operation")));
}

// ========== OPERATIONS IN EXPRESSIONS ==========

#[test]
fn test_operations_in_function_calls() {
    let source = r#"
def add(a: int, b: int) -> int:
    return a + b

x: int = add(1 + 2, 3 * 4)
"#;
    let program = Parser::parse(source).expect("Failed to parse");
    
    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&program);
    
    assert!(result.is_ok(), "Expected no errors, got: {:?}", result.err());
}

#[test]
fn test_invalid_operation_in_assignment() {
    let source = r#"
x: int = 5
y: int = x + "text"
"#;
    let program = Parser::parse(source).expect("Failed to parse");
    
    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&program);
    
    assert!(result.is_err(), "Expected error for invalid operation in assignment");
    let errors = result.unwrap_err();
    assert!(errors.iter().any(|e| e.to_string().contains("Invalid operation")));
}
