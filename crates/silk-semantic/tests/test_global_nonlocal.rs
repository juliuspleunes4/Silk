/// Tests for global and nonlocal statement control flow analysis
use silk_parser::Parser;
use silk_semantic::{SemanticAnalyzer, SemanticError};

// Helper function to analyze code
fn analyze(source: &str) -> Result<(), Vec<SemanticError>> {
    let program = Parser::parse(source).unwrap();
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program)
}

// ========== GLOBAL STATEMENT TESTS ==========

#[test]
fn test_global_marks_variable_as_initialized() {
    let source = r#"
def func():
    global x
    y = x + 1
    "#;
    
    let result = analyze(source);
    // Should not have UninitializedVariable error for x
    if let Err(errors) = &result {
        for error in errors {
            assert!(
                !matches!(error, SemanticError::UninitializedVariable { name, .. } if name == "x"),
                "Variable 'x' should be marked as initialized by global statement"
            );
        }
    }
}

#[test]
fn test_global_multiple_variables() {
    let source = r#"
def func():
    global x, y, z
    result = x + y + z
    "#;
    
    let result = analyze(source);
    // Should not have UninitializedVariable errors for x, y, or z
    if let Err(errors) = &result {
        for error in errors {
            if let SemanticError::UninitializedVariable { name, .. } = error {
                assert!(
                    !["x", "y", "z"].contains(&name.as_str()),
                    "Variables declared global should be marked as initialized, but got error for: {}",
                    name
                );
            }
        }
    }
}

#[test]
fn test_global_allows_assignment_to_outer_scope() {
    let source = r#"
counter = 0

def increment():
    global counter
    counter = counter + 1
    "#;
    
    let result = analyze(source);
    // Should not have errors - global allows modifying outer scope variable
    if let Err(errors) = &result {
        for error in errors {
            assert!(
                !matches!(error, SemanticError::UninitializedVariable { name, .. } if name == "counter"),
                "Global variable 'counter' should not produce initialization error"
            );
        }
    }
}

#[test]
fn test_global_in_nested_function() {
    let source = r#"
value = 42

def outer():
    def inner():
        global value
        result = value * 2
    inner()
    "#;
    
    let result = analyze(source);
    // Should not have UninitializedVariable error for value in inner function
    if let Err(errors) = &result {
        for error in errors {
            assert!(
                !matches!(error, SemanticError::UninitializedVariable { name, .. } if name == "value"),
                "Global 'value' should be accessible in nested function"
            );
        }
    }
}

#[test]
fn test_global_before_assignment() {
    let source = r#"
def setup():
    global config
    config = {"key": "value"}
    "#;
    
    let result = analyze(source);
    // Global declaration before assignment is valid - it will be initialized by assignment
    // Should not fail
    assert!(result.is_ok() || {
        if let Err(errors) = &result {
            errors.iter().all(|e| !matches!(e, SemanticError::UninitializedVariable { .. }))
        } else {
            true
        }
    });
}

// ========== NONLOCAL STATEMENT TESTS ==========

#[test]
fn test_nonlocal_marks_variable_as_initialized() {
    let source = r#"
def outer():
    count = 0
    def inner():
        nonlocal count
        count = count + 1
    inner()
    "#;
    
    let result = analyze(source);
    // Should not have UninitializedVariable error for count in inner function
    if let Err(errors) = &result {
        for error in errors {
            assert!(
                !matches!(error, SemanticError::UninitializedVariable { name, .. } if name == "count"),
                "Nonlocal variable 'count' should be marked as initialized"
            );
        }
    }
}

#[test]
fn test_nonlocal_multiple_variables() {
    let source = r#"
def outer():
    x = 1
    y = 2
    z = 3
    def inner():
        nonlocal x, y, z
        result = x + y + z
    inner()
    "#;
    
    let result = analyze(source);
    // Should not have UninitializedVariable errors for x, y, or z
    if let Err(errors) = &result {
        for error in errors {
            if let SemanticError::UninitializedVariable { name, .. } = error {
                assert!(
                    !["x", "y", "z"].contains(&name.as_str()),
                    "Variables declared nonlocal should be marked as initialized, but got error for: {}",
                    name
                );
            }
        }
    }
}

#[test]
fn test_nonlocal_allows_modification() {
    let source = r#"
def outer():
    total = 0
    def add_one():
        nonlocal total
        total = total + 1
    add_one()
    "#;
    
    let result = analyze(source);
    // Should not have errors - nonlocal allows modifying enclosing scope variable
    if let Err(errors) = &result {
        for error in errors {
            assert!(
                !matches!(error, SemanticError::UninitializedVariable { name, .. } if name == "total"),
                "Nonlocal variable 'total' should not produce initialization error"
            );
        }
    }
}

#[test]
fn test_nonlocal_in_deeply_nested_function() {
    let source = r#"
def level1():
    data = []
    def level2():
        def level3():
            nonlocal data
            data.append(1)
        level3()
    level2()
    "#;
    
    let result = analyze(source);
    // Should not have UninitializedVariable error for data in level3
    if let Err(errors) = &result {
        for error in errors {
            assert!(
                !matches!(error, SemanticError::UninitializedVariable { name, .. } if name == "data"),
                "Nonlocal 'data' should be accessible in deeply nested function"
            );
        }
    }
}

#[test]
fn test_nonlocal_before_use_in_expression() {
    let source = r#"
def outer():
    value = 10
    def inner():
        nonlocal value
        double = value * 2
    inner()
    "#;
    
    let result = analyze(source);
    // Nonlocal declaration makes the variable available for use
    if let Err(errors) = &result {
        for error in errors {
            assert!(
                !matches!(error, SemanticError::UninitializedVariable { name, .. } if name == "value"),
                "Nonlocal variable should be usable immediately after declaration"
            );
        }
    }
}

// ========== COMBINED AND EDGE CASE TESTS ==========

#[test]
fn test_global_and_nonlocal_in_same_function() {
    let source = r#"
global_var = "global"

def outer():
    outer_var = "outer"
    def inner():
        global global_var
        nonlocal outer_var
        result = global_var + outer_var
    inner()
    "#;
    
    let result = analyze(source);
    // Both global and nonlocal should work together
    if let Err(errors) = &result {
        for error in errors {
            if let SemanticError::UninitializedVariable { name, .. } = error {
                assert!(
                    !["global_var", "outer_var"].contains(&name.as_str()),
                    "Global and nonlocal variables should both be initialized, but got error for: {}",
                    name
                );
            }
        }
    }
}

#[test]
fn test_global_doesnt_affect_local_variables() {
    let source = r#"
def func():
    global x
    y = 10
    result = y
    "#;
    
    let result = analyze(source);
    // Local variable y should work normally
    // Should not have errors for y
    if let Err(errors) = &result {
        for error in errors {
            assert!(
                !matches!(error, SemanticError::UninitializedVariable { name, .. } if name == "y"),
                "Local variable 'y' should work independently of global declaration"
            );
        }
    }
}

#[test]
fn test_nonlocal_doesnt_affect_other_variables() {
    let source = r#"
def outer():
    x = 1
    def inner():
        nonlocal x
        y = 2
        result = y
    inner()
    "#;
    
    let result = analyze(source);
    // Local variable y in inner function should work normally
    if let Err(errors) = &result {
        for error in errors {
            assert!(
                !matches!(error, SemanticError::UninitializedVariable { name, .. } if name == "y"),
                "Local variable 'y' should work independently of nonlocal declaration"
            );
        }
    }
}

#[test]
fn test_global_with_conditional_assignment() {
    let source = r#"
config = None

def initialize(use_default):
    global config
    if use_default:
        config = {"mode": "default"}
    else:
        config = {"mode": "custom"}
    "#;
    
    let result = analyze(source);
    // Global variable can be assigned in conditional branches
    // Should not have errors
    assert!(result.is_ok() || {
        if let Err(errors) = &result {
            errors.iter().all(|e| !matches!(e, SemanticError::UninitializedVariable { .. }))
        } else {
            true
        }
    });
}

#[test]
fn test_nonlocal_with_loop_modification() {
    let source = r#"
def outer():
    count = 0
    def increment_multiple(n):
        nonlocal count
        for i in range(n):
            count = count + 1
    increment_multiple(5)
    "#;
    
    let result = analyze(source);
    // Nonlocal variable can be modified in loops
    if let Err(errors) = &result {
        for error in errors {
            assert!(
                !matches!(error, SemanticError::UninitializedVariable { name, .. } if name == "count"),
                "Nonlocal variable should work in loop context"
            );
        }
    }
}

#[test]
fn test_global_statement_at_module_level() {
    let source = r#"
global x
x = 10
    "#;
    
    let result = analyze(source);
    // Global at module level is technically redundant but valid
    // Should not cause errors
    assert!(result.is_ok() || {
        if let Err(errors) = &result {
            // Filter out unused variable warnings
            errors.iter().all(|e| !matches!(e, SemanticError::UninitializedVariable { .. }))
        } else {
            true
        }
    });
}

#[test]
fn test_multiple_global_declarations() {
    let source = r#"
def func1():
    global shared
    shared = 1

def func2():
    global shared
    value = shared + 1
    "#;
    
    let result = analyze(source);
    // Multiple functions can declare same variable as global
    if let Err(errors) = &result {
        for error in errors {
            assert!(
                !matches!(error, SemanticError::UninitializedVariable { name, .. } if name == "shared"),
                "Global variable should work across multiple function declarations"
            );
        }
    }
}

#[test]
fn test_nonlocal_in_lambda_like_construct() {
    let source = r#"
def make_counter():
    count = 0
    def increment():
        nonlocal count
        count = count + 1
        return count
    return increment
    "#;
    
    let result = analyze(source);
    // Nonlocal in closure pattern (common use case)
    if let Err(errors) = &result {
        for error in errors {
            assert!(
                !matches!(error, SemanticError::UninitializedVariable { name, .. } if name == "count"),
                "Nonlocal should work in closure pattern"
            );
        }
    }
}
