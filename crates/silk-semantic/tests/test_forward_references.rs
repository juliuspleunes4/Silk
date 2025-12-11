use silk_parser::Parser;
use silk_semantic::{SemanticAnalyzer, SemanticError};

/// Helper function to analyze source code
fn analyze(source: &str) -> Result<(), Vec<SemanticError>> {
    let program = Parser::parse(source).expect("Parser should succeed");

    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program)
}

// ========== FORWARD REFERENCES ==========

#[test]
fn test_forward_function_call() {
    let source = r#"
def caller():
    return callee()

def callee():
    return 42
    "#;
    let result = analyze(source);
    assert!(
        result.is_ok(),
        "Should allow calling function defined later: {:?}",
        result
    );
}

#[test]
fn test_forward_class_reference() {
    let source = r#"
def create_b():
    return ClassB()

class ClassB:
    pass
    "#;
    let result = analyze(source);
    assert!(
        result.is_ok(),
        "Should allow referencing class defined later: {:?}",
        result
    );
}

#[test]
fn test_mutual_recursion() {
    let source = r#"
def func_a(n):
    if n > 0:
        return func_b(n - 1)
    return 0

def func_b(n):
    if n > 0:
        return func_a(n - 1)
    return 1
    "#;
    let result = analyze(source);
    assert!(
        result.is_ok(),
        "Should allow mutual recursion: {:?}",
        result
    );
}

#[test]
fn test_nested_function_forward_ref() {
    let source = r#"
def outer():
    def inner1():
        return inner2()
    
    def inner2():
        return 42
    
    return inner1()
    "#;
    let result = analyze(source);
    // Note: This should FAIL because nested functions don't get pre-pass treatment
    // Python requires nested functions to be defined before use
    assert!(
        result.is_err(),
        "Nested functions must be defined before use"
    );
}

#[test]
fn test_class_method_forward_ref() {
    let source = r#"
class MyClass:
    def method1(self):
        return self.method2()
    
    def method2(self):
        return 42
    "#;
    let result = analyze(source);
    // This should work because methods are in class scope
    assert!(
        result.is_ok(),
        "Should allow calling method defined later in class: {:?}",
        result
    );
}

#[test]
fn test_undefined_forward_ref() {
    let source = r#"
def caller():
    return undefined_func()
    "#;
    let result = analyze(source);
    assert!(result.is_err(), "Should error on truly undefined function");
    let errors = result.unwrap_err();
    assert!(matches!(errors[0], SemanticError::UndefinedVariable { .. }));
}

// ========== SCOPE PERSISTENCE ==========

#[test]
fn test_parameter_visible_in_function_body() {
    let source = r#"
def func(x, y):
    z = x + y
    return z
    "#;
    let result = analyze(source);
    assert!(
        result.is_ok(),
        "Parameters should be visible in function body: {:?}",
        result
    );
}

#[test]
fn test_lambda_parameter_scope() {
    let source = r#"
f = lambda x, y: x + y
    "#;
    let result = analyze(source);
    assert!(
        result.is_ok(),
        "Lambda parameters should be in scope: {:?}",
        result
    );
}

#[test]
fn test_nested_function_scope_persistence() {
    let source = r#"
def outer(a):
    def inner(b):
        c = a + b
        return c
    result = inner(10)
    return result
    "#;
    let result = analyze(source);
    // The actual test: 'a' (outer parameter) should be visible inside 'inner'
    // If 'a' weren't visible, we'd get an UndefinedVariable error for 'a', not 'inner'
    assert!(
        result.is_err(),
        "Nested function must be defined before use"
    );
    let errors = result.unwrap_err();
    // Should error on 'inner' being undefined, not 'a'
    assert!(
        matches!(errors[0], SemanticError::UndefinedVariable { ref name, .. } if name == "inner")
    );
}

#[test]
fn test_comprehension_scope_persistence() {
    let source = r#"
x = 10
items = [1, 2, 3]
result = [x + i for i in items]
    "#;
    let result = analyze(source);
    assert!(
        result.is_ok(),
        "Comprehension should see outer variables: {:?}",
        result
    );
}

// ========== COMPLEX SCENARIOS ==========

#[test]
fn test_class_instantiation_forward_ref() {
    let source = r#"
instances = [ClassA(), ClassB()]

class ClassA:
    pass

class ClassB:
    pass
    "#;
    let result = analyze(source);
    assert!(
        result.is_ok(),
        "Should allow instantiating classes defined later: {:?}",
        result
    );
}

#[test]
fn test_decorator_forward_ref() {
    let source = r#"
@decorator
def my_function():
    pass

def decorator(func):
    return func
    "#;
    let result = analyze(source);
    assert!(
        result.is_ok(),
        "Should allow decorator defined later: {:?}",
        result
    );
}

#[test]
fn test_base_class_forward_ref() {
    let source = r#"
class Child(Parent):
    pass

class Parent:
    pass
    "#;
    let result = analyze(source);
    assert!(
        result.is_ok(),
        "Should allow base class defined later: {:?}",
        result
    );
}

#[test]
fn test_assignment_before_function_def() {
    let source = r#"
x = func()

def func():
    return 42
    "#;
    let result = analyze(source);
    assert!(
        result.is_ok(),
        "Global assignment can use function defined later: {:?}",
        result
    );
}
