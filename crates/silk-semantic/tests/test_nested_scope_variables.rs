/// Tests for Step 15.5: Nested Scope Variable Visibility
///
/// Verifies that inner functions can properly access variables from outer scopes (closures).

use silk_parser::Parser;
use silk_semantic::{ControlFlowAnalyzer, SemanticError};

/// Helper to analyze source code
/// Filters out UnusedFunction and UnusedVariable errors since we're testing
/// nested scope variable visibility, not whether test functions are called.
fn analyze(source: &str) -> Result<(), Vec<SemanticError>> {
    let program = Parser::parse(source).expect("Parser should succeed");
    let mut analyzer = ControlFlowAnalyzer::new();
    let result = analyzer.analyze(&program);
    
    // Filter out UnusedFunction and UnusedVariable errors - we're testing scope visibility
    match result {
        Ok(()) => Ok(()),
        Err(errors) => {
            let relevant_errors: Vec<_> = errors
                .into_iter()
                .filter(|e| !matches!(e,
                    SemanticError::UnusedFunction { .. } |
                    SemanticError::UnusedVariable { .. }
                ))
                .collect();
            
            if relevant_errors.is_empty() {
                Ok(())
            } else {
                Err(relevant_errors)
            }
        }
    }
}

#[test]
fn test_inner_function_reads_outer_variable() {
    let source = r#"
def outer():
    x = 5
    def inner():
        return x
    return inner()
"#;
    let result = analyze(source);
    assert!(result.is_ok(), "Inner function should see outer variable: {:?}", result.err());
}

#[test]
fn test_multiple_nesting_levels() {
    let source = r#"
def level1():
    a = 1
    def level2():
        b = 2
        def level3():
            return a + b
        return level3()
    return level2()
"#;
    let result = analyze(source);
    assert!(result.is_ok(), "Deeply nested functions should see all outer variables: {:?}", result.err());
}

#[test]
fn test_inner_function_shadows_outer_variable() {
    let source = r#"
def outer():
    x = 5
    def inner():
        x = 10
        return x
    return inner()
"#;
    let result = analyze(source);
    assert!(result.is_ok(), "Inner function can shadow outer variable: {:?}", result.err());
}

#[test]
fn test_lambda_closure_reads_outer() {
    let source = r#"
def outer():
    x = 5
    f = lambda: x
    return f()
"#;
    let result = analyze(source);
    assert!(result.is_ok(), "Lambda should see outer variable: {:?}", result.err());
}

#[test]
fn test_lambda_with_parameter_and_closure() {
    let source = r#"
def outer():
    x = 5
    f = lambda y: x + y
    return f(10)
"#;
    let result = analyze(source);
    assert!(result.is_ok(), "Lambda with param should see outer variable: {:?}", result.err());
}

#[test]
fn test_nested_function_in_loop() {
    let source = r#"
def outer():
    for i in range(10):
        def inner():
            return i
        inner()
    return 0
"#;
    let result = analyze(source);
    assert!(result.is_ok(), "Nested function in loop should see loop variable: {:?}", result.err());
}

#[test]
fn test_closure_doesnt_see_future_variables() {
    let source = r#"
def outer():
    def inner():
        return y
    y = 5
    return inner()
"#;
    let result = analyze(source);
    assert!(result.is_err(), "Inner function should not see variables defined after it");
    
    let errors = result.unwrap_err();
    assert!(errors.iter().any(|e| matches!(e, SemanticError::UninitializedVariable { name, .. } if name == "y")));
}

#[test]
fn test_nested_exception_handler_visibility() {
    let source = r#"
def outer():
    x = 5
    try:
        risky()
    except Exception as _e:
        def inner():
            return x
        inner()
    return 0
"#;
    let result = analyze(source);
    assert!(result.is_ok(), "Nested function in except should see outer variable: {:?}", result.err());
}

#[test]
fn test_comprehension_sees_outer_variable() {
    // Note: Comprehensions are not yet fully implemented in control flow analysis
    // This test verifies a simple case that should work
    let source = r#"
def outer():
    x = 5
    result = []
    for i in range(10):
        result.append(x + i)
    return result
"#;
    let result = analyze(source);
    assert!(result.is_ok(), "Nested loop should see outer variable: {:?}", result.err());
}

#[test]
fn test_multiple_inner_functions_share_outer_scope() {
    let source = r#"
def outer():
    x = 5
    def inner1():
        return x
    def inner2():
        return x + 1
    return inner1() + inner2()
"#;
    let result = analyze(source);
    assert!(result.is_ok(), "Multiple inner functions should share outer scope: {:?}", result.err());
}

#[test]
fn test_inner_function_parameter_shadows_outer() {
    let source = r#"
def outer():
    x = 5
    def inner(x):
        return x
    return inner(10)
"#;
    let result = analyze(source);
    assert!(result.is_ok(), "Inner function parameter should shadow outer variable: {:?}", result.err());
}

#[test]
fn test_closure_with_unused_outer_variable() {
    let source = r#"
def outer():
    x = 5
    y = 10
    def inner():
        return x
    return inner()
"#;
    let result = analyze(source);
    // y is unused but that's a different concern (UnusedVariable warning)
    // The important thing is no UninitializedVariable error
    match result {
        Ok(()) => assert!(true),
        Err(errors) => {
            // Filter out UnusedVariable errors - we're testing scope visibility
            let init_errors: Vec<_> = errors.iter()
                .filter(|e| matches!(e, SemanticError::UninitializedVariable { .. }))
                .collect();
            assert!(init_errors.is_empty(), "Should not have initialization errors: {:?}", init_errors);
        }
    }
}
