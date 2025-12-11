use silk_parser::Parser;
use silk_semantic::{ControlFlowAnalyzer, SemanticError};

/// Helper to parse source and run control flow analysis
fn analyze_control_flow(source: &str) -> Result<(), Vec<SemanticError>> {
    let program = Parser::parse(source).expect("Parser failed");
    
    let mut analyzer = ControlFlowAnalyzer::new();
    analyzer.analyze(&program)
}

#[test]
fn test_reachable_after_try_except() {
    let source = r#"
def foo():
    try:
        risky_operation()
    except Exception:
        handle_error()
    print("reachable - exception might not occur")
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_ok(), "Code after try/except should be reachable");
}

#[test]
fn test_unreachable_in_try_after_return() {
    let source = r#"
def foo():
    try:
        return 1
        print("unreachable in try block")
    except Exception:
        pass
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_err(), "Code after return in try block should be unreachable");
    
    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 1);
    assert!(matches!(errors[0], SemanticError::UnreachableCode { .. }));
}

#[test]
fn test_reachable_in_except_handler() {
    let source = r#"
def foo():
    try:
        might_fail()
    except Exception:
        print("reachable - handler can execute")
        log_error()
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_ok(), "Code in except handler should be reachable");
}

#[test]
fn test_unreachable_in_except_after_return() {
    let source = r#"
def foo():
    try:
        operation()
    except Exception:
        return None
        print("unreachable after return in except")
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_err(), "Code after return in except handler should be unreachable");
}

#[test]
fn test_finally_always_executes() {
    let source = r#"
def foo():
    try:
        operation()
    except Exception:
        handle()
    finally:
        print("reachable - finally always executes")
        cleanup()
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_ok(), "Code in finally block should be reachable");
}

#[test]
fn test_reachable_after_try_except_finally() {
    let source = r#"
def foo():
    try:
        operation()
    except Exception:
        handle()
    finally:
        cleanup()
    print("reachable - after complete try/except/finally")
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_ok(), "Code after try/except/finally should be reachable");
}

#[test]
fn test_unreachable_after_try_all_paths_return() {
    let source = r#"
def foo():
    try:
        return success()
    except Exception:
        return failure()
    print("unreachable - all paths return")
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_err(), "Code should be unreachable when all paths return");
}

#[test]
fn test_reachable_after_try_partial_returns() {
    let source = r#"
def foo():
    try:
        maybe_return()
    except ValueError:
        return "error"
    except TypeError:
        pass
    print("reachable - not all handlers return")
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_ok(), "Code should be reachable when some handlers don't return");
}

#[test]
fn test_nested_try_except() {
    let source = r#"
def foo():
    try:
        try:
            inner_operation()
        except ValueError:
            print("reachable - inner handler")
    except Exception:
        print("reachable - outer handler")
    print("reachable - after nested try")
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_ok(), "Nested try/except should handle reachability correctly");
}

#[test]
fn test_try_with_else_clause() {
    let source = r#"
def foo():
    try:
        operation()
    except Exception:
        handle()
    else:
        print("reachable - else executes if no exception")
    print("reachable - after try/except/else")
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_ok(), "Try else clause should be reachable");
}

#[test]
fn test_finally_after_return_in_try() {
    let source = r#"
def foo():
    try:
        return 1
    finally:
        print("reachable - finally executes even after return")
        cleanup()
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_ok(), "Finally block should be reachable even after return in try");
}

#[test]
fn test_unreachable_after_finally_with_return() {
    let source = r#"
def foo():
    try:
        operation()
    finally:
        return cleanup()
    print("unreachable - finally returns")
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_err(), "Code after finally with return should be unreachable");
}

#[test]
fn test_multiple_except_handlers() {
    let source = r#"
def foo():
    try:
        operation()
    except ValueError:
        print("reachable - handler 1")
    except TypeError:
        print("reachable - handler 2")
    except Exception:
        print("reachable - handler 3")
    print("reachable - after all handlers")
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_ok(), "Multiple except handlers should all be reachable");
}

#[test]
fn test_try_except_in_loop() {
    let source = r#"
def foo():
    for i in range(10):
        try:
            process(i)
        except Exception:
            continue
    print("reachable - loop completes")
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_ok(), "Try/except in loop should handle reachability correctly");
}

#[test]
fn test_bare_except() {
    let source = r#"
def foo():
    try:
        operation()
    except:
        print("reachable - bare except catches all")
    print("reachable - after bare except")
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_ok(), "Bare except handler should be reachable");
}
