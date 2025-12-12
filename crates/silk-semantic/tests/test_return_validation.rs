/// Tests for return type validation (Step 14)
use silk_parser::Parser;
use silk_semantic::{ControlFlowAnalyzer, SemanticError};

/// Helper to analyze source code
/// Filters out UnusedFunction and UnusedVariable errors since we're testing
/// return type validation, not whether test functions are called.
fn analyze(source: &str) -> Result<(), Vec<SemanticError>> {
    let program = Parser::parse(source).expect("Parser should succeed");
    let mut analyzer = ControlFlowAnalyzer::new();
    let result = analyzer.analyze(&program);
    
    // Filter out UnusedFunction and UnusedVariable errors - we're testing return validation
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
fn test_typed_function_must_return() {
    let source = r#"
def foo(x: int) -> int:
    x + 1
"#;
    let result = analyze(source);
    assert!(result.is_err(), "Function with return type but no return statement should error");
    let errors = result.err().unwrap();
    assert_eq!(errors.len(), 1);
    assert!(matches!(errors[0], SemanticError::MissingReturn { .. }));
}

#[test]
fn test_untyped_function_optional_return() {
    let source = r#"
def foo(x):
    x + 1
"#;
    let result = analyze(source);
    assert!(result.is_ok(), "Function without return type should not require return: {:?}", result.err());
}

#[test]
fn test_void_function_explicit_return_none() {
    let source = r#"
def foo():
    print("hello")
    return None
"#;
    let result = analyze(source);
    assert!(result.is_ok(), "Function with explicit return None should be OK: {:?}", result.err());
}

#[test]
fn test_return_type_annotation_enforced() {
    let source = r#"
def foo() -> str:
    if condition():
        return "yes"
"#;
    let result = analyze(source);
    assert!(result.is_err(), "Function with return type missing return in else branch should error");
    let errors = result.err().unwrap();
    assert_eq!(errors.len(), 1);
    assert!(matches!(errors[0], SemanticError::MissingReturn { .. }));
}

#[test]
fn test_missing_return_with_type_hint() {
    let source = r#"
def calculate(x: int, y: int) -> int:
    result = x + y
    print(result)
"#;
    let result = analyze(source);
    assert!(result.is_err(), "Function with return type but no return should error");
    let errors = result.err().unwrap();
    assert_eq!(errors.len(), 1);
    assert!(matches!(errors[0], SemanticError::MissingReturn { .. }));
}

#[test]
fn test_all_paths_return_with_type_hint() {
    let source = r#"
def calculate(x: int, y: int) -> int:
    if x > 0:
        return x + y
    elif x < 0:
        return x - y
    else:
        return 0
"#;
    let result = analyze(source);
    assert!(result.is_ok(), "Function with return on all paths should be OK: {:?}", result.err());
}

#[test]
fn test_optional_return_type_allows_none() {
    let source = r#"
def maybe_value(condition: bool) -> int:
    if condition:
        return 42
"#;
    let result = analyze(source);
    // Missing return when condition is False
    assert!(result.is_err(), "Function missing return in else branch should error");
}

#[test]
fn test_explicit_none_return_type_no_return_needed() {
    let source = r#"
def print_message(msg: str) -> None:
    print(msg)
"#;
    let result = analyze(source);
    assert!(result.is_ok(), "Function returning None doesn't need explicit return: {:?}", result.err());
}

#[test]
fn test_typed_function_with_early_returns() {
    let source = r#"
def validate(x: int) -> bool:
    if x < 0:
        return False
    if x > 100:
        return False
    return True
"#;
    let result = analyze(source);
    assert!(result.is_ok(), "Function with multiple early returns and final return should be OK: {:?}", result.err());
}

#[test]
fn test_typed_function_missing_final_return() {
    let source = r#"
def validate(x: int) -> bool:
    if x < 0:
        return False
    if x > 100:
        return False
"#;
    let result = analyze(source);
    assert!(result.is_err(), "Function missing final return should error");
    let errors = result.err().unwrap();
    assert_eq!(errors.len(), 1);
    assert!(matches!(errors[0], SemanticError::MissingReturn { .. }));
}

#[test]
fn test_untyped_function_with_return_value() {
    let source = r#"
def compute(x, y):
    return x + y
"#;
    let result = analyze(source);
    assert!(result.is_ok(), "Untyped function with return is OK: {:?}", result.err());
}

#[test]
fn test_untyped_function_partial_returns() {
    let source = r#"
def maybe_compute(x, y):
    if x > 0:
        return x + y
    print("negative x")
"#;
    let result = analyze(source);
    // Without return type, partial returns are OK (implicit None on other paths)
    assert!(result.is_ok(), "Untyped function with partial returns should be OK: {:?}", result.err());
}

#[test]
fn test_typed_function_with_pass() {
    let source = r#"
def stub() -> int:
    pass
"#;
    let result = analyze(source);
    assert!(result.is_err(), "Stub function with return type needs return or ...");
    let errors = result.err().unwrap();
    assert_eq!(errors.len(), 1);
    assert!(matches!(errors[0], SemanticError::MissingReturn { .. }));
}

#[test]
fn test_typed_function_with_ellipsis() {
    let source = r#"
def stub() -> int:
    ...
"#;
    let result = analyze(source);
    // Ellipsis is just a literal expression, still needs return
    assert!(result.is_err(), "Stub function with ellipsis still needs return");
}
