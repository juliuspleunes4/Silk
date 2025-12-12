/// Tests for complex return patterns (Step 13)
use silk_parser::Parser;
use silk_semantic::{ControlFlowAnalyzer, SemanticError};

/// Helper to analyze source code
/// Filters out UnusedFunction and UnusedVariable errors since we're testing return paths
fn analyze(source: &str) -> Result<(), Vec<SemanticError>> {
    let program = Parser::parse(source).expect("Parser should succeed");
    let mut analyzer = ControlFlowAnalyzer::new();
    match analyzer.analyze(&program) {
        Ok(()) => Ok(()),
        Err(errors) => {
            // Filter out UnusedFunction and UnusedVariable errors
            let relevant_errors: Vec<_> = errors.into_iter()
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
fn test_return_in_try_block_only() {
    let source = r#"
def foo() -> int:
    try:
        return 1
    except:
        print("error")
"#;
    let result = analyze(source);
    assert!(result.is_err(), "Function missing return in except handler should error");
    let errors = result.err().unwrap();
    assert_eq!(errors.len(), 1);
    assert!(matches!(errors[0], SemanticError::MissingReturn { .. }));
}

#[test]
fn test_return_in_all_try_except_branches() {
    let source = r#"
def foo() -> int:
    try:
        return 1
    except:
        return 2
"#;
    let result = analyze(source);
    assert!(result.is_ok(), "Function with return in all try/except branches should be OK: {:?}", result.err());
}

#[test]
fn test_return_in_except_handler_only() {
    let source = r#"
def foo() -> int:
    try:
        print("attempt")
    except:
        return 1
"#;
    let result = analyze(source);
    assert!(result.is_err(), "Function missing return in try block should error");
    let errors = result.err().unwrap();
    assert_eq!(errors.len(), 1);
    assert!(matches!(errors[0], SemanticError::MissingReturn { .. }));
}

#[test]
fn test_return_in_finally_overrides() {
    let source = r#"
def foo() -> int:
    try:
        print("attempt")
    except:
        print("error")
    finally:
        return 1
"#;
    let result = analyze(source);
    // Finally always executes and has return, so this is OK
    assert!(result.is_ok(), "Function with return in finally should be OK: {:?}", result.err());
}

#[test]
fn test_return_in_loop_not_sufficient() {
    let source = r#"
def foo() -> int:
    while condition():
        return 1
"#;
    let result = analyze(source);
    // Loop might not execute, so return inside loop doesn't guarantee function returns
    assert!(result.is_err(), "Return in loop should not be sufficient");
    let errors = result.err().unwrap();
    assert_eq!(errors.len(), 1);
    assert!(matches!(errors[0], SemanticError::MissingReturn { .. }));
}

#[test]
fn test_return_in_loop_with_return_after() {
    let source = r#"
def foo() -> int:
    while condition():
        return 1
    return 2
"#;
    let result = analyze(source);
    assert!(result.is_ok(), "Function with return after loop should be OK: {:?}", result.err());
}

#[test]
fn test_return_in_for_loop_not_sufficient() {
    let source = r#"
def foo() -> int:
    for x in items():
        return x
"#;
    let result = analyze(source);
    // For loop might iterate over empty sequence
    assert!(result.is_err(), "Return in for loop should not be sufficient");
    let errors = result.err().unwrap();
    assert_eq!(errors.len(), 1);
    assert!(matches!(errors[0], SemanticError::MissingReturn { .. }));
}

#[test]
fn test_return_after_infinite_loop_unreachable() {
    let source = r#"
def foo() -> int:
    while True:
        pass
    return 1
"#;
    let result = analyze(source);
    // Infinite loop makes code after unreachable, but also means function doesn't return
    // The function should error for missing return because infinite loop doesn't have return
    assert!(result.is_err(), "Infinite loop without return should error");
}

#[test]
fn test_return_in_infinite_loop_sufficient() {
    let source = r#"
def foo() -> int:
    while True:
        return 1
"#;
    let result = analyze(source);
    // Infinite loop that always returns is OK
    assert!(result.is_ok(), "Infinite loop with return should be OK: {:?}", result.err());
}

#[test]
fn test_multiple_return_points() {
    let source = r#"
def foo(x: int) -> int:
    if x > 0:
        return 1
    elif x < 0:
        return -1
    return 0
"#;
    let result = analyze(source);
    assert!(result.is_ok(), "Function with multiple returns should be OK: {:?}", result.err());
}

#[test]
fn test_conditional_return_with_raise() {
    let source = r#"
def foo(x: int) -> int:
    if x > 0:
        return x
    else:
        raise ValueError("negative")
"#;
    let result = analyze(source);
    assert!(result.is_ok(), "Function with return or raise in all paths should be OK: {:?}", result.err());
}

#[test]
fn test_nested_try_return() {
    let source = r#"
def foo() -> int:
    try:
        try:
            return 1
        except:
            return 2
    except:
        return 3
"#;
    let result = analyze(source);
    assert!(result.is_ok(), "Nested try with returns in all paths should be OK: {:?}", result.err());
}

#[test]
fn test_try_with_else_return() {
    let source = r#"
def foo() -> int:
    try:
        attempt()
    except:
        return 1
    else:
        return 2
"#;
    let result = analyze(source);
    // Try body doesn't return, else returns, except returns
    // If try succeeds, else executes (returns 2)
    // If try raises, except executes (returns 1)
    // So all paths return
    assert!(result.is_ok(), "Try with else and except both returning should be OK: {:?}", result.err());
}

#[test]
fn test_return_in_loop_else_not_sufficient() {
    let source = r#"
def foo() -> int:
    while condition():
        break
    else:
        return 1
"#;
    let result = analyze(source);
    // Loop else only executes if loop doesn't break
    // If loop breaks, we exit without return
    assert!(result.is_err(), "Return only in loop else should not be sufficient");
}
