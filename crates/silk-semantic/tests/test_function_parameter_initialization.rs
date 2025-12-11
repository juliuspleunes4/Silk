/// Tests for function parameter initialization tracking (Step 11)
use silk_parser::Parser;
use silk_semantic::{ControlFlowAnalyzer, SemanticError};

/// Helper to analyze source code
fn analyze(source: &str) -> Result<(), Vec<SemanticError>> {
    let program = Parser::parse(source).expect("Parser should succeed");
    let mut analyzer = ControlFlowAnalyzer::new();
    analyzer.analyze(&program)
}

#[test]
fn test_parameter_initialized_on_entry() {
    let source = r#"
def foo(x):
    return x
"#;
    let result = analyze(source);
    assert!(result.is_ok(), "Function parameter should be initialized on entry");
}

#[test]
fn test_args_kwargs_initialized() {
    let source = r#"
def foo(*args, **kwargs):
    return (args, kwargs)
"#;
    let result = analyze(source);
    assert!(result.is_ok(), "*args and **kwargs should be initialized on entry");
}

#[test]
fn test_default_parameter_expression_checked() {
    let source = r#"
def foo(x=undefined_var):
    pass
"#;
    let result = analyze(source);
    assert!(result.is_err(), "Default parameter should check for uninitialized variables");
    let errors = result.unwrap_err();
    // Should have at least one error (UninitializedVariable)
    // May also have UnusedVariable for parameter x
    assert!(!errors.is_empty(), "Should have at least one error");
    // Verify there's an UninitializedVariable error
    assert!(errors.iter().any(|e| matches!(e, SemanticError::UninitializedVariable { .. })));
}

#[test]
fn test_default_uses_outer_variable() {
    let source = r#"
x = 10
def foo(y=x):
    return y
"#;
    let result = analyze(source);
    assert!(result.is_ok(), "Default can use variable from outer scope: {:?}", result.err());
}

#[test]
fn test_default_cannot_use_parameter() {
    let source = r#"
def foo(a=10, b=a):
    pass
"#;
    let result = analyze(source);
    assert!(result.is_err(), "Default expression cannot use other parameters");
    let errors = result.unwrap_err();
    // Should have at least one error (UninitializedVariable for 'a')
    // May also have UnusedVariable errors for unused parameters
    assert!(!errors.is_empty(), "Should have at least one error");
    // Verify there's an UninitializedVariable error for 'a'
    assert!(errors.iter().any(|e| matches!(e, SemanticError::UninitializedVariable { name, .. } if name == "a")));
}

#[test]
fn test_parameter_shadows_outer_scope() {
    let source = r#"
x = 10
def foo(x):
    return x
"#;
    let result = analyze(source);
    assert!(result.is_ok(), "Parameter shadows outer scope variable");
}

#[test]
fn test_nested_function_parameter_scope() {
    let source = r#"
def outer(x):
    def inner(y):
        return y
    return inner(x)
"#;
    let result = analyze(source);
    assert!(result.is_ok(), "Nested function parameters should be initialized: {:?}", result.err());
}

#[test]
fn test_lambda_parameter_initialization() {
    let source = r#"
f = lambda x: x + 1
y = f(5)
print(y)
"#;
    let result = analyze(source);
    assert!(result.is_ok(), "Lambda parameter should be initialized: {:?}", result.err());
}

#[test]
fn test_multiple_defaults_with_expression() {
    let source = r#"
a = 10
b = 20
def foo(x=a, y=b, z=a+b):
    return x + y + z
"#;
    let result = analyze(source);
    assert!(result.is_ok(), "Multiple defaults can use outer variables and expressions: {:?}", result.err());
}

#[test]
fn test_kwonly_default_checked() {
    let source = r#"
def foo(*, x=undefined_var):
    pass
"#;
    let result = analyze(source);
    assert!(result.is_err(), "Keyword-only parameter default should be checked");
}

#[test]
fn test_mixed_params_all_initialized() {
    let source = r#"
def foo(a, b=10, *args, c, d=20, **kwargs):
    return a + b + c + d + len(args) + len(kwargs)
"#;
    let result = analyze(source);
    assert!(result.is_ok(), "All parameter types should be initialized");
}

#[test]
fn test_default_with_function_call() {
    let source = r#"
def get_default():
    return 42

def foo(y=get_default()):
    return y
"#;
    let result = analyze(source);
    assert!(result.is_ok(), "Default can call function from outer scope: {:?}", result.err());
}
