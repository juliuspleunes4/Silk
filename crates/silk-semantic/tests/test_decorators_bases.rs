//! Tests for decorator and base class expression validation

use silk_parser::Parser;
use silk_semantic::{SemanticAnalyzer, SemanticError};

/// Helper to parse and analyze source code
fn analyze(source: &str) -> Result<(), Vec<SemanticError>> {
    let program = Parser::parse(source).expect("Parser should succeed");

    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program)
}

// ========== FUNCTION DECORATORS ==========

#[test]
fn test_function_decorator_undefined() {
    let source = r#"
@undefined_decorator
def my_function():
    pass
    "#;
    let result = analyze(source);
    assert!(result.is_err(), "Should detect undefined decorator");
    let errors = result.unwrap_err();
    assert!(
        matches!(errors[0], SemanticError::UndefinedVariable { ref name, .. } if name == "undefined_decorator")
    );
}

#[test]
fn test_function_decorator_defined() {
    let source = r#"
def decorator(func):
    return func

@decorator
def my_function():
    pass
    "#;
    let result = analyze(source);
    assert!(
        result.is_ok(),
        "Defined decorator should work: {:?}",
        result
    );
}

#[test]
fn test_function_decorator_forward_reference() {
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
        "Forward reference decorator should work: {:?}",
        result
    );
}

#[test]
fn test_function_decorator_with_call() {
    let source = r#"
def decorator_factory(arg):
    pass

@decorator_factory(42)
def my_function():
    pass
    "#;
    let result = analyze(source);
    assert!(
        result.is_ok(),
        "Decorator with call should work: {:?}",
        result
    );
}

#[test]
fn test_function_decorator_attribute() {
    let source = r#"
class Module:
    pass

module = Module()

@module.decorator
def my_function():
    pass
    "#;
    let result = analyze(source);
    // Note: This will pass because we validate 'module' exists, not that it has a 'decorator' attribute
    // Attribute checking is beyond current scope
    assert!(
        result.is_ok(),
        "Decorator with attribute should validate object: {:?}",
        result
    );
}

#[test]
fn test_function_multiple_decorators() {
    let source = r#"
@decorator1
@decorator2
@decorator3
def my_function():
    pass

def decorator1(func):
    return func

def decorator2(func):
    return func

def decorator3(func):
    return func
    "#;
    let result = analyze(source);
    assert!(
        result.is_ok(),
        "Multiple decorators should work: {:?}",
        result
    );
}

#[test]
fn test_function_multiple_decorators_one_undefined() {
    let source = r#"
def decorator1(func):
    return func

@decorator1
@undefined_decorator
@decorator3
def my_function():
    pass

def decorator3(func):
    return func
    "#;
    let result = analyze(source);
    assert!(
        result.is_err(),
        "Should detect undefined decorator in chain"
    );
    let errors = result.unwrap_err();
    assert!(
        matches!(errors[0], SemanticError::UndefinedVariable { ref name, .. } if name == "undefined_decorator")
    );
}

// ========== CLASS DECORATORS ==========

#[test]
fn test_class_decorator_undefined() {
    let source = r#"
@undefined_decorator
class MyClass:
    pass
    "#;
    let result = analyze(source);
    assert!(result.is_err(), "Should detect undefined class decorator");
    let errors = result.unwrap_err();
    assert!(
        matches!(errors[0], SemanticError::UndefinedVariable { ref name, .. } if name == "undefined_decorator")
    );
}

#[test]
fn test_class_decorator_defined() {
    let source = r#"
def dataclass(cls):
    return cls

@dataclass
class MyClass:
    pass
    "#;
    let result = analyze(source);
    assert!(
        result.is_ok(),
        "Defined class decorator should work: {:?}",
        result
    );
}

#[test]
fn test_class_decorator_forward_reference() {
    let source = r#"
@dataclass
class MyClass:
    pass

def dataclass(cls):
    return cls
    "#;
    let result = analyze(source);
    assert!(
        result.is_ok(),
        "Forward reference class decorator should work: {:?}",
        result
    );
}

// ========== BASE CLASSES ==========

#[test]
fn test_base_class_undefined() {
    let source = r#"
class MyClass(UndefinedBase):
    pass
    "#;
    let result = analyze(source);
    assert!(result.is_err(), "Should detect undefined base class");
    let errors = result.unwrap_err();
    assert!(
        matches!(errors[0], SemanticError::UndefinedVariable { ref name, .. } if name == "UndefinedBase")
    );
}

#[test]
fn test_base_class_defined() {
    let source = r#"
class BaseClass:
    pass

class MyClass(BaseClass):
    pass
    "#;
    let result = analyze(source);
    assert!(
        result.is_ok(),
        "Defined base class should work: {:?}",
        result
    );
}

#[test]
fn test_base_class_forward_reference() {
    let source = r#"
class MyClass(BaseClass):
    pass

class BaseClass:
    pass
    "#;
    let result = analyze(source);
    assert!(
        result.is_ok(),
        "Forward reference base class should work: {:?}",
        result
    );
}

#[test]
fn test_multiple_base_classes() {
    let source = r#"
class Base1:
    pass

class Base2:
    pass

class MyClass(Base1, Base2):
    pass
    "#;
    let result = analyze(source);
    assert!(
        result.is_ok(),
        "Multiple base classes should work: {:?}",
        result
    );
}

#[test]
fn test_multiple_base_classes_one_undefined() {
    let source = r#"
class Base1:
    pass

class MyClass(Base1, UndefinedBase, Base3):
    pass

class Base3:
    pass
    "#;
    let result = analyze(source);
    assert!(result.is_err(), "Should detect undefined base class");
    let errors = result.unwrap_err();
    assert!(
        matches!(errors[0], SemanticError::UndefinedVariable { ref name, .. } if name == "UndefinedBase")
    );
}

#[test]
fn test_base_class_with_attribute() {
    let source = r#"
class Module:
    class Inner:
        pass

module = Module()

class MyClass(module.Inner):
    pass
    "#;
    let result = analyze(source);
    // Note: This validates 'module' exists, not that it has 'Inner' attribute
    assert!(
        result.is_ok(),
        "Base class with attribute should validate object: {:?}",
        result
    );
}

// ========== COMBINED DECORATORS AND BASE CLASSES ==========

#[test]
fn test_decorated_class_with_base() {
    let source = r#"
def decorator(cls):
    return cls

class BaseClass:
    pass

@decorator
class MyClass(BaseClass):
    pass
    "#;
    let result = analyze(source);
    assert!(
        result.is_ok(),
        "Decorated class with base should work: {:?}",
        result
    );
}

#[test]
fn test_decorated_class_undefined_decorator_valid_base() {
    let source = r#"
class BaseClass:
    pass

@undefined_decorator
class MyClass(BaseClass):
    pass
    "#;
    let result = analyze(source);
    assert!(
        result.is_err(),
        "Should detect undefined decorator even with valid base"
    );
    let errors = result.unwrap_err();
    assert!(
        matches!(errors[0], SemanticError::UndefinedVariable { ref name, .. } if name == "undefined_decorator")
    );
}

#[test]
fn test_decorated_class_valid_decorator_undefined_base() {
    let source = r#"
def decorator(cls):
    return cls

@decorator
class MyClass(UndefinedBase):
    pass
    "#;
    let result = analyze(source);
    assert!(
        result.is_err(),
        "Should detect undefined base even with valid decorator"
    );
    let errors = result.unwrap_err();
    assert!(
        matches!(errors[0], SemanticError::UndefinedVariable { ref name, .. } if name == "UndefinedBase")
    );
}

// ========== CLASS KEYWORD ARGUMENTS ==========

#[test]
fn test_class_keyword_metaclass_undefined() {
    let source = r#"
class MyClass(metaclass=UndefinedMeta):
    pass
    "#;
    let result = analyze(source);
    assert!(result.is_err(), "Should detect undefined metaclass");
    let errors = result.unwrap_err();
    assert!(
        matches!(errors[0], SemanticError::UndefinedVariable { ref name, .. } if name == "UndefinedMeta")
    );
}

#[test]
fn test_class_keyword_metaclass_defined() {
    let source = r#"
class MetaClass:
    pass

class MyClass(metaclass=MetaClass):
    pass
    "#;
    let result = analyze(source);
    assert!(
        result.is_ok(),
        "Defined metaclass should work: {:?}",
        result
    );
}

#[test]
fn test_class_keyword_metaclass_forward_reference() {
    let source = r#"
class MyClass(metaclass=MetaClass):
    pass

class MetaClass:
    pass
    "#;
    let result = analyze(source);
    assert!(
        result.is_ok(),
        "Forward reference metaclass should work: {:?}",
        result
    );
}

#[test]
fn test_class_with_base_and_metaclass_both_undefined() {
    let source = r#"
class MyClass(UndefinedBase, metaclass=UndefinedMeta):
    pass
    "#;
    let result = analyze(source);
    assert!(
        result.is_err(),
        "Should detect both undefined base and metaclass"
    );
    let errors = result.unwrap_err();
    // Should have 2 errors
    assert_eq!(errors.len(), 2);
}

#[test]
fn test_class_with_base_and_metaclass_both_defined() {
    let source = r#"
class BaseClass:
    pass

class MetaClass:
    pass

class MyClass(BaseClass, metaclass=MetaClass):
    pass
    "#;
    let result = analyze(source);
    assert!(
        result.is_ok(),
        "Both base and metaclass defined should work: {:?}",
        result
    );
}
