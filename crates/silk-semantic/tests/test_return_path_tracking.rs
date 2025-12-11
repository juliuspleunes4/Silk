/// Tests for return path tracking (Step 12)
use silk_parser::Parser;
use silk_semantic::{ControlFlowAnalyzer, SemanticError};

/// Helper to analyze source code
/// Filters out UnusedFunction and UnusedVariable errors since we're testing
/// return path tracking, not whether test functions are called.
fn analyze(source: &str) -> Result<(), Vec<SemanticError>> {
    let program = Parser::parse(source).expect("Parser should succeed");
    let mut analyzer = ControlFlowAnalyzer::new();
    let result = analyzer.analyze(&program);
    
    // Filter out UnusedFunction and UnusedVariable errors - we're testing return paths
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
fn test_function_returns_on_all_paths() {
    let source = r#"
def foo(x: int) -> int:
    if x > 0:
        return 1
    else:
        return 2
"#;
    let result = analyze(source);
    assert!(result.is_ok(), "Function that returns on all paths should be OK: {:?}", result.err());
}

#[test]
fn test_missing_return_error() {
    let source = r#"
def foo(x: int) -> int:
    if x > 0:
        return 1
"#;
    let result = analyze(source);
    assert!(result.is_err(), "Function missing return in else path should error");
    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 1, "Should have exactly one error");
    match &errors[0] {
        SemanticError::MissingReturn { function_name, .. } => {
            assert_eq!(function_name, "foo");
        }
        _ => panic!("Expected MissingReturn error, got: {:?}", errors[0]),
    }
}

#[test]
fn test_function_with_no_return_type_ok() {
    let source = r#"
def foo(x):
    if x > 0:
        print(x)
"#;
    let result = analyze(source);
    assert!(result.is_ok(), "Function without return type annotation doesn't need to return: {:?}", result.err());
}

#[test]
fn test_return_in_if_else_all_branches() {
    let source = r#"
def foo(x: int, y: int) -> int:
    if x > 0:
        return x
    elif y > 0:
        return y
    else:
        return 0
"#;
    let result = analyze(source);
    assert!(result.is_ok(), "Function with returns in all if/elif/else branches should be OK: {:?}", result.err());
}

#[test]
fn test_missing_return_in_one_branch() {
    let source = r#"
def foo(x: int) -> int:
    if x > 0:
        return 1
    elif x < 0:
        print("negative")
    else:
        return -1
"#;
    let result = analyze(source);
    assert!(result.is_err(), "Function missing return in elif branch should error");
}

#[test]
fn test_return_after_loop() {
    let source = r#"
def foo() -> int:
    while True:
        break
    return 42
"#;
    let result = analyze(source);
    assert!(result.is_ok(), "Function with return after loop should be OK: {:?}", result.err());
}

#[test]
fn test_return_in_nested_function() {
    let source = r#"
def outer() -> int:
    def inner() -> int:
        return 5
    return inner()
"#;
    let result = analyze(source);
    assert!(result.is_ok(), "Nested functions each with returns should be OK: {:?}", result.err());
}

#[test]
fn test_implicit_none_return() {
    let source = r#"
def foo() -> None:
    print("hello")
"#;
    let result = analyze(source);
    assert!(result.is_ok(), "Function returning None doesn't need explicit return: {:?}", result.err());
}

#[test]
fn test_early_return_ok() {
    let source = r#"
def foo(x: int) -> int:
    if x == 0:
        return 0
    return x * 2
"#;
    let result = analyze(source);
    assert!(result.is_ok(), "Function with early return and final return should be OK: {:?}", result.err());
}

#[test]
fn test_return_in_infinite_loop() {
    let source = r#"
def foo() -> int:
    while True:
        return 42
"#;
    let result = analyze(source);
    assert!(result.is_ok(), "Return inside infinite loop is OK (loop always returns): {:?}", result.err());
}

#[test]
fn test_missing_return_after_conditional() {
    let source = r#"
def foo(x: int) -> int:
    if x > 0:
        print(x)
"#;
    let result = analyze(source);
    assert!(result.is_err(), "Function with no return after if should error");
}

#[test]
fn test_return_with_nested_if() {
    let source = r#"
def foo(a: int, b: int) -> int:
    if a > 0:
        if b > 0:
            return 1
        else:
            return 2
    else:
        return 3
"#;
    let result = analyze(source);
    assert!(result.is_ok(), "Function with nested if/else all returning should be OK: {:?}", result.err());
}
