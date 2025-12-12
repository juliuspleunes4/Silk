/// Comprehensive tests for method call tracking in control flow analysis
use silk_parser::Parser;
use silk_semantic::{SemanticAnalyzer, SemanticError};

// Helper function to analyze code and filter out control flow warnings
fn analyze_ignoring_warnings(source: &str) -> Result<(), Vec<SemanticError>> {
    let program = Parser::parse(source).unwrap();
    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&program);
    
    // Filter out unused variable/parameter warnings - we're testing method tracking
    match result {
        Ok(()) => Ok(()),
        Err(errors) => {
            let serious_errors: Vec<_> = errors.into_iter()
                .filter(|e| !matches!(e, 
                    SemanticError::UnusedVariable { .. }
                ))
                .collect();
            
            if serious_errors.is_empty() {
                Ok(())
            } else {
                Err(serious_errors)
            }
        }
    }
}

#[test]
fn test_simple_method_call_tracked() {
    // Basic instance method call should be tracked
    let source = r#"
class Math:
    def double(self, x):
        return x * 2

m = Math()
result = m.double(5)
"#;
    let result = analyze_ignoring_warnings(source);
    
    assert!(result.is_ok(), "Method call should be tracked: {:?}", result.err());
}

#[test]
fn test_multiple_method_calls() {
    // Multiple different methods called should all be tracked
    let source = r#"
class Calculator:
    def add(self, a, b):
        return a + b
    
    def subtract(self, a, b):
        return a - b
    
    def multiply(self, a, b):
        return a * b

calc = Calculator()
r1 = calc.add(5, 3)
r2 = calc.subtract(10, 4)
r3 = calc.multiply(6, 7)
"#;
    let result = analyze_ignoring_warnings(source);
    
    assert!(result.is_ok(), "All method calls should be tracked: {:?}", result.err());
}

#[test]
fn test_unused_method_detected() {
    // Method defined but never called should still be detected as unused
    let source = r#"
class Service:
    def used_method(self):
        return "used"
    
    def unused_method(self):
        return "unused"

s = Service()
result = s.used_method()
"#;
    let result = analyze_ignoring_warnings(source);
    
    // Should detect unused_method as unused
    assert!(result.is_err(), "Unused method should be detected");
    let errors = result.unwrap_err();
    assert!(errors.iter().any(|e| matches!(e, SemanticError::UnusedFunction { name, .. } if name == "unused_method")),
        "Should detect unused_method, got: {:?}", errors);
}

#[test]
fn test_method_called_in_different_scopes() {
    // Method called in nested function should be tracked
    let source = r#"
class Processor:
    def process(self, data):
        return data.upper()

def main():
    p = Processor()
    result = p.process("hello")
    return result

output = main()
"#;
    let result = analyze_ignoring_warnings(source);
    
    assert!(result.is_ok(), "Method call in nested function should be tracked: {:?}", result.err());
}

#[test]
fn test_chained_method_calls() {
    // Chained method calls should all be tracked
    let source = r#"
class Builder:
    def set_x(self, x):
        self.x = x
        return self
    
    def set_y(self, y):
        self.y = y
        return self
    
    def build(self):
        return (self.x, self.y)

result = Builder().set_x(10).set_y(20).build()
"#;
    let result = analyze_ignoring_warnings(source);
    
    assert!(result.is_ok(), "Chained method calls should all be tracked: {:?}", result.err());
}

#[test]
fn test_method_call_with_arguments() {
    // Method calls with various argument types
    let source = r#"
class Handler:
    def handle(self, event, priority):
        return f"{event}:{priority}"

h = Handler()
r1 = h.handle("click", 1)
r2 = h.handle("submit", 2)
"#;
    let result = analyze_ignoring_warnings(source);
    
    assert!(result.is_ok(), "Method calls with arguments should be tracked: {:?}", result.err());
}

#[test]
fn test_method_in_conditional() {
    // Method called inside conditional should be tracked
    let source = r#"
class Validator:
    def validate(self, value):
        return value > 0

v = Validator()
if v.validate(10):
    result = "valid"
else:
    result = "invalid"
"#;
    let result = analyze_ignoring_warnings(source);
    
    assert!(result.is_ok(), "Method in conditional should be tracked: {:?}", result.err());
}

#[test]
fn test_method_in_loop() {
    // Method called in loop should be tracked
    let source = r#"
class Iterator:
    def next_value(self):
        return 42

it = Iterator()
for i in range(3):
    value = it.next_value()
"#;
    let result = analyze_ignoring_warnings(source);
    
    assert!(result.is_ok(), "Method in loop should be tracked: {:?}", result.err());
}

#[test]
fn test_multiple_instances_same_method() {
    // Same method called on different instances
    let source = r#"
class Counter:
    def increment(self):
        return 1

c1 = Counter()
c2 = Counter()
r1 = c1.increment()
r2 = c2.increment()
"#;
    let result = analyze_ignoring_warnings(source);
    
    assert!(result.is_ok(), "Method on multiple instances should be tracked: {:?}", result.err());
}

#[test]
fn test_method_as_return_value() {
    // Method call result used as return value
    let source = r#"
class Formatter:
    def format_text(self, text):
        return text.upper()

def process():
    f = Formatter()
    return f.format_text("hello")

result = process()
"#;
    let result = analyze_ignoring_warnings(source);
    
    assert!(result.is_ok(), "Method call as return value should be tracked: {:?}", result.err());
}
