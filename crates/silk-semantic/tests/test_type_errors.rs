//! Tests for type error creation and display

use silk_lexer::Span;
use silk_semantic::SemanticError;

/// Helper function to create a dummy span for testing
fn test_span(line: usize, column: usize) -> Span {
    Span::new(0, 1, line, column)
}

#[test]
fn test_assignment_type_mismatch_error() {
    let error = SemanticError::AssignmentTypeMismatch {
        expected_type: "int".to_string(),
        value_type: "str".to_string(),
        line: 10,
        column: 5,
        span: test_span(10, 5),
    };

    let error_msg = error.to_string();
    assert!(error_msg.contains("Type mismatch in assignment"));
    assert!(error_msg.contains("int"));
    assert!(error_msg.contains("str"));
    assert!(error_msg.contains("line 10"));
}

#[test]
fn test_argument_type_mismatch_error() {
    let error = SemanticError::ArgumentTypeMismatch {
        param_name: "x".to_string(),
        arg_index: 1,
        expected_type: "int".to_string(),
        actual_type: "str".to_string(),
        line: 15,
        column: 10,
        span: test_span(15, 10),
    };

    let error_msg = error.to_string();
    assert!(error_msg.contains("function call"));
    assert!(error_msg.contains("argument 1"));
    assert!(error_msg.contains("parameter 'x'"));
    assert!(error_msg.contains("int"));
    assert!(error_msg.contains("str"));
}

#[test]
fn test_return_type_mismatch_error() {
    let error = SemanticError::ReturnTypeMismatch {
        expected_type: "int".to_string(),
        actual_type: "str".to_string(),
        line: 20,
        column: 8,
        span: test_span(20, 8),
    };

    let error_msg = error.to_string();
    assert!(error_msg.contains("return statement"));
    assert!(error_msg.contains("returning 'str'"));
    assert!(error_msg.contains("expects 'int'"));
}

#[test]
fn test_invalid_binary_operation_error() {
    let error = SemanticError::InvalidBinaryOperation {
        operator: "+".to_string(),
        left_type: "int".to_string(),
        right_type: "str".to_string(),
        line: 25,
        column: 12,
        span: test_span(25, 12),
    };

    let error_msg = error.to_string();
    assert!(error_msg.contains("Invalid operation"));
    assert!(error_msg.contains("operator '+'"));
    assert!(error_msg.contains("'int'"));
    assert!(error_msg.contains("'str'"));
}

#[test]
fn test_invalid_unary_operation_error() {
    let error = SemanticError::InvalidUnaryOperation {
        operator: "-".to_string(),
        operand_type: "str".to_string(),
        line: 30,
        column: 5,
        span: test_span(30, 5),
    };

    let error_msg = error.to_string();
    assert!(error_msg.contains("Invalid operation"));
    assert!(error_msg.contains("operator '-'"));
    assert!(error_msg.contains("type 'str'"));
}

#[test]
fn test_invalid_subscript_error() {
    let error = SemanticError::InvalidSubscript {
        collection_type: "list[int]".to_string(),
        index_type: "str".to_string(),
        line: 35,
        column: 8,
        span: test_span(35, 8),
    };

    let error_msg = error.to_string();
    assert!(error_msg.contains("Invalid subscript"));
    assert!(error_msg.contains("'list[int]'"));
    assert!(error_msg.contains("'str'"));
}

#[test]
fn test_argument_count_mismatch_error() {
    let error = SemanticError::ArgumentCountMismatch {
        function_name: "add".to_string(),
        expected: 2,
        actual: 3,
        line: 40,
        column: 10,
        span: test_span(40, 10),
    };

    let error_msg = error.to_string();
    assert!(error_msg.contains("Argument count mismatch"));
    assert!(error_msg.contains("function 'add'"));
    assert!(error_msg.contains("expects 2"));
    assert!(error_msg.contains("got 3"));
}

#[test]
fn test_error_equality() {
    let error1 = SemanticError::AssignmentTypeMismatch {
        expected_type: "int".to_string(),
        value_type: "str".to_string(),
        line: 10,
        column: 5,
        span: test_span(10, 5),
    };

    let error2 = SemanticError::AssignmentTypeMismatch {
        expected_type: "int".to_string(),
        value_type: "str".to_string(),
        line: 10,
        column: 5,
        span: test_span(10, 5),
    };

    let error3 = SemanticError::ReturnTypeMismatch {
        expected_type: "int".to_string(),
        actual_type: "str".to_string(),
        line: 10,
        column: 5,
        span: test_span(10, 5),
    };

    assert_eq!(error1, error2);
    assert_ne!(error1, error3);
}

#[test]
fn test_error_clone() {
    let error = SemanticError::InvalidBinaryOperation {
        operator: "*".to_string(),
        left_type: "str".to_string(),
        right_type: "int".to_string(),
        line: 45,
        column: 15,
        span: test_span(45, 15),
    };

    let cloned = error.clone();
    assert_eq!(error, cloned);
}

#[test]
fn test_error_debug_format() {
    let error = SemanticError::ArgumentTypeMismatch {
        param_name: "value".to_string(),
        arg_index: 0,
        expected_type: "float".to_string(),
        actual_type: "int".to_string(),
        line: 50,
        column: 20,
        span: test_span(50, 20),
    };

    let debug_str = format!("{:?}", error);
    assert!(debug_str.contains("ArgumentTypeMismatch"));
}
