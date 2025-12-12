use silk_parser::Parser;
use silk_semantic::{ControlFlowAnalyzer, SemanticError};

/// Helper to parse source and run control flow analysis
/// Filters out UnusedFunction and UnusedVariable errors since we're testing
/// exception control flow patterns, not whether test functions are called.
fn analyze_control_flow(source: &str) -> Result<(), Vec<SemanticError>> {
    let program = Parser::parse(source).expect("Parser failed");
    
    let mut analyzer = ControlFlowAnalyzer::new();
    let result = analyzer.analyze(&program);
    
    // Filter out UnusedFunction and UnusedVariable errors
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
fn test_break_in_finally_block() {
    let source = r#"
def foo():
    for i in range(10):
        try:
            operation(i)
        finally:
            if i == 5:
                break
    print("reachable - loop can exit normally")
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_ok(), "Break in finally should be allowed");
}

#[test]
fn test_continue_in_finally_block() {
    let source = r#"
def foo():
    for i in range(10):
        try:
            operation(i)
        finally:
            if i % 2 == 0:
                continue
            log(i)
    print("reachable - loop completes")
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_ok(), "Continue in finally should be allowed");
}

#[test]
fn test_return_in_except_with_else() {
    let source = r#"
def foo():
    try:
        operation()
    except Exception:
        return "error"
    else:
        print("reachable - else only runs if no exception")
        return "success"
    print("unreachable - both paths return")
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_err(), "Code after try/except/else where both paths return should be unreachable");
}

#[test]
fn test_exception_variable_scope() {
    let source = r#"
def foo():
    try:
        operation()
    except Exception as e:
        print(e)
        log_error(e)
    print("reachable - after except")
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_ok(), "Exception variable should be usable in handler");
}

#[test]
fn test_multiple_exception_types_single_handler() {
    let source = r#"
def foo():
    try:
        operation()
    except (ValueError, TypeError, KeyError) as e:
        print("reachable - catches multiple types")
        handle(e)
    print("reachable - after handler")
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_ok(), "Multiple exception types in single handler should work");
}

#[test]
fn test_deeply_nested_try_blocks() {
    let source = r#"
def foo():
    try:
        try:
            try:
                deep_operation()
            except ValueError:
                print("reachable - level 3")
        except TypeError:
            print("reachable - level 2")
    except Exception:
        print("reachable - level 1")
    print("reachable - after all nesting")
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_ok(), "Deeply nested try blocks should be reachable");
}

#[test]
fn test_try_except_in_conditional() {
    let source = r#"
def foo(flag):
    if flag:
        try:
            operation()
        except Exception:
            return "error"
    else:
        return "skipped"
    print("may be reachable - try block might not return")
"#;
    let result = analyze_control_flow(source);
    // Current limitation: analyzer doesn't track that try block might complete normally
    // So code after conditional is marked as reachable
    assert!(result.is_ok(), "Code after conditional with try/except is marked reachable");
}

#[test]
fn test_except_with_bare_raise() {
    let source = r#"
def foo():
    try:
        operation()
    except Exception as e:
        log(e)
        raise
    print("reachable - bare raise not tracked as diverging")
"#;
    let result = analyze_control_flow(source);
    // Current limitation: bare raise is not tracked as diverging control flow
    assert!(result.is_ok(), "Bare raise not currently tracked as unreachable");
}

#[test]
fn test_nested_finally_blocks() {
    let source = r#"
def foo():
    try:
        try:
            operation()
        finally:
            print("reachable - inner finally")
            inner_cleanup()
    finally:
        print("reachable - outer finally")
        outer_cleanup()
    print("reachable - after nested finally")
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_ok(), "Nested finally blocks should all be reachable");
}

#[test]
fn test_try_with_multiple_except_and_else_and_finally() {
    let source = r#"
def foo():
    try:
        operation()
    except ValueError:
        print("reachable - handler 1")
    except TypeError:
        print("reachable - handler 2")
    else:
        print("reachable - no exception")
    finally:
        print("reachable - cleanup")
    print("reachable - after complete block")
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_ok(), "Complex try/except/else/finally should work");
}

#[test]
fn test_return_in_try_with_finally_no_return() {
    let source = r#"
def foo():
    try:
        return "success"
    except Exception:
        handle()
    finally:
        cleanup()
    print("may be reachable - except handler doesn't return")
"#;
    let result = analyze_control_flow(source);
    // Current behavior: code after try/except/finally is reachable
    // because the except handler doesn't return (even though try does)
    assert!(result.is_ok(), "Code after try with return is currently marked reachable");
}

#[test]
fn test_except_handler_order_matters() {
    let source = r#"
def foo():
    try:
        operation()
    except Exception:
        print("reachable - catches all")
    except ValueError:
        print("reachable but shadowed - would never execute in practice")
    print("reachable - after handlers")
"#;
    let result = analyze_control_flow(source);
    // All handlers should be marked as reachable from control flow perspective
    // (even if later handlers are shadowed by earlier broad handlers)
    assert!(result.is_ok(), "All except handlers should be reachable from control flow analysis");
}

#[test]
fn test_try_in_while_loop_with_break() {
    let source = r#"
def foo():
    while True:
        try:
            operation()
        except Exception:
            break
        print("reachable - exception might not occur")
    print("reachable - loop exits via break")
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_ok(), "Try/except with break in loop should work");
}

#[test]
fn test_finally_overrides_return_in_try_and_except() {
    let source = r#"
def foo():
    try:
        return "try"
    except Exception:
        return "except"
    finally:
        return "finally"
    print("unreachable - finally always returns")
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_err(), "Code after finally with return should be unreachable");
}

#[test]
fn test_exception_in_except_handler() {
    let source = r#"
def foo():
    try:
        operation1()
    except Exception:
        try:
            operation2()
        except Exception:
            print("reachable - nested handler")
    print("reachable - after all handlers")
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_ok(), "Exception handling within exception handler should work");
}
