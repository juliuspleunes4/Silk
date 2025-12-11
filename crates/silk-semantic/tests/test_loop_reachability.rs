use silk_parser::Parser;
use silk_semantic::{ControlFlowAnalyzer, SemanticError};

/// Helper to parse source and run control flow analysis
fn analyze_control_flow(source: &str) -> Result<(), Vec<SemanticError>> {
    let program = Parser::parse(source).expect("Parser failed");
    
    let mut analyzer = ControlFlowAnalyzer::new();
    analyzer.analyze(&program)
}

#[test]
fn test_reachable_after_while_loop() {
    let source = r#"
def foo():
    while x > 0:
        x = x - 1
    print("reachable - loop can exit")
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_ok(), "Code after while loop should be reachable");
}

#[test]
fn test_reachable_after_for_loop() {
    let source = r#"
def foo():
    for i in range(10):
        print(i)
    print("reachable - for loop always exits")
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_ok(), "Code after for loop should be reachable");
}

#[test]
fn test_unreachable_after_infinite_loop_no_break() {
    let source = r#"
def foo():
    while True:
        print("infinite")
    print("unreachable - infinite loop with no break")
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_err(), "Code after infinite loop with no break should be unreachable");
    
    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 1);
    assert!(matches!(errors[0], SemanticError::UnreachableCode { .. }));
}

#[test]
fn test_reachable_after_infinite_loop_with_break() {
    let source = r#"
def foo():
    while True:
        if x > 10:
            break
        print("looping")
    print("reachable - loop has break")
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_ok(), "Code after infinite loop with break should be reachable");
}

#[test]
fn test_reachable_after_infinite_loop_conditional_break() {
    let source = r#"
def foo():
    while True:
        x = x + 1
        if x > 100:
            break
    print("reachable")
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_ok(), "Code after while True with conditional break should be reachable");
}

#[test]
fn test_loop_else_reachability() {
    let source = r#"
def foo():
    while x > 0:
        x = x - 1
    else:
        print("reachable - else executes when loop completes normally")
    print("also reachable")
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_ok(), "Code in loop else clause should be reachable");
}

#[test]
fn test_nested_loops_reachability() {
    let source = r#"
def foo():
    for i in range(10):
        for j in range(5):
            print(i, j)
        print("reachable - inner loop completes")
    print("reachable - outer loop completes")
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_ok(), "Code after nested loops should be reachable");
}

#[test]
fn test_break_in_outer_loop() {
    let source = r#"
def foo():
    while True:
        print("outer")
        break
    print("reachable - break exits loop")
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_ok(), "Code after loop with break should be reachable");
}

#[test]
fn test_continue_doesnt_affect_outer_reachability() {
    let source = r#"
def foo():
    for i in range(10):
        if i % 2 == 0:
            continue
        print(i)
    print("reachable - loop completes despite continue")
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_ok(), "Code after loop with continue should be reachable");
}

#[test]
fn test_while_with_break_in_nested_if() {
    let source = r#"
def foo():
    while True:
        if condition1:
            if condition2:
                break
    print("reachable - nested break can exit")
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_ok(), "Code after while True with nested break should be reachable");
}

#[test]
fn test_infinite_loop_all_paths_continue() {
    let source = r#"
def foo():
    while True:
        if x > 0:
            continue
        else:
            continue
    print("unreachable - all paths continue, no break")
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_err(), "Code after infinite loop with only continues should be unreachable");
}

#[test]
fn test_for_loop_with_break_still_reachable() {
    let source = r#"
def foo():
    for i in range(100):
        if i > 50:
            break
    print("reachable - for loop is finite even with break")
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_ok(), "Code after for loop with break should be reachable");
}

#[test]
fn test_while_true_with_return_not_break() {
    let source = r#"
def foo():
    while True:
        if x > 10:
            return
    print("unreachable - return doesn't break the loop")
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_err(), "Code after while True with only return should be unreachable");
}

#[test]
fn test_loop_else_with_break() {
    let source = r#"
def foo():
    while x > 0:
        if x == 5:
            break
        x = x - 1
    else:
        print("reachable - else executes if no break")
    print("reachable - after loop")
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_ok(), "Loop else clause should be reachable");
}
