//! Tests for function call type checking
//!
//! Validates that the semantic analyzer correctly checks:
//! - Argument count matches parameter count
//! - Argument types match parameter types

use silk_parser::Parser;
use silk_semantic::{SemanticAnalyzer, SemanticError};

/// Helper function to analyze code and return errors
fn analyze_code(source: &str) -> Result<(), Vec<SemanticError>> {
    let program = Parser::parse(source).expect("Parser should succeed");

    let mut analyzer = SemanticAnalyzer::new_without_control_flow();
    analyzer.analyze(&program)
}

// ========== VALID FUNCTION CALLS ==========

#[test]
fn test_valid_single_param_call() {
    let source = r#"
def greet(name: str):
    pass

greet("Alice")
"#;
    let result = analyze_code(source);
    assert!(
        result.is_ok(),
        "Valid function call with matching type should succeed"
    );
}

#[test]
fn test_valid_multiple_params_call() {
    let source = r#"
def add(x: int, y: int):
    pass

add(10, 20)
"#;
    let result = analyze_code(source);
    assert!(
        result.is_ok(),
        "Valid function call with multiple matching types should succeed"
    );
}

#[test]
fn test_valid_mixed_types_call() {
    let source = r#"
def process(name: str, age: int, score: float):
    pass

process("Bob", 25, 95.5)
"#;
    let result = analyze_code(source);
    assert!(
        result.is_ok(),
        "Valid function call with mixed types should succeed"
    );
}

#[test]
fn test_valid_int_to_float_widening() {
    let source = r#"
def calculate(value: float):
    pass

calculate(42)
"#;
    let result = analyze_code(source);
    assert!(result.is_ok(), "Int should widen to float parameter");
}

#[test]
fn test_no_params_function() {
    let source = r#"
def hello():
    pass

hello()
"#;
    let result = analyze_code(source);
    assert!(result.is_ok(), "Function with no parameters should work");
}

#[test]
fn test_function_without_annotations() {
    let source = r#"
def process(x, y):
    pass

process(1, "hello")
"#;
    let result = analyze_code(source);
    assert!(
        result.is_ok(),
        "Function without type annotations should allow any arguments"
    );
}

// ========== ARGUMENT COUNT MISMATCH ==========

#[test]
fn test_too_few_arguments() {
    let source = r#"
def add(x: int, y: int):
    pass

add(10)
"#;
    let result = analyze_code(source);
    assert!(
        result.is_err(),
        "Calling with too few arguments should fail"
    );

    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 1);

    match &errors[0] {
        SemanticError::ArgumentCountMismatch {
            function_name,
            expected,
            actual,
            ..
        } => {
            assert_eq!(function_name, "add");
            assert_eq!(*expected, 2);
            assert_eq!(*actual, 1);
        }
        _ => panic!("Expected ArgumentCountMismatch error"),
    }
}

#[test]
fn test_too_many_arguments() {
    let source = r#"
def greet(name: str):
    pass

greet("Alice", "Bob")
"#;
    let result = analyze_code(source);
    assert!(
        result.is_err(),
        "Calling with too many arguments should fail"
    );

    let errors = result.unwrap_err();
    match &errors[0] {
        SemanticError::ArgumentCountMismatch {
            function_name,
            expected,
            actual,
            ..
        } => {
            assert_eq!(function_name, "greet");
            assert_eq!(*expected, 1);
            assert_eq!(*actual, 2);
        }
        _ => panic!("Expected ArgumentCountMismatch error"),
    }
}

#[test]
fn test_zero_args_to_one_param() {
    let source = r#"
def process(x: int):
    pass

process()
"#;
    let result = analyze_code(source);
    assert!(
        result.is_err(),
        "Calling with no args when params required should fail"
    );

    let errors = result.unwrap_err();
    match &errors[0] {
        SemanticError::ArgumentCountMismatch {
            expected, actual, ..
        } => {
            assert_eq!(*expected, 1);
            assert_eq!(*actual, 0);
        }
        _ => panic!("Expected ArgumentCountMismatch error"),
    }
}

// ========== ARGUMENT TYPE MISMATCH ==========

#[test]
fn test_wrong_type_single_param() {
    let source = r#"
def greet(name: str):
    pass

greet(42)
"#;
    let result = analyze_code(source);
    if let Err(ref errors) = result {
        eprintln!("Errors: {:?}", errors);
    }
    assert!(result.is_err(), "Calling with wrong type should fail");

    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 1);

    match &errors[0] {
        SemanticError::ArgumentTypeMismatch {
            param_name,
            arg_index,
            expected_type,
            actual_type,
            ..
        } => {
            assert_eq!(param_name, "name");
            assert_eq!(*arg_index, 1);
            assert_eq!(expected_type, "str");
            assert_eq!(actual_type, "int");
        }
        _ => panic!("Expected ArgumentTypeMismatch error, got: {:?}", errors[0]),
    }
}

#[test]
fn test_wrong_type_second_param() {
    let source = r#"
def process(name: str, age: int):
    pass

process("Alice", "twenty")
"#;
    let result = analyze_code(source);
    assert!(
        result.is_err(),
        "Wrong type on second parameter should fail"
    );

    let errors = result.unwrap_err();
    match &errors[0] {
        SemanticError::ArgumentTypeMismatch {
            param_name,
            arg_index,
            expected_type,
            actual_type,
            ..
        } => {
            assert_eq!(param_name, "age");
            assert_eq!(*arg_index, 2);
            assert_eq!(expected_type, "int");
            assert_eq!(actual_type, "str");
        }
        _ => panic!("Expected ArgumentTypeMismatch error"),
    }
}

#[test]
fn test_float_to_int_not_allowed() {
    let source = r#"
def process(count: int):
    pass

process(3.14)
"#;
    let result = analyze_code(source);
    assert!(result.is_err(), "Float to int narrowing should fail");

    let errors = result.unwrap_err();
    match &errors[0] {
        SemanticError::ArgumentTypeMismatch {
            expected_type,
            actual_type,
            ..
        } => {
            assert_eq!(expected_type, "int");
            assert_eq!(actual_type, "float");
        }
        _ => panic!("Expected ArgumentTypeMismatch error"),
    }
}

#[test]
fn test_bool_to_str_not_allowed() {
    let source = r#"
def greet(message: str):
    pass

greet(True)
"#;
    let result = analyze_code(source);
    assert!(result.is_err(), "Bool to str should fail");

    let errors = result.unwrap_err();
    match &errors[0] {
        SemanticError::ArgumentTypeMismatch {
            expected_type,
            actual_type,
            ..
        } => {
            assert_eq!(expected_type, "str");
            assert_eq!(actual_type, "bool");
        }
        _ => panic!("Expected ArgumentTypeMismatch error"),
    }
}

// ========== EXPRESSION ARGUMENTS ==========

#[test]
fn test_expression_argument_type() {
    let source = r#"
def add(x: int, y: int):
    pass

add(10 + 5, 20 * 2)
"#;
    let result = analyze_code(source);
    assert!(
        result.is_ok(),
        "Expression arguments with correct types should succeed"
    );
}

#[test]
fn test_expression_wrong_type() {
    let source = r#"
def process(value: int):
    pass

process("hello" + "world")
"#;
    let result = analyze_code(source);
    assert!(
        result.is_err(),
        "Expression with wrong result type should fail"
    );

    let errors = result.unwrap_err();
    match &errors[0] {
        SemanticError::ArgumentTypeMismatch {
            expected_type,
            actual_type,
            ..
        } => {
            assert_eq!(expected_type, "int");
            assert_eq!(actual_type, "str");
        }
        _ => panic!("Expected ArgumentTypeMismatch error"),
    }
}

// ========== NESTED FUNCTION CALLS ==========

#[test]
fn test_nested_function_calls() {
    let source = r#"
def inner(x: int) -> int:
    pass

def outer(y: int):
    pass

outer(inner(42))
"#;
    let result = analyze_code(source);
    assert!(
        result.is_ok(),
        "Nested function calls with matching types should succeed"
    );
}

#[test]
fn test_nested_wrong_type() {
    let source = r#"
def inner(x: int) -> str:
    pass

def outer(y: int):
    pass

outer(inner(42))
"#;
    let result = analyze_code(source);
    assert!(
        result.is_err(),
        "Nested call with wrong return type should fail"
    );

    let errors = result.unwrap_err();
    match &errors[0] {
        SemanticError::ArgumentTypeMismatch {
            expected_type,
            actual_type,
            ..
        } => {
            assert_eq!(expected_type, "int");
            assert!(actual_type.contains("str") || actual_type.contains("function"));
        }
        _ => panic!("Expected ArgumentTypeMismatch error"),
    }
}

// ========== MULTIPLE ERRORS ==========

#[test]
fn test_multiple_type_errors() {
    let source = r#"
def process(name: str, age: int, score: float):
    pass

process(123, "twenty", True)
"#;
    let result = analyze_code(source);
    assert!(result.is_err(), "Multiple wrong types should fail");

    let errors = result.unwrap_err();
    // Should have at least one error (first mismatch)
    assert!(!errors.is_empty());

    match &errors[0] {
        SemanticError::ArgumentTypeMismatch { .. } => {
            // First error should be ArgumentTypeMismatch
        }
        _ => panic!("Expected ArgumentTypeMismatch error"),
    }
}

// ========== FORWARD REFERENCES ==========

#[test]
fn test_forward_reference_call() {
    let source = r#"
def caller():
    callee(42)

def callee(x: int):
    pass
"#;
    let result = analyze_code(source);
    assert!(
        result.is_ok(),
        "Forward reference with correct type should work"
    );
}

#[test]
fn test_forward_reference_wrong_type() {
    let source = r#"
def caller():
    callee("wrong")

def callee(x: int):
    pass
"#;
    let result = analyze_code(source);
    assert!(
        result.is_err(),
        "Forward reference with wrong type should fail"
    );

    let errors = result.unwrap_err();
    match &errors[0] {
        SemanticError::ArgumentTypeMismatch {
            expected_type,
            actual_type,
            ..
        } => {
            assert_eq!(expected_type, "int");
            assert_eq!(actual_type, "str");
        }
        _ => panic!("Expected ArgumentTypeMismatch error"),
    }
}
