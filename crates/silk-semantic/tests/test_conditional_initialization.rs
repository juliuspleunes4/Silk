use silk_parser::Parser;
use silk_semantic::{ControlFlowAnalyzer, SemanticError};

/// Helper to parse source and run control flow analysis
/// Filters out UnusedFunction and UnusedVariable errors since we're testing
/// control flow initialization paths, not whether test functions are called.
fn analyze_control_flow(source: &str) -> Result<(), Vec<SemanticError>> {
    let program = Parser::parse(source).expect("Parser failed");
    
    let mut analyzer = ControlFlowAnalyzer::new();
    let result = analyzer.analyze(&program);
    
    // Filter out UnusedFunction and UnusedVariable errors - we're testing initialization paths
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
fn test_uninitialized_from_conditional_branch() {
    let source = r#"
def foo(cond):
    if cond:
        x = 10
    print(x)
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_err(), "x is not initialized in else branch, should error");
    let errors = result.unwrap_err();
    assert!(errors.len() >= 1, "Should have error for uninitialized x");
}

#[test]
fn test_initialized_in_all_branches() {
    let source = r#"
def foo(cond):
    if cond:
        x = 10
    else:
        x = 20
    print(x)
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_ok(), "x is initialized in both branches, should be valid");
}

#[test]
fn test_initialized_in_if_not_else() {
    let source = r#"
def foo(cond):
    if cond:
        x = 10
    y = x + 1
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_err(), "x not initialized if cond is False");
}

#[test]
fn test_initialized_before_if_used_after() {
    let source = r#"
def foo(cond):
    x = 5
    if cond:
        x = 10
    print(x)
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_ok(), "x initialized before if, can be used after");
}

#[test]
fn test_conditional_initialization_in_loop() {
    let source = r#"
def foo():
    items = [1, 2, 3]
    for item in items:
        if item > 1:
            found = item
    print(found)
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_err(), "found not initialized if all items <= 1");
}

#[test]
fn test_nested_conditional_initialization() {
    let source = r#"
def foo(a, b):
    if a > 0:
        if b > 0:
            x = 1
        else:
            x = 2
    else:
        x = 3
    print(x)
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_ok(), "x initialized in all nested branches");
}

#[test]
fn test_initialization_in_try_except() {
    let source = r#"
def foo():
    try:
        x = risky_operation()
    except Exception:
        x = 0
    print(x)
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_ok(), "x initialized in both try and except");
}

#[test]
fn test_initialization_in_one_except_handler() {
    let source = r#"
def foo():
    y = 0
    try:
        x = risky_operation()
    except ValueError:
        y = 1
    print(x)
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_err(), "x not initialized if exception is not ValueError");
}

#[test]
fn test_initialization_in_all_except_handlers() {
    let source = r#"
def foo():
    try:
        x = risky_operation()
    except ValueError:
        x = 1
    except TypeError:
        x = 2
    except:
        x = 3
    print(x)
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_ok(), "x initialized in try and all except handlers");
}

#[test]
fn test_elif_chain_initialization() {
    let source = r#"
def foo(val):
    if val == 1:
        x = "one"
    elif val == 2:
        x = "two"
    elif val == 3:
        x = "three"
    else:
        x = "other"
    print(x)
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_ok(), "x initialized in all elif branches including else");
}

#[test]
fn test_elif_chain_missing_else() {
    let source = r#"
def foo(val):
    if val == 1:
        x = "one"
    elif val == 2:
        x = "two"
    print(x)
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_err(), "x not initialized if val is not 1 or 2");
}

#[test]
fn test_initialization_with_early_return() {
    let source = r#"
def foo(cond):
    if cond:
        return
    else:
        x = 10
    print(x)
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_ok(), "x only needs to be initialized on reachable path (else branch)");
}

#[test]
fn test_both_branches_initialize_different_vars() {
    let source = r#"
def foo(cond):
    if cond:
        x = 1
        y = 2
    else:
        x = 3
        z = 4
    print(x)
    print(y)
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_err(), "y not initialized in else branch");
}

#[test]
fn test_initialization_in_nested_try_except() {
    let source = r#"
def foo():
    try:
        try:
            x = inner_operation()
        except ValueError:
            x = 1
    except Exception:
        x = 2
    print(x)
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_ok(), "x initialized in all paths through nested try/except");
}

#[test]
fn test_partial_initialization_in_if_elif() {
    let source = r#"
def foo(val):
    if val > 10:
        x = 1
    elif val > 5:
        pass
    else:
        x = 3
    print(x)
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_err(), "x not initialized in elif branch");
}
