use silk_parser::Parser;
use silk_semantic::{ControlFlowAnalyzer, SemanticError};

/// Helper to parse source and run control flow analysis
fn analyze_control_flow(source: &str) -> Result<(), Vec<SemanticError>> {
    let program = Parser::parse(source).expect("Parser failed");
    
    let mut analyzer = ControlFlowAnalyzer::new();
    analyzer.analyze(&program)
}

/// Helper to check if error is UninitializedVariable
fn is_uninitialized_variable_error(error: &SemanticError) -> bool {
    matches!(error, SemanticError::UninitializedVariable { .. })
}

#[test]
fn test_variable_initialized_before_use() {
    let source = r#"
x = 5
y = x
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_ok(), "Should have no errors when variable is initialized before use");
}

#[test]
fn test_uninitialized_variable_error() {
    let source = r#"
y = x
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_err(), "Should detect uninitialized variable");
    
    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 1, "Should have exactly 1 error");
    assert!(is_uninitialized_variable_error(&errors[0]), "Should be UninitializedVariable error");
}

#[test]
fn test_function_parameter_always_initialized() {
    let source = r#"
def foo(x):
    y = x
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_ok(), "Function parameters should be initialized");
}

#[test]
fn test_loop_variable_initialized() {
    let source = r#"
items = [1, 2, 3]
for i in items:
    x = i
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_ok(), "Loop variable should be initialized");
}

#[test]
fn test_multiple_assignments() {
    let source = r#"
a = 1
b = 2
c = a + b
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_ok(), "All variables initialized before use");
}

#[test]
fn test_initialization_in_if_branch() {
    let source = r#"
cond = True
if cond:
    x = 1
y = x
"#;
    // Step 10: Variable must be initialized in ALL reachable branches
    // x is only initialized in the if branch, not the else path
    let result = analyze_control_flow(source);
    assert!(result.is_err(), "Step 10: Variable must be initialized in all branches");
    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 1, "Should have exactly 1 error");
    assert!(is_uninitialized_variable_error(&errors[0]), "Should be UninitializedVariable error");
}

#[test]
fn test_walrus_operator_initialization() {
    let source = r#"
if (x := 5) > 0:
    y = x
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_ok(), "Walrus operator should initialize variable");
}

#[test]
fn test_for_loop_target_initialization() {
    let source = r#"
for x in items:
    pass
y = x
"#;
    let result = analyze_control_flow(source);
    // x is initialized by for loop, but items is not defined
    let result_err = result.unwrap_err();
    assert!(result_err.len() >= 1, "Should have error for undefined 'items'");
}

#[test]
fn test_reassignment_is_allowed() {
    let source = r#"
x = 1
x = 2
y = x
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_ok(), "Reassignment should be allowed");
}

#[test]
fn test_except_handler_variable_initialization() {
    let source = r#"
x = 1
try:
    y = x / 0
except Exception as e:
    message = str(e)
    print(message)
"#;
    let result = analyze_control_flow(source);
    // e should be initialized in handler, str and print are built-ins
    assert!(result.is_ok(), "Exception handler variable e should be initialized");
}

#[test]
fn test_with_statement_variable_initialization() {
    let source = r#"
filename = "file.txt"
with open(filename) as f:
    content = f.read()
    print(content)
"#;
    let result = analyze_control_flow(source);
    // f should be initialized, open/print are built-ins, read is an attribute
    assert!(result.is_ok(), "With statement variable f should be initialized");
}

#[test]
fn test_uninitialized_in_expression() {
    let source = r#"
result = x + y
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_err(), "Should detect uninitialized variables in expression");
    
    let errors = result.unwrap_err();
    assert!(errors.len() >= 2, "Should have errors for both x and y");
}

#[test]
fn test_augmented_assignment_initialization() {
    let source = r#"
x = 1
x += 2
y = x
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_ok(), "Augmented assignment should work with initialized variable");
}

#[test]
fn test_augmented_assignment_requires_initialization() {
    let source = r#"
x += 1
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_err(), "Augmented assignment should require variable to exist");
    
    let errors = result.unwrap_err();
    assert!(errors.len() >= 1, "Should have error for uninitialized x");
    assert!(is_uninitialized_variable_error(&errors[0]), "Should be UninitializedVariable error");
}

#[test]
fn test_annotated_assignment_with_value() {
    let source = r#"
x: int = 5
y = x
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_ok(), "Annotated assignment with value should initialize variable");
}

#[test]
fn test_annotated_assignment_without_value() {
    let source = r#"
x: int
y = x
"#;
    let result = analyze_control_flow(source);
    // Annotated assignment without value still initializes the variable (to None conceptually)
    assert!(result.is_ok(), "Annotated assignment should initialize variable even without value");
}

#[test]
fn test_nested_function_scope() {
    let source = r#"
x = 1

def foo():
    y = 2
    z = x + y
"#;
    // Functions create new scopes - x from outer scope is not visible
    // This is correct behavior for Step 9
    let result = analyze_control_flow(source);
    assert!(result.is_err(), "Should have error for x not visible in function scope");
    
    let errors = result.unwrap_err();
    assert!(errors.len() >= 1, "Should have error for x");
}

#[test]
fn test_multiple_function_parameters() {
    let source = r#"
def foo(a, b, c):
    result = a + b + c
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_ok(), "All function parameters should be initialized");
}

#[test]
fn test_vararg_and_kwarg_parameters() {
    let source = r#"
def foo(*args, **kwargs):
    x = args
    y = kwargs
"#;
    let result = analyze_control_flow(source);
    assert!(result.is_ok(), "Vararg and kwarg parameters should be initialized");
}
