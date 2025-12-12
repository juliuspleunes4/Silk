/// Semantic analysis tests for lambda expressions with default parameters
use silk_parser::Parser;
use silk_semantic::{SemanticAnalyzer, SemanticError};

// Helper function to analyze code and filter out control flow warnings
fn analyze_ignoring_warnings(source: &str) -> Result<(), Vec<SemanticError>> {
    let program = Parser::parse(source).unwrap();
    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&program);
    
    // Filter out unused variable/function warnings - we're testing semantics, not control flow
    match result {
        Ok(()) => Ok(()),
        Err(errors) => {
            let serious_errors: Vec<_> = errors.into_iter()
                .filter(|e| !matches!(e, 
                    SemanticError::UnusedVariable { .. } |
                    SemanticError::UnusedFunction { .. }
                ))
                .collect();
            
            if serious_errors.is_empty() {
                Ok(())
            } else {
                Err(serious_errors)
            }
        }
    }
}

#[test]
fn test_lambda_default_type_checking() {
    // Basic lambda with default - should analyze without errors
    let source = r#"
f = lambda x=10: x * 2
result = f()
"#;
    
    let result = analyze_ignoring_warnings(source);
    assert!(result.is_ok(), "Lambda with default should work: {:?}", result.err());
}

#[test]
fn test_lambda_default_with_undefined_variable() {
    // Default value references undefined variable - should error
    let source = r#"
f = lambda x=undefined_var: x
"#;

    
    let result = analyze_ignoring_warnings(source);

    assert!(result.is_err(), "Should error on undefined variable in default");
    
    let errors = result.unwrap_err();
    assert!(!errors.is_empty(), "Should have errors");
    
    // Check that the error mentions the undefined variable
    let error_msg = format!("{:?}", errors[0]);
    assert!(
        error_msg.contains("undefined_var") || error_msg.contains("Undefined"),
        "Error should mention undefined variable, got: {}",
        error_msg
    );
}

#[test]
fn test_lambda_default_simple_literal() {
    // Simple literal default - should work
    let source = r#"
f = lambda x=5: x
"#;

    
    let result = analyze_ignoring_warnings(source);
    
    assert!(result.is_ok(), "Lambda with simple literal default should work: {:?}", result.err());
}

#[test]
fn test_lambda_default_with_outer_scope() {
    // Default references outer scope variable - should work (evaluated at definition time)
    let source = r#"
outer_value = 5
f = lambda x=outer_value: x * 2
result = f()
"#;

    
    let result = analyze_ignoring_warnings(source);

    assert!(result.is_ok(), "Lambda default can reference outer scope variables: {:?}", result.err());
}

#[test]
fn test_lambda_default_with_function_call() {
    // Default can be a function call (evaluated in outer scope)
    let source = r#"
def get_default():
    return 10

f = lambda x=get_default(): x * 2
result = f()
"#;

    
    let result = analyze_ignoring_warnings(source);

    assert!(result.is_ok(), "Lambda with function call default should work: {:?}", result.err());
}

#[test]
fn test_lambda_default_list_literal() {
    // List literal as default
    let source = r#"
f = lambda items=[1, 2, 3]: len(items)
result = f()
"#;

    
    let result = analyze_ignoring_warnings(source);

    assert!(result.is_ok(), "Lambda with list literal default should work: {:?}", result.err());
}

#[test]
fn test_lambda_multiple_defaults() {
    // Multiple parameters with defaults
    let source = r#"
f = lambda x=1, y=2, z=3: x + y + z
result = f()
"#;

    
    let result = analyze_ignoring_warnings(source);

    assert!(result.is_ok(), "Lambda with multiple defaults should work: {:?}", result.err());
}

#[test]
fn test_lambda_default_expression() {
    // Default can be an expression
    let source = r#"
f = lambda x=10+5: x
"#;

    
    let result = analyze_ignoring_warnings(source);

    assert!(result.is_ok(), "Lambda with expression default should work: {:?}", result.err());
}

#[test]
fn test_lambda_default_cannot_reference_parameter() {
    // Default cannot reference another parameter (not in scope yet)
    let source = r#"
f = lambda x=10, y=x: x + y
"#;

    
    let result = analyze_ignoring_warnings(source);

    // Should error - x is not in scope when y's default is evaluated
    assert!(result.is_err(), "Default cannot reference other parameters");
    
    let errors = result.unwrap_err();
    let error_msg = format!("{:?}", errors);
    assert!(
        error_msg.contains("x") || error_msg.contains("Undefined"),
        "Should error about undefined 'x' in default, got: {}",
        error_msg
    );
}

#[test]
fn test_lambda_default_with_nested_lambda() {
    // Nested lambda as default
    let source = r#"
f = lambda func=lambda y: y*2: func(10)
"#;

    
    let result = analyze_ignoring_warnings(source);

    assert!(result.is_ok(), "Lambda with nested lambda default should work: {:?}", result.err());
}

#[test]
fn test_lambda_default_evaluated_in_outer_scope() {
    // This test verifies that defaults are evaluated in outer scope, not lambda scope
    let source = r#"
x = 100
f = lambda x=x: x  # Default 'x' refers to outer x (100), not parameter x
result = f()  # Should use default value of 100
"#;

    
    let result = analyze_ignoring_warnings(source);

    // This should work - outer 'x' is in scope when default is evaluated
    assert!(result.is_ok(), "Default should be evaluated in outer scope: {:?}", result.err());
}
