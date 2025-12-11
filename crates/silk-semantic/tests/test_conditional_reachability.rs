use silk_parser::Parser;
use silk_semantic::{ControlFlowAnalyzer, SemanticError};

/// Helper to parse source and run control flow analysis
fn analyze_control_flow(source: &str) -> Result<(), Vec<SemanticError>> {
    let program = Parser::parse(source).expect("Parser failed");
    
    let mut analyzer = ControlFlowAnalyzer::new();
    analyzer.analyze(&program)
}

/// Helper to check if error is UnreachableCode
fn is_unreachable_code_error(error: &SemanticError) -> bool {
    matches!(error, SemanticError::UnreachableCode { .. })
}

#[test]
fn test_reachable_after_if_no_else() {
    let source = r#"
def foo(x):
    if x > 0:
        return 1
    print("reachable - if has no else clause")
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_ok(), "Code after if (without else) should be reachable");
}

#[test]
fn test_reachable_after_if_only_one_branch_returns() {
    let source = r#"
def bar(x):
    if x > 0:
        return 1
    else:
        pass
    print("reachable - only one branch returns")
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_ok(), "Code after if/else should be reachable if one branch doesn't return");
}

#[test]
fn test_unreachable_after_if_all_branches_return() {
    let source = r#"
def baz(x):
    if x > 0:
        return 1
    else:
        return -1
    print("unreachable")
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_err(), "Should detect unreachable code when all branches return");
    
    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 1, "Should have exactly 1 error");
    assert!(is_unreachable_code_error(&errors[0]), "Should be UnreachableCode error");
}

#[test]
fn test_reachable_after_elif_chains() {
    let source = r#"
def multi(x):
    if x > 0:
        return 1
    elif x < 0:
        return -1
    print("reachable - no else clause covers x == 0")
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_ok(), "Code after if/elif (no else) should be reachable");
}

#[test]
fn test_unreachable_after_exhaustive_if_elif_else() {
    let source = r#"
def exhaustive(x):
    if x > 0:
        return 1
    elif x < 0:
        return -1
    else:
        return 0
    print("unreachable")
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_err(), "Should detect unreachable code when all branches return");
    
    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 1, "Should have exactly 1 error");
    assert!(is_unreachable_code_error(&errors[0]), "Should be UnreachableCode error");
}

#[test]
fn test_nested_conditionals_reachability() {
    let source = r#"
def nested(x, y):
    if x > 0:
        if y > 0:
            return 1
        else:
            return 2
        print("unreachable in outer if")
    else:
        return 3
    print("unreachable after outer if/else")
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_err(), "Should detect unreachable code in nested conditionals");
    
    let errors = result.unwrap_err();
    // Should detect unreachable in the nested if block
    assert!(errors.len() >= 1, "Should have at least 1 error");
    assert!(is_unreachable_code_error(&errors[0]), "Should be UnreachableCode error");
}

#[test]
fn test_conditional_in_loop() {
    let source = r#"
def loop_with_conditional():
    for i in range(10):
        if i == 5:
            break
        print(i)
    print("reachable - loop can exit")
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_ok(), "Code after loop should be reachable");
}

#[test]
fn test_early_return_in_nested_if() {
    let source = r#"
def early_return(x, y, z):
    if x > 0:
        if y > 0:
            if z > 0:
                return 1
            print("reachable - inner if has no else")
        print("reachable - middle if has no else")
    print("reachable - outer if has no else")
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_ok(), "Deep nesting without exhaustive branches should be reachable");
}

#[test]
fn test_if_with_break_in_loop() {
    let source = r#"
condition = True
while True:
    if condition:
        break
    else:
        continue
    print("unreachable - all paths break or continue")
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_err(), "Should detect unreachable code when all branches break/continue");
    
    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 1, "Should have exactly 1 error");
    assert!(is_unreachable_code_error(&errors[0]), "Should be UnreachableCode error");
}

#[test]
fn test_if_with_raise_all_branches() {
    let source = r#"
def error_handler(x):
    if x < 0:
        raise ValueError("negative")
    else:
        raise ValueError("non-negative")
    print("unreachable")
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_err(), "Should detect unreachable code when all branches raise");
    
    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 1, "Should have exactly 1 error");
    assert!(is_unreachable_code_error(&errors[0]), "Should be UnreachableCode error");
}

#[test]
fn test_complex_elif_chain_partial_returns() {
    let source = r#"
def complex_chain(x):
    if x == 1:
        return "one"
    elif x == 2:
        pass
    elif x == 3:
        return "three"
    elif x == 4:
        pass
    else:
        return "other"
    print("reachable - some branches don't return")
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_ok(), "Code after if/elif/else should be reachable if any branch doesn't terminate");
}

#[test]
fn test_if_with_mixed_terminators() {
    let source = r#"
def mixed(x):
    if x == 1:
        return 1
    elif x == 2:
        raise Exception()
    elif x == 3:
        return 3
    else:
        raise Exception()
    print("unreachable - all branches terminate")
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_err(), "Should detect unreachable code when all branches terminate (mixed return/raise)");
    
    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 1, "Should have exactly 1 error");
    assert!(is_unreachable_code_error(&errors[0]), "Should be UnreachableCode error");
}
