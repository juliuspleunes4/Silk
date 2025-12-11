use silk_lexer::Lexer;
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
fn test_unreachable_after_return() {
    let source = r#"
def foo():
    return 42
    print("unreachable")
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_err(), "Should detect unreachable code");
    
    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 1, "Should have exactly 1 error");
    assert!(is_unreachable_code_error(&errors[0]), "Should be UnreachableCode error");
}

#[test]
fn test_unreachable_after_break() {
    let source = r#"
while True:
    break
    print("unreachable")
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_err(), "Should detect unreachable code");
    
    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 1, "Should have exactly 1 error");
    assert!(is_unreachable_code_error(&errors[0]), "Should be UnreachableCode error");
}

#[test]
fn test_unreachable_after_continue() {
    let source = r#"
for i in range(10):
    continue
    print("unreachable")
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_err(), "Should detect unreachable code");
    
    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 1, "Should have exactly 1 error");
    assert!(is_unreachable_code_error(&errors[0]), "Should be UnreachableCode error");
}

#[test]
fn test_unreachable_after_raise() {
    let source = r#"
def bar():
    raise ValueError()
    print("unreachable")
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_err(), "Should detect unreachable code");
    
    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 1, "Should have exactly 1 error");
    assert!(is_unreachable_code_error(&errors[0]), "Should be UnreachableCode error");
}

#[test]
fn test_multiple_unreachable_statements() {
    let source = r#"
def baz():
    return 1
    x = 2
    y = 3
    print(x)
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_err(), "Should detect unreachable code");
    
    let errors = result.unwrap_err();
    // Should detect first unreachable statement (x = 2)
    // Subsequent statements are also unreachable but we stop at first
    assert_eq!(errors.len(), 1, "Should have exactly 1 error");
    assert!(is_unreachable_code_error(&errors[0]), "Should be UnreachableCode error");
}

#[test]
fn test_reachable_in_if_branch() {
    let source = r#"
def conditional(x):
    if x > 0:
        return 1
    print("reachable - no else clause")
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_ok(), "Code after if (without else) is reachable");
}

#[test]
fn test_unreachable_after_if_all_branches_return() {
    let source = r#"
def all_return(x):
    if x > 0:
        return 1
    else:
        return -1
    print("unreachable")
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_err(), "Should detect unreachable code");
    
    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 1, "Should have exactly 1 error");
    assert!(is_unreachable_code_error(&errors[0]), "Should be UnreachableCode error");
}

#[test]
fn test_reachable_after_loop() {
    let source = r#"
for i in range(10):
    if i == 5:
        break
print("reachable - loops can be exited")
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_ok(), "Code after loop is reachable");
}

#[test]
fn test_nested_unreachable_code() {
    let source = r#"
def nested():
    if True:
        return 1
        print("unreachable in if")
    else:
        return 2
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_err(), "Should detect unreachable code");
    
    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 1, "Should have exactly 1 error");
    assert!(is_unreachable_code_error(&errors[0]), "Should be UnreachableCode error");
}

#[test]
fn test_unreachable_in_try_block() {
    let source = r#"
def with_try():
    try:
        return 1
        print("unreachable in try")
    except:
        pass
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_err(), "Should detect unreachable code");
    
    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 1, "Should have exactly 1 error");
    assert!(is_unreachable_code_error(&errors[0]), "Should be UnreachableCode error");
}
