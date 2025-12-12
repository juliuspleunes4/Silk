/// Tests for Task 2: Try/Except Return Path Analysis
/// 
/// These tests verify that control flow analysis correctly determines reachability
/// after try/except blocks when returns are involved.

use silk_parser::Parser;
use silk_semantic::{ControlFlowAnalyzer, SemanticError};

fn analyze_control_flow(source: &str) -> Result<(), Vec<SemanticError>> {
    let program = Parser::parse(source).expect("Parser failed");
    let mut analyzer = ControlFlowAnalyzer::new();
    analyzer.analyze(&program)
}

#[test]
fn test_try_returns_except_doesnt() {
    let source = r#"
def foo():
    try:
        return 1
    except Exception:
        handle()
    print("should be REACHABLE - except handler doesn't return")

foo()
"#;
    let result = analyze_control_flow(source);
    // If try returns but except doesn't, exception path is reachable
    assert!(result.is_ok(), "Code after should be reachable: {:?}", result);
}

#[test]
fn test_try_returns_all_excepts_return() {
    let source = r#"
def foo():
    try:
        return 1
    except ValueError:
        return 2
    except KeyError:
        return 3
    print("should be UNREACHABLE - all paths return")

foo()
"#;
    let result = analyze_control_flow(source);
    // Try returns AND all except handlers return → unreachable after
    assert!(result.is_err(), "Code after should be unreachable");
    let errors = result.unwrap_err();
    assert!(errors.iter().any(|e| matches!(e, SemanticError::UnreachableCode { .. })),
        "Expected unreachable code error, got: {:?}", errors);
}

#[test]
fn test_try_returns_some_excepts_return() {
    let source = r#"
def foo():
    try:
        return 1
    except ValueError:
        return 2
    except KeyError:
        handle()
    print("should be REACHABLE - not all excepts return")

foo()
"#;
    let result = analyze_control_flow(source);
    // Try returns but not all excepts return → reachable after
    assert!(result.is_ok(), "Code after should be reachable: {:?}", result);
}

#[test]
fn test_try_doesnt_return_except_returns() {
    let source = r#"
def foo():
    try:
        operation()
    except Exception:
        return 1
    print("should be REACHABLE - try doesn't return")

foo()
"#;
    let result = analyze_control_flow(source);
    // Try doesn't return → reachable after
    assert!(result.is_ok(), "Code after should be reachable: {:?}", result);
}

#[test]
fn test_try_with_else_all_return() {
    let source = r#"
def foo():
    try:
        operation()
    except Exception:
        return 1
    else:
        return 2
    print("should be UNREACHABLE - both except and else return")

foo()
"#;
    let result = analyze_control_flow(source);
    // If try raises: except returns
    // If try succeeds: else returns
    // Either way, all paths return → unreachable after
    assert!(result.is_err(), "Code after should be unreachable: {:?}", result);
}

#[test]
fn test_try_returns_with_else_unreachable() {
    let source = r#"
def foo():
    try:
        return 1
    except Exception:
        return 2
    else:
        print("unreachable else")
        return 3
    print("unreachable after")

foo()
"#;
    let result = analyze_control_flow(source);
    // Try returns, so else never runs, and all paths return
    assert!(result.is_err(), "Code should contain unreachable errors");
    let errors = result.unwrap_err();
    // Should have errors for both unreachable else and unreachable after
    let unreachable_count = errors.iter().filter(|e| matches!(e, SemanticError::UnreachableCode { .. })).count();
    assert!(unreachable_count >= 1, "Expected unreachable code errors, got: {:?}", errors);
}

#[test]
fn test_try_with_finally_returns() {
    let source = r#"
def foo():
    try:
        return 1
    except Exception:
        return 2
    finally:
        cleanup()
    print("should be UNREACHABLE - all paths return")

foo()
"#;
    let result = analyze_control_flow(source);
    // Try returns and all excepts return, finally doesn't affect reachability
    assert!(result.is_err(), "Code after should be unreachable");
}

#[test]
fn test_finally_with_return_overrides() {
    let source = r#"
def foo():
    try:
        operation()
    except Exception:
        handle()
    finally:
        return 1
    print("should be UNREACHABLE - finally returns")

foo()
"#;
    let result = analyze_control_flow(source);
    // Finally returns, so code after is unreachable
    assert!(result.is_err(), "Code after should be unreachable");
}

#[test]
fn test_nested_try_except_returns() {
    let source = r#"
def foo():
    try:
        try:
            return 1
        except ValueError:
            return 2
    except KeyError:
        return 3
    print("should be UNREACHABLE - all nested paths return")

foo()
"#;
    let result = analyze_control_flow(source);
    // All paths in nested try return, outer except returns → unreachable
    assert!(result.is_err(), "Code after should be unreachable");
}

#[test]
fn test_multiple_except_mixed_behavior() {
    let source = r#"
def foo():
    try:
        operation()
    except ValueError:
        return 1
    except KeyError:
        handle()
    except TypeError:
        return 2
    print("should be REACHABLE - not all excepts return")

foo()
"#;
    let result = analyze_control_flow(source);
    // Try doesn't return, and not all excepts return → reachable
    assert!(result.is_ok(), "Code after should be reachable: {:?}", result);
}

#[test]
fn test_bare_except_returns() {
    let source = r#"
def foo():
    try:
        return 1
    except:
        return 2
    print("should be UNREACHABLE - all paths return")

foo()
"#;
    let result = analyze_control_flow(source);
    // Try returns and bare except catches everything and returns → unreachable
    assert!(result.is_err(), "Code after should be unreachable");
}

#[test]
fn test_try_raises_except_returns() {
    let source = r#"
def foo():
    try:
        raise Exception()
    except Exception:
        return 1
    print("should be UNREACHABLE - try raises, except returns")

foo()
"#;
    let result = analyze_control_flow(source);
    // Try diverges (raises), except returns → unreachable after
    assert!(result.is_err(), "Code after should be unreachable");
}
