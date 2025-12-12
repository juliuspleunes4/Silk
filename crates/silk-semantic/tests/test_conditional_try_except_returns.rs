/// Tests for Task 3: Track All-Paths-Return Across Try/Except in Conditionals
/// 
/// These tests verify that control flow analysis correctly determines whether
/// functions return on all paths when try/except blocks are inside conditionals.

use silk_parser::Parser;
use silk_semantic::{ControlFlowAnalyzer, SemanticError};

fn analyze_control_flow(source: &str) -> Result<(), Vec<SemanticError>> {
    let program = Parser::parse(source).expect("Parser failed");
    let mut analyzer = ControlFlowAnalyzer::new();
    analyzer.analyze(&program)
}

#[test]
fn test_if_with_try_except_both_return() {
    let source = r#"
def foo() -> int:
    condition = True
    if condition:
        try:
            return 1
        except Exception:
            return 2
    else:
        return 3

foo()
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_ok(), "Function should return on all paths: {:?}", result);
}

#[test]
fn test_if_else_both_have_try_except_returns() {
    let source = r#"
def foo() -> int:
    condition = True
    if condition:
        try:
            return 1
        except Exception:
            return 2
    else:
        try:
            return 3
        except Exception:
            return 4

foo()
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_ok(), "Function should return on all paths: {:?}", result);
}

#[test]
fn test_if_try_except_returns_no_else() {
    let source = r#"
def foo() -> int:
    condition = True
    if condition:
        try:
            return 1
        except Exception:
            return 2
    x = 1

foo()
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_err(), "Should have missing return error");
    let errors = result.unwrap_err();
    assert!(errors.iter().any(|e| matches!(e, SemanticError::MissingReturn { .. })),
        "Expected missing return error, got: {:?}", errors);
}

#[test]
fn test_try_except_in_if_regular_return_in_else() {
    let source = r#"
def foo() -> int:
    condition = True
    if condition:
        try:
            operation()
            return 1
        except Exception:
            return 2
    else:
        return 3

foo()
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_ok(), "Function should return on all paths: {:?}", result);
}

#[test]
fn test_nested_if_within_try() {
    let source = r#"
def foo() -> int:
    condition = True
    try:
        if condition:
            return 1
        else:
            return 2
    except Exception:
        return 3

foo()
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_ok(), "Function should return on all paths: {:?}", result);
}

#[test]
fn test_nested_if_partial_returns_in_try() {
    let source = r#"
def foo() -> int:
    condition = True
    try:
        if condition:
            return 1
        x = 2
    except Exception:
        return 2
    y = 3

foo()
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_err(), "Should have missing return error");
}

#[test]
fn test_try_in_if_except_doesnt_return() {
    let source = r#"
def foo() -> int:
    condition = True
    if condition:
        try:
            return 1
        except Exception:
            handle()
            return 2
    else:
        return 3

foo()
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_ok(), "Function should return on all paths: {:?}", result);
}

#[test]
fn test_if_try_returns_except_doesnt_no_else() {
    let source = r#"
def foo() -> int:
    condition = True
    if condition:
        try:
            return 1
        except Exception:
            handle()
    x = 1

foo()
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_err(), "Should have missing return error");
}

#[test]
fn test_elif_chain_with_try_except() {
    let source = r#"
def foo() -> int:
    condition1 = True
    condition2 = True
    if condition1:
        try:
            return 1
        except Exception:
            return 2
    elif condition2:
        return 3
    else:
        try:
            return 4
        except Exception:
            return 5

foo()
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_ok(), "Function should return on all paths: {:?}", result);
}

#[test]
fn test_complex_nested_try_in_conditional() {
    let source = r#"
def foo() -> int:
    outer_condition = True
    inner_condition = True
    if outer_condition:
        try:
            if inner_condition:
                return 1
            else:
                return 2
        except ValueError:
            return 3
        except KeyError:
            return 4
    else:
        return 5

foo()
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_ok(), "Function should return on all paths: {:?}", result);
}
