/// Tests for parameter default value validation.
/// Default expressions are evaluated in the outer scope, not the function scope.

use silk_parser::Parser;
use silk_semantic::SemanticAnalyzer;

/// Helper function to analyze source code
fn analyze(source: &str) -> Result<(), String> {
    let program = Parser::parse(source).expect("Parser should succeed");
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program)
        .map_err(|errors| format!("{:?}", errors))
}

#[test]
fn test_function_default_uses_outer_variable() {
    let source = r#"
x = 42
def f(a=x):
    pass
"#;
    assert!(analyze(source).is_ok()); // Should succeed - x is in outer scope
}

#[test]
fn test_function_default_undefined_variable() {
    let source = r#"
def f(a=undefined_var):
    pass
"#;
    let result = analyze(source);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("undefined_var"));
}

#[test]
fn test_function_default_cannot_use_parameter() {
    let source = r#"
def f(a=10, b=a):
    pass
"#;
    // Should fail - 'a' is not in scope when evaluating b's default
    assert!(analyze(source).is_err());
}

// TODO: Lambda parameter defaults require parser support first
// #[test]
// fn test_lambda_default_uses_outer_variable() {
//     let source = r#"
// x = 42
// f = lambda a=x: a
// "#;
//     assert!(analyze(source).is_ok()); // Should succeed - x is in outer scope
// }

// #[test]
// fn test_lambda_default_undefined_variable() {
//     let source = r#"
// f = lambda a=undefined_var: a
// "#;
//     let result = analyze(source);
//     assert!(result.is_err());
//     assert!(result.unwrap_err().contains("undefined_var"));
// }

// #[test]
// fn test_lambda_default_cannot_use_parameter() {
//     let source = r#"
// f = lambda a=10, b=a: b
// "#;
//     // Should fail - 'a' is not in scope when evaluating b's default
//     assert!(analyze(source).is_err());
// }

#[test]
fn test_kwonly_default_uses_outer_variable() {
    let source = r#"
x = 42
def f(*, a=x):
    pass
"#;
    assert!(analyze(source).is_ok()); // Should succeed - x is in outer scope
}

#[test]
fn test_nested_function_default_uses_enclosing_variable() {
    let source = r#"
def outer():
    x = 100
    def inner(a=x):
        pass
"#;
    assert!(analyze(source).is_ok()); // Should succeed - x is in outer() scope
}

#[test]
fn test_default_with_expression() {
    let source = r#"
x = 10
y = 20
def f(a=x+y):
    pass
"#;
    assert!(analyze(source).is_ok()); // Should succeed - both x and y are in outer scope
}
