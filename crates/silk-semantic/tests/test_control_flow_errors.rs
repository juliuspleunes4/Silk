use silk_lexer::Span;
use silk_semantic::SemanticError;

// ========== ERROR CONSTRUCTION TESTS ==========

#[test]
fn test_unreachable_code_error_construction() {
    let span = Span::new(0, 10, 5, 1);
    let error = SemanticError::UnreachableCode {
        statement_type: "return".to_string(),
        line: 5,
        column: 1,
        span,
    };

    let message = format!("{}", error);
    assert!(message.contains("Unreachable code"));
    assert!(message.contains("line 5"));
    assert!(message.contains("column 1"));
    assert!(message.contains("return"));
}

#[test]
fn test_uninitialized_variable_error_construction() {
    let span = Span::new(0, 10, 3, 5);
    let error = SemanticError::UninitializedVariable {
        name: "x".to_string(),
        line: 3,
        column: 5,
        span,
    };

    let message = format!("{}", error);
    assert!(message.contains("Variable 'x'"));
    assert!(message.contains("may be used before being initialized"));
    assert!(message.contains("line 3"));
    assert!(message.contains("column 5"));
}

#[test]
fn test_missing_return_error_construction() {
    let span = Span::new(0, 10, 10, 1);
    let error = SemanticError::MissingReturn {
        function_name: "calculate".to_string(),
        line: 10,
        column: 1,
        span,
    };

    let message = format!("{}", error);
    assert!(message.contains("Function 'calculate'"));
    assert!(message.contains("missing a return statement"));
    assert!(message.contains("line 10"));
}

#[test]
fn test_infinite_loop_error_construction() {
    let span = Span::new(0, 20, 7, 4);
    let error = SemanticError::InfiniteLoop {
        line: 7,
        column: 4,
        span,
    };

    let message = format!("{}", error);
    assert!(message.contains("Infinite loop"));
    assert!(message.contains("line 7"));
    assert!(message.contains("never terminate"));
}

#[test]
fn test_dead_code_error_construction() {
    let span = Span::new(0, 15, 12, 8);
    let error = SemanticError::DeadCode {
        reason: "function is never called".to_string(),
        line: 12,
        column: 8,
        span,
    };

    let message = format!("{}", error);
    assert!(message.contains("Dead code"));
    assert!(message.contains("line 12"));
    assert!(message.contains("function is never called"));
}

// ========== ERROR VARIANT DISTINCTION TESTS ==========

#[test]
fn test_unreachable_vs_dead_code_distinction() {
    // Unreachable: Code that follows a control flow statement
    let unreachable = SemanticError::UnreachableCode {
        statement_type: "break".to_string(),
        line: 1,
        column: 1,
        span: Span::new(0, 1, 1, 1),
    };

    // Dead code: Code that can never be executed for other reasons
    let dead = SemanticError::DeadCode {
        reason: "never called".to_string(),
        line: 1,
        column: 1,
        span: Span::new(0, 1, 1, 1),
    };

    assert!(matches!(unreachable, SemanticError::UnreachableCode { .. }));
    assert!(matches!(dead, SemanticError::DeadCode { .. }));
}

#[test]
fn test_error_span_tracking() {
    let span = Span::new(100, 150, 10, 5);
    let error = SemanticError::UnreachableCode {
        statement_type: "return".to_string(),
        line: 10,
        column: 5,
        span,
    };

    // Verify error contains correct span information
    if let SemanticError::UnreachableCode { span: err_span, .. } = error {
        assert_eq!(err_span.start, 100);
        assert_eq!(err_span.end, 150);
        assert_eq!(err_span.line, 10);
        assert_eq!(err_span.column, 5);
    } else {
        panic!("Expected UnreachableCode error");
    }
}

// ========== ERROR MESSAGE CLARITY TESTS ==========

#[test]
fn test_unreachable_code_messages_for_different_statements() {
    let test_cases = vec![
        ("return", "code after 'return'"),
        ("break", "code after 'break'"),
        ("continue", "code after 'continue'"),
        ("raise", "code after 'raise'"),
    ];

    for (stmt_type, expected_text) in test_cases {
        let error = SemanticError::UnreachableCode {
            statement_type: stmt_type.to_string(),
            line: 1,
            column: 1,
            span: Span::new(0, 1, 1, 1),
        };

        let message = format!("{}", error);
        assert!(
            message.contains(expected_text),
            "Expected '{}' to contain '{}'",
            message,
            expected_text
        );
    }
}
