/// Integration tests for control flow analysis with semantic analyzer
/// 
/// These tests verify that control flow analysis is properly integrated
/// with the semantic analyzer and that errors from both systems are properly merged.

use silk_parser::Parser;
use silk_semantic::{SemanticAnalyzer, SemanticError};

// Helper function to filter only control flow errors
fn filter_control_flow_errors(errors: &[SemanticError]) -> Vec<&SemanticError> {
    errors
        .iter()
        .filter(|e| matches!(
            e,
            SemanticError::UnreachableCode { .. }
                | SemanticError::UninitializedVariable { .. }
                | SemanticError::MissingReturn { .. }
                | SemanticError::UnusedVariable { .. }
                | SemanticError::UnusedFunction { .. }
        ))
        .collect()
}

// Helper function to filter only semantic errors (non-control-flow)
fn filter_semantic_errors(errors: &[SemanticError]) -> Vec<&SemanticError> {
    errors
        .iter()
        .filter(|e| !matches!(
            e,
            SemanticError::UnreachableCode { .. }
                | SemanticError::UninitializedVariable { .. }
                | SemanticError::MissingReturn { .. }
                | SemanticError::UnusedVariable { .. }
                | SemanticError::UnusedFunction { .. }
        ))
        .collect()
}

#[test]
fn test_semantic_analyzer_runs_control_flow() {
    // Verify that control flow analysis runs by default
    let source = r#"
def foo():
    x = 5
    return 1
    y = 10
"#;
    let program = Parser::parse(source).expect("Failed to parse");

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&program);

    assert!(result.is_err(), "Expected control flow errors");
    let errors = result.err().unwrap();
    
    // Should have unreachable code and unused variable/function warnings
    let cf_errors = filter_control_flow_errors(&errors);
    assert!(!cf_errors.is_empty(), "Expected control flow errors");
    
    // Check for unreachable code
    assert!(
        cf_errors.iter().any(|e| matches!(e, SemanticError::UnreachableCode { .. })),
        "Expected unreachable code error"
    );
}

#[test]
fn test_combined_type_and_control_flow_errors() {
    // Test that both semantic and control flow errors are reported
    let source = r#"
def bad_function(x: int) -> int:
    if x > 0:
        return "wrong type"
    y = 5
"#;
    let program = Parser::parse(source).expect("Failed to parse");

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&program);

    assert!(result.is_err());
    let errors = result.err().unwrap();
    
    // Should have both semantic errors (type mismatch) and control flow errors (missing return, unused var)
    let semantic = filter_semantic_errors(&errors);
    let cf = filter_control_flow_errors(&errors);
    
    assert!(!semantic.is_empty(), "Expected semantic errors");
    assert!(!cf.is_empty(), "Expected control flow errors");
    
    // Check for return type mismatch
    assert!(
        semantic.iter().any(|e| matches!(e, SemanticError::ReturnTypeMismatch { .. })),
        "Expected return type mismatch, got: {:?}", semantic
    );
    
    // Check for missing return on all paths
    assert!(
        cf.iter().any(|e| matches!(e, SemanticError::MissingReturn { .. })),
        "Expected missing return error, got: {:?}", cf
    );
}

#[test]
fn test_control_flow_after_type_checking() {
    // Verify control flow runs even when type errors exist
    let source = r#"
def get_value() -> int:
    return "wrong"
    x = 10
"#;
    let program = Parser::parse(source).expect("Failed to parse");

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&program);

    assert!(result.is_err());
    let errors = result.err().unwrap();
    
    // Should have both type error and unreachable code
    assert!(
        errors.iter().any(|e| matches!(e, SemanticError::ReturnTypeMismatch { .. })),
        "Expected return type mismatch"
    );
    assert!(
        errors.iter().any(|e| matches!(e, SemanticError::UnreachableCode { .. })),
        "Expected unreachable code error"
    );
}

#[test]
fn test_control_flow_disabled_flag() {
    // Verify that control flow can be disabled
    let source = r#"
def foo():
    x = 5
    return 1
    y = 10
"#;
    let program = Parser::parse(source).expect("Failed to parse");

    let mut analyzer = SemanticAnalyzer::new_without_control_flow();
    let result = analyzer.analyze(&program);

    // Should succeed because control flow is disabled
    assert!(result.is_ok(), "Should succeed with control flow disabled, got: {:?}", result);
}

#[test]
fn test_control_flow_can_be_toggled() {
    // Verify that control flow can be enabled/disabled via setter
    let source = r#"
def foo():
    return 1
    y = 10
"#;
    let program = Parser::parse(source).expect("Failed to parse");

    // First, disable control flow
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.set_control_flow_enabled(false);
    let result1 = analyzer.analyze(&program);
    assert!(result1.is_ok(), "Should succeed with control flow disabled");

    // Then enable it
    let mut analyzer2 = SemanticAnalyzer::new();
    analyzer2.set_control_flow_enabled(true);
    let result2 = analyzer2.analyze(&program);
    assert!(result2.is_err(), "Should fail with control flow enabled");
}

#[test]
fn test_integration_complex_program() {
    // Complex program with multiple types of errors
    let source = r#"
def process(x: int) -> int:
    if x < 0:
        return "negative"
    elif x == 0:
        y = 5
        return 0
    else:
        return x
        z = 10

def unused_helper():
    return 42

result = process(5)
bad: str = result
"#;
    let program = Parser::parse(source).expect("Failed to parse");

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&program);

    assert!(result.is_err());
    let errors = result.err().unwrap();
    
    // Should have:
    // - Return type mismatch (semantic)
    // - Unreachable code (control flow)
    // - Unused variable (control flow)
    // - Unused function (control flow)
    // - Assignment type mismatch (semantic)
    
    assert!(
        errors.iter().any(|e| matches!(e, SemanticError::ReturnTypeMismatch { .. })),
        "Expected return type mismatch"
    );
    assert!(
        errors.iter().any(|e| matches!(e, SemanticError::UnreachableCode { .. })),
        "Expected unreachable code"
    );
    assert!(
        errors.iter().any(|e| matches!(e, SemanticError::UnusedVariable { .. })),
        "Expected unused variable"
    );
    assert!(
        errors.iter().any(|e| matches!(e, SemanticError::UnusedFunction { .. })),
        "Expected unused function"
    );
    assert!(
        errors.iter().any(|e| matches!(e, SemanticError::AssignmentTypeMismatch { .. })),
        "Expected assignment type mismatch"
    );
}

#[test]
fn test_error_ordering_preserved() {
    // Verify that semantic errors come before control flow errors
    let source = r#"
x = undefined_var
y = 5
"#;
    let program = Parser::parse(source).expect("Failed to parse");

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&program);

    assert!(result.is_err());
    let errors = result.err().unwrap();
    
    // The first error should be the semantic error (undefined variable)
    assert!(
        matches!(errors[0], SemanticError::UndefinedVariable { .. }),
        "First error should be UndefinedVariable, got: {:?}", errors[0]
    );
    
    // Later errors should be control flow warnings
    let has_cf = errors.iter().skip(1).any(|e| matches!(e, SemanticError::UnusedVariable { .. }));
    assert!(has_cf, "Should have control flow warnings after semantic errors");
}

#[test]
fn test_no_duplicate_errors() {
    // Verify that errors are not duplicated between semantic and control flow
    let source = r#"
def foo() -> int:
    return 1

x = foo()
print(x)
"#;
    let program = Parser::parse(source).expect("Failed to parse");

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&program);

    // Should succeed - no errors
    assert!(result.is_ok(), "Valid program should not produce errors, got: {:?}", result);
}

#[test]
fn test_uninitialized_variable_detection() {
    // Test that control flow detects uninitialized variables
    let source = r#"
def foo():
    print(x)
    x = 5
"#;
    let program = Parser::parse(source).expect("Failed to parse");

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&program);

    assert!(result.is_err());
    let errors = result.err().unwrap();
    
    assert!(
        errors.iter().any(|e| matches!(e, SemanticError::UninitializedVariable { .. })),
        "Expected uninitialized variable error, got: {:?}", errors
    );
}

#[test]
fn test_missing_return_detection() {
    // Test that control flow detects missing returns
    let source = r#"
def get_value() -> int:
    x = 5
    if x > 0:
        return 1
"#;
    let program = Parser::parse(source).expect("Failed to parse");

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&program);

    assert!(result.is_err());
    let errors = result.err().unwrap();
    
    assert!(
        errors.iter().any(|e| matches!(e, SemanticError::MissingReturn { .. })),
        "Expected missing return error, got: {:?}", errors
    );
}
