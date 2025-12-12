use silk_parser::Parser;
use silk_semantic::{ControlFlowAnalyzer, SemanticError};

fn analyze_control_flow(source: &str) -> Result<(), Vec<SemanticError>> {
    let program = Parser::parse(source).expect("Parser failed");
    let mut analyzer = ControlFlowAnalyzer::new();
    analyzer.analyze(&program)
}

#[test]
fn test_bare_raise_makes_code_unreachable() {
    let source = r#"
def foo():
    try:
        operation()
    except Exception:
        raise
    print("should be REACHABLE - if try succeeds, except doesn't run")

foo()
"#;
    let result = analyze_control_flow(source);
    // Code after is REACHABLE because try might complete normally (no exception)
    // If try completes normally, except handler never runs
    assert!(result.is_ok(), "Code after try/except should be reachable when try can complete normally");
}

#[test]
fn test_bare_raise_in_except_with_finally() {
    let source = r#"
def foo():
    try:
        operation()
    except Exception:
        raise
    finally:
        cleanup()
    print("should be REACHABLE - if try succeeds, except doesn't run")

foo()
"#;
    let result = analyze_control_flow(source);
    // Finally runs, code after is reachable if try completes normally
    assert!(result.is_ok(), "Code should be reachable");
}

#[test]
fn test_simple_raise_at_top_level() {
    let source = r#"
raise ValueError()
print("unreachable")
"#;
    let result = analyze_control_flow(source);
    println!("Errors: {:?}", result);
    assert!(result.is_err(), "Expected unreachable code error");
}

#[test]
fn test_raise_with_expression_makes_unreachable() {
    let source = r#"
def foo():
    raise ValueError("error")
    print("should be unreachable")
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_err(), "Expected unreachable code error");
}

#[test]
fn test_except_without_raise_is_reachable() {
    let source = r#"
def foo():
    try:
        operation()
    except Exception:
        handle()
    print("should be reachable - except doesn't raise")

foo()
"#;
    let result = analyze_control_flow(source);
    // Should NOT have unreachable code error
    assert!(result.is_ok(), "Code after try/except should be reachable when except doesn't raise");
}

#[test]
fn test_all_except_handlers_raise() {
    let source = r#"
def foo():
    try:
        operation()
    except ValueError:
        raise
    except KeyError:
        raise
    print("should be REACHABLE - if try completes normally, no exception")

foo()
"#;
    let result = analyze_control_flow(source);
    // Try might complete normally (no exception) → code after is reachable
    assert!(result.is_ok(), "Code after try/except is reachable if try can complete normally");
}

#[test]
fn test_try_returns_all_except_raise() {
    let source = r#"
def foo():
    try:
        return "success"
    except Exception:
        raise
    print("should be unreachable - try ALWAYS returns")
"#;
    let result = analyze_control_flow(source);
    // Try ALWAYS returns → code after IS unreachable
    assert!(result.is_err(), "Expected unreachable code error");
}

#[test]
fn test_try_raises_except_raises() {
    let source = r#"
def foo():
    try:
        raise ValueError()
    except ValueError:
        raise
    print("should be unreachable - try raises, except re-raises")
"#;
    let result = analyze_control_flow(source);
    // Try always raises, except always re-raises → unreachable
    assert!(result.is_err(), "Expected unreachable code error");
}
