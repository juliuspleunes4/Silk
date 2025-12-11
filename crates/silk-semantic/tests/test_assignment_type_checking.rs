//! Tests for annotated assignment type checking
//!
//! Validates that the semantic analyzer correctly checks type compatibility
//! between declared type annotations and assigned values.

use silk_parser::Parser;
use silk_semantic::{SemanticAnalyzer, SemanticError};

/// Helper function to analyze code and return errors
fn analyze_code(source: &str) -> Result<(), Vec<SemanticError>> {
    let program = Parser::parse(source).expect("Parser should succeed");

    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program)
}

// ========== VALID ASSIGNMENTS ==========

#[test]
fn test_valid_int_assignment() {
    let source = r#"
x: int = 42
"#;
    let result = analyze_code(source);
    assert!(result.is_ok(), "Valid int assignment should succeed");
}

#[test]
fn test_valid_float_assignment() {
    let source = r#"
x: float = 3.14
"#;
    let result = analyze_code(source);
    assert!(result.is_ok(), "Valid float assignment should succeed");
}

#[test]
fn test_valid_str_assignment() {
    let source = r#"
x: str = "hello"
"#;
    let result = analyze_code(source);
    assert!(result.is_ok(), "Valid str assignment should succeed");
}

#[test]
fn test_valid_bool_assignment() {
    let source = r#"
x: bool = True
"#;
    let result = analyze_code(source);
    assert!(result.is_ok(), "Valid bool assignment should succeed");
}

#[test]
fn test_int_to_float_compatible() {
    let source = r#"
x: float = 42
"#;
    let result = analyze_code(source);
    assert!(
        result.is_ok(),
        "Int should be compatible with float annotation"
    );
}

#[test]
fn test_annotation_without_value() {
    let source = r#"
x: int
"#;
    let result = analyze_code(source);
    assert!(result.is_ok(), "Annotation without value should succeed");
}

// ========== INVALID ASSIGNMENTS ==========

#[test]
fn test_invalid_int_str_assignment() {
    let source = r#"
x: int = "hello"
"#;
    let result = analyze_code(source);
    assert!(result.is_err(), "Assigning str to int should fail");

    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 1);

    match &errors[0] {
        SemanticError::AssignmentTypeMismatch {
            expected_type,
            value_type,
            ..
        } => {
            assert_eq!(expected_type.to_string(), "int");
            assert_eq!(value_type.to_string(), "str");
        }
        _ => panic!("Expected AssignmentTypeMismatch error"),
    }
}

#[test]
fn test_invalid_str_int_assignment() {
    let source = r#"
x: str = 42
"#;
    let result = analyze_code(source);
    assert!(result.is_err(), "Assigning int to str should fail");

    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 1);

    match &errors[0] {
        SemanticError::AssignmentTypeMismatch {
            expected_type,
            value_type,
            ..
        } => {
            assert_eq!(expected_type.to_string(), "str");
            assert_eq!(value_type.to_string(), "int");
        }
        _ => panic!("Expected AssignmentTypeMismatch error"),
    }
}

#[test]
fn test_invalid_float_str_assignment() {
    let source = r#"
x: float = "3.14"
"#;
    let result = analyze_code(source);
    assert!(result.is_err(), "Assigning str to float should fail");

    let errors = result.unwrap_err();
    match &errors[0] {
        SemanticError::AssignmentTypeMismatch {
            expected_type,
            value_type,
            ..
        } => {
            assert_eq!(expected_type.to_string(), "float");
            assert_eq!(value_type.to_string(), "str");
        }
        _ => panic!("Expected AssignmentTypeMismatch error"),
    }
}

#[test]
fn test_invalid_bool_int_assignment() {
    let source = r#"
x: bool = 1
"#;
    let result = analyze_code(source);
    assert!(result.is_err(), "Assigning int to bool should fail");

    let errors = result.unwrap_err();
    match &errors[0] {
        SemanticError::AssignmentTypeMismatch {
            expected_type,
            value_type,
            ..
        } => {
            assert_eq!(expected_type.to_string(), "bool");
            assert_eq!(value_type.to_string(), "int");
        }
        _ => panic!("Expected AssignmentTypeMismatch error"),
    }
}

#[test]
fn test_float_to_int_incompatible() {
    let source = r#"
x: int = 3.14
"#;
    let result = analyze_code(source);
    assert!(
        result.is_err(),
        "Assigning float to int should fail (narrowing conversion)"
    );

    let errors = result.unwrap_err();
    match &errors[0] {
        SemanticError::AssignmentTypeMismatch {
            expected_type,
            value_type,
            ..
        } => {
            assert_eq!(expected_type.to_string(), "int");
            assert_eq!(value_type.to_string(), "float");
        }
        _ => panic!("Expected AssignmentTypeMismatch error"),
    }
}

// ========== COLLECTION TYPE CHECKING ==========

#[test]
fn test_valid_list_assignment() {
    let source = r#"
x: list = [1, 2, 3]
"#;
    let result = analyze_code(source);
    assert!(result.is_ok(), "Valid list assignment should succeed");
}

#[test]
fn test_valid_dict_assignment() {
    let source = r#"
x: dict = {"a": 1, "b": 2}
"#;
    let result = analyze_code(source);
    assert!(result.is_ok(), "Valid dict assignment should succeed");
}

#[test]
fn test_valid_set_assignment() {
    let source = r#"
x: set = {1, 2, 3}
"#;
    let result = analyze_code(source);
    assert!(result.is_ok(), "Valid set assignment should succeed");
}

#[test]
fn test_invalid_list_dict_assignment() {
    let source = r#"
x: list = {"a": 1}
"#;
    let result = analyze_code(source);
    assert!(result.is_err(), "Assigning dict to list should fail");

    let errors = result.unwrap_err();
    match &errors[0] {
        SemanticError::AssignmentTypeMismatch {
            expected_type,
            value_type,
            ..
        } => {
            assert!(expected_type.to_string().starts_with("list"));
            assert!(value_type.to_string().starts_with("dict"));
        }
        _ => panic!("Expected AssignmentTypeMismatch error"),
    }
}

// ========== EXPRESSION TYPE CHECKING ==========

#[test]
fn test_valid_arithmetic_expression() {
    let source = r#"
x: int = 10 + 20
y: float = 3.14 * 2.0
"#;
    let result = analyze_code(source);
    assert!(
        result.is_ok(),
        "Valid arithmetic expressions should succeed"
    );
}

#[test]
fn test_invalid_expression_type() {
    let source = r#"
x: int = "hello" + "world"
"#;
    let result = analyze_code(source);
    assert!(
        result.is_err(),
        "Assigning str expression to int should fail"
    );

    let errors = result.unwrap_err();
    match &errors[0] {
        SemanticError::AssignmentTypeMismatch {
            expected_type,
            value_type,
            ..
        } => {
            assert_eq!(expected_type.to_string(), "int");
            assert_eq!(value_type.to_string(), "str");
        }
        _ => panic!("Expected AssignmentTypeMismatch error"),
    }
}

// ========== MULTIPLE ASSIGNMENTS ==========

#[test]
fn test_multiple_valid_assignments() {
    let source = r#"
a: int = 42
b: float = 3.14
c: str = "hello"
d: bool = True
"#;
    let result = analyze_code(source);
    assert!(result.is_ok(), "Multiple valid assignments should succeed");
}

#[test]
fn test_multiple_errors() {
    let source = r#"
a: int = "wrong"
b: str = 123
c: float = True
"#;
    let result = analyze_code(source);
    assert!(result.is_err(), "Multiple invalid assignments should fail");

    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 3, "Should have 3 type mismatch errors");

    // All should be AssignmentTypeMismatch errors
    for error in &errors {
        assert!(matches!(
            error,
            SemanticError::AssignmentTypeMismatch { .. }
        ));
    }
}

// ========== EDGE CASES ==========

#[test]
fn test_none_type_assignment() {
    let source = r#"
x: int = None
"#;
    let result = analyze_code(source);
    assert!(result.is_err(), "Assigning None to int should fail");
}

#[test]
fn test_empty_list_assignment() {
    let source = r#"
x: list = []
"#;
    let result = analyze_code(source);
    assert!(result.is_ok(), "Assigning empty list should succeed");
}

#[test]
fn test_empty_dict_assignment() {
    let source = r#"
x: dict = {}
"#;
    let result = analyze_code(source);
    assert!(result.is_ok(), "Assigning empty dict should succeed");
}
