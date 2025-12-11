//! Tests for semantic analyzer with real AST nodes

use silk_parser::Parser;
use silk_semantic::{SemanticAnalyzer, SemanticError};

/// Helper to parse and analyze source code
fn analyze(source: &str) -> Result<(), Vec<SemanticError>> {
    let program = Parser::parse(source).expect("Parser should succeed");

    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program)
}

// ========== SYMBOL COLLECTION TESTS ==========

#[test]
fn test_collect_simple_assignment() {
    let source = "x = 5";
    let result = analyze(source);
    assert!(
        result.is_ok(),
        "Simple assignment should not produce errors"
    );
}

#[test]
fn test_collect_multiple_assignments() {
    let source = r#"
x = 1
y = 2
z = 3
    "#;
    let result = analyze(source);
    assert!(
        result.is_ok(),
        "Multiple assignments should not produce errors"
    );
}

#[test]
fn test_collect_use_before_define() {
    let source = "y = x + 5";
    let result = analyze(source);
    // x is undefined
    assert!(result.is_err(), "x should be undefined");
    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 1);
    assert!(matches!(errors[0], SemanticError::UndefinedVariable { .. }));
}

#[test]
fn test_collect_augmented_assignment() {
    let source = r#"
x = 10
x += 5
    "#;
    let result = analyze(source);
    assert!(
        result.is_ok(),
        "Augmented assignment should work with defined variable"
    );
}

#[test]
fn test_collect_augmented_assignment_undefined() {
    let source = "x += 5";
    let result = analyze(source);
    // Augmented assignment requires variable to be defined (x += 5 is x = x + 5)
    assert!(
        result.is_err(),
        "Augmented assignment should error if variable undefined"
    );
    let errors = result.unwrap_err();
    assert!(matches!(errors[0], SemanticError::UndefinedVariable { .. }));
}

#[test]
fn test_collect_function_definition() {
    let source = r#"
def add(a, b):
    return a + b
    "#;
    let result = analyze(source);
    assert!(
        result.is_ok(),
        "Function definition should not produce errors"
    );
}

#[test]
fn test_collect_function_with_defaults() {
    let source = r#"
def greet(name, greeting="Hello"):
    return greeting + " " + name
    "#;
    let result = analyze(source);
    assert!(
        result.is_ok(),
        "Function with default parameters should work"
    );
}

#[test]
fn test_collect_nested_function() {
    let source = r#"
def outer():
    def inner():
        return 42
    return 0
    "#;
    let result = analyze(source);
    assert!(
        result.is_ok(),
        "Nested function definition should work: {:?}",
        result
    );
}

#[test]
#[ignore = "TODO: Implement nested function scope resolution"]
fn test_call_nested_function() {
    let source = r#"
def outer():
    def inner():
        return 42
    return inner()
    "#;
    let result = analyze(source);
    assert!(
        result.is_ok(),
        "Calling nested functions should work: {:?}",
        result
    );
}

#[test]
fn test_collect_function_parameters() {
    let source = r#"
def func(a, b, c):
    return a
    "#;
    let result = analyze(source);
    assert!(
        result.is_ok(),
        "Function parameters should be in scope: {:?}",
        result
    );
}

#[test]
fn test_collect_class_definition() {
    let source = r#"
class Point:
    def __init__(self, x, y):
        self.x = x
        self.y = y
    "#;
    let result = analyze(source);
    assert!(result.is_ok(), "Class definition should not produce errors");
}

#[test]
fn test_collect_empty_class() {
    let source = r#"
class Empty:
    pass
    "#;
    let result = analyze(source);
    assert!(result.is_ok(), "Empty class should work");
}

#[test]
fn test_collect_class_with_methods() {
    let source = r#"
class Calculator:
    def add(self, a, b):
        return a + b
    
    def subtract(self, a, b):
        return a - b
    "#;
    let result = analyze(source);
    assert!(result.is_ok(), "Class with multiple methods should work");
}

#[test]
fn test_collect_import() {
    let source = "import math";
    let result = analyze(source);
    assert!(result.is_ok(), "Import statement should define module");
}

#[test]
fn test_collect_import_with_alias() {
    let source = "import numpy as np";
    let result = analyze(source);
    assert!(result.is_ok(), "Import with alias should work");
}

#[test]
fn test_collect_from_import() {
    let source = "from math import pi, e";
    let result = analyze(source);
    assert!(result.is_ok(), "From import should define symbols");
}

#[test]
fn test_collect_from_import_with_alias() {
    let source = "from math import pi as PI";
    let result = analyze(source);
    assert!(result.is_ok(), "From import with alias should work");
}

#[test]
#[ignore = "Investigate hanging issue"]
fn test_collect_for_loop_variable() {
    let source = r#"
items = [1, 2, 3]
for item in items:
    print(item)
    "#;
    let result = analyze(source);
    // Will have error for undefined 'print'
    assert!(result.is_err());
    let errors = result.unwrap_err();
    // Should only complain about 'print', not 'item'
    assert_eq!(errors.len(), 1);
    assert!(
        matches!(errors[0], SemanticError::UndefinedVariable { ref name, .. } if name == "print")
    );
}

#[test]
#[ignore = "Investigate hanging issue"]
fn test_collect_for_loop_defines_variable() {
    let source = r#"
items = [1, 2, 3]
for x in items:
    pass
y = x
    "#;
    let result = analyze(source);
    assert!(
        result.is_ok(),
        "For loop variable should be available after loop"
    );
}

#[test]
fn test_collect_with_statement_variable() {
    let source = r#"
file = open("test.txt")
with file as f:
    data = f.read()
    "#;
    let result = analyze(source);
    // open is now a built-in function, so no errors expected
    // Method calls like f.read() don't generate errors (they return Unknown)
    assert!(
        result.is_ok(),
        "Should succeed - 'open' is built-in and 'f' is defined by with statement"
    );
}

#[test]
fn test_collect_except_handler_variable() {
    let source = r#"
try:
    x = 1
except Exception as e:
    print(e)
    "#;
    let result = analyze(source);
    // Will have error for 'Exception' and 'print'
    assert!(result.is_err());
    let errors = result.unwrap_err();
    // Check that 'e' is not in the errors
    for error in &errors {
        if let SemanticError::UndefinedVariable { name, .. } = error {
            assert_ne!(name, "e", "e should be defined by except handler");
        }
    }
}

#[test]
fn test_collect_variables_in_if_block() {
    let source = r#"
x = 10
if x > 5:
    y = 20
z = y
    "#;
    let result = analyze(source);
    assert!(
        result.is_ok(),
        "Variables defined in if block should be available outside"
    );
}

#[test]
fn test_collect_variables_in_while_block() {
    let source = r#"
x = 0
while x:
    y = 42
    x = 0
result = y
    "#;
    let result = analyze(source);
    assert!(
        result.is_ok(),
        "Variables defined in while block should be available outside: {:?}",
        result
    );
}

#[test]
fn test_collect_variables_in_try_block() {
    let source = r#"
try:
    x = 10
except:
    pass
y = x
    "#;
    let result = analyze(source);
    assert!(
        result.is_ok(),
        "Variables defined in try block should be available outside"
    );
}

// ========== FUNCTION PARAMETER TESTS ==========

#[test]
fn test_vararg_parameter() {
    let source = r#"
def func(*args):
    return args
    "#;
    let result = analyze(source);
    assert!(result.is_ok(), "Vararg parameter should be defined");
}

#[test]
fn test_kwarg_parameter() {
    let source = r#"
def func(**kwargs):
    return kwargs
    "#;
    let result = analyze(source);
    assert!(result.is_ok(), "Kwarg parameter should be defined");
}

#[test]
fn test_all_parameter_types() {
    let source = r#"
def func(a, b=5, *args, c, d=10, **kwargs):
    return (a, b, args, c, d, kwargs)
    "#;
    let result = analyze(source);
    assert!(result.is_ok(), "All parameter types should be defined");
}

// ========== REDEFINITION TESTS ==========

#[test]
fn test_redefinition_variable() {
    let source = r#"
x = 5
x = 10
    "#;
    let result = analyze(source);
    // Python allows variable reassignment - this should succeed
    assert!(
        result.is_ok(),
        "Variable reassignment should be allowed: {:?}",
        result
    );
}

#[test]
fn test_redefinition_function() {
    let source = r#"
def foo():
    pass

def foo():
    pass
    "#;
    let result = analyze(source);
    // This should produce a redefinition error
    assert!(
        result.is_err(),
        "Function redefinition should produce error"
    );
    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 1);
    assert!(matches!(errors[0], SemanticError::RedefinedVariable { .. }));
}

#[test]
fn test_redefinition_class() {
    let source = r#"
class Point:
    pass

class Point:
    pass
    "#;
    let result = analyze(source);
    // This should produce a redefinition error
    assert!(result.is_err(), "Class redefinition should produce error");
    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 1);
    assert!(matches!(errors[0], SemanticError::RedefinedVariable { .. }));
}

#[test]
fn test_function_parameter_shadows_variable() {
    let source = r#"
x = 10
def func(x):
    return x * 2
    "#;
    let result = analyze(source);
    assert!(
        result.is_ok(),
        "Function parameter can shadow outer variable"
    );
}
