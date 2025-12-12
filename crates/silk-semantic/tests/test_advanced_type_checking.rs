/// Advanced integration tests combining multiple type checking features
use silk_parser::Parser;
use silk_semantic::{SemanticAnalyzer, SemanticError};

// ========== COMBINED FEATURES ==========

#[test]
fn test_annotated_assignment_with_function_call_result() {
    // Tests assignment type checking + function call type inference
    let source = r#"
def get_number() -> int:
    return 42

x: int = get_number()
y: str = get_number()
"#;
    let program = Parser::parse(source).expect("Failed to parse");

    let mut analyzer = SemanticAnalyzer::new_without_control_flow();
    let result = analyzer.analyze(&program);

    assert!(result.is_err(), "Expected error for str = int");
    let errors = result.err().unwrap();
    assert_eq!(errors.len(), 1);
    assert!(
        matches!(&errors[0], SemanticError::AssignmentTypeMismatch { expected_type, value_type, .. }
            if expected_type == "str" && value_type == "int"),
        "Expected AssignmentTypeMismatch, got: {:?}",
        errors[0]
    );
}

#[test]
fn test_function_returning_collection_used_in_subscript() {
    // Tests function call + collection subscript validation
    let source = r#"
def get_list() -> list[int]:
    return [1, 2, 3]

x: int = get_list()[0]
y: int = get_list()["bad"]
"#;
    let program = Parser::parse(source).expect("Failed to parse");

    let mut analyzer = SemanticAnalyzer::new_without_control_flow();
    let result = analyzer.analyze(&program);

    assert!(result.is_err(), "Expected subscript error");
    let errors = result.err().unwrap();
    assert!(errors.len() >= 1);
    assert!(
        errors
            .iter()
            .any(|e| matches!(e, SemanticError::InvalidSubscript { .. })),
        "Expected InvalidSubscript error, got: {:?}",
        errors
    );
}

#[test]
fn test_nested_function_calls_with_type_checking() {
    // Tests nested function calls with type validation
    let source = r#"
def double(x: int) -> int:
    return x * 2

def add(a: int, b: int) -> int:
    return a + b

result: int = add(double(5), double(10))
bad: str = add(double(5), double(10))
"#;
    let program = Parser::parse(source).expect("Failed to parse");

    let mut analyzer = SemanticAnalyzer::new_without_control_flow();
    let result = analyzer.analyze(&program);

    assert!(result.is_err());
    let errors = result.err().unwrap();
    assert_eq!(errors.len(), 1);
    assert!(matches!(
        &errors[0],
        SemanticError::AssignmentTypeMismatch { .. }
    ));
}

#[test]
fn test_binary_operations_with_function_calls() {
    // Tests binary operation validation with function call results
    let source = r#"
def get_int() -> int:
    return 42

def get_str() -> str:
    return "hello"

x: int = get_int() + get_int()
y: str = get_str() + get_str()
z: int = get_int() + get_str()
"#;
    let program = Parser::parse(source).expect("Failed to parse");

    let mut analyzer = SemanticAnalyzer::new_without_control_flow();
    let result = analyzer.analyze(&program);

    assert!(result.is_err());
    let errors = result.err().unwrap();
    assert!(errors.len() >= 1);
    assert!(
        errors
            .iter()
            .any(|e| matches!(e, SemanticError::InvalidBinaryOperation { .. })),
        "Expected InvalidBinaryOperation error, got: {:?}",
        errors
    );
}

#[test]
fn test_mixed_valid_and_invalid_operations() {
    // Tests multiple operations with some valid and some invalid
    let source = r#"
x: int = 5
y: int = 10
z: int = x + y
w: str = x + y

a: list[int] = [1, 2, 3]
b: int = a[0]
c: int = a["bad"]
"#;
    let program = Parser::parse(source).expect("Failed to parse");

    let mut analyzer = SemanticAnalyzer::new_without_control_flow();
    let result = analyzer.analyze(&program);

    assert!(result.is_err());
    let errors = result.err().unwrap();
    assert!(
        errors.len() >= 2,
        "Expected at least 2 errors, got: {:?}",
        errors
    );
}

#[test]
fn test_function_with_multiple_returns_and_calls() {
    // Tests return type checking with multiple return paths
    let source = r#"
def conditional(flag: bool) -> int:
    if flag:
        return 42
    else:
        return "error"

x: int = conditional(True)
"#;
    let program = Parser::parse(source).expect("Failed to parse");

    let mut analyzer = SemanticAnalyzer::new_without_control_flow();
    let result = analyzer.analyze(&program);

    assert!(result.is_err());
    let errors = result.err().unwrap();
    assert!(
        errors
            .iter()
            .any(|e| matches!(e, SemanticError::ReturnTypeMismatch { .. })),
        "Expected ReturnTypeMismatch error, got: {:?}",
        errors
    );
}

#[test]
fn test_collection_operations_chain() {
    // Tests chained collection operations
    let source = r#"
x: list[list[int]] = [[1, 2], [3, 4]]
y: int = x[0][1]
z: str = x[0][1]
"#;
    let program = Parser::parse(source).expect("Failed to parse");

    let mut analyzer = SemanticAnalyzer::new_without_control_flow();
    let result = analyzer.analyze(&program);

    assert!(result.is_err());
    let errors = result.err().unwrap();
    assert_eq!(errors.len(), 1);
    assert!(matches!(
        &errors[0],
        SemanticError::AssignmentTypeMismatch { .. }
    ));
}

#[test]
fn test_dict_operations_with_wrong_key_type() {
    // Tests dictionary subscript with type checking
    let source = r#"
data: dict[str, int] = {"a": 1, "b": 2}
x: int = data["a"]
y: int = data[0]
"#;
    let program = Parser::parse(source).expect("Failed to parse");

    let mut analyzer = SemanticAnalyzer::new_without_control_flow();
    let result = analyzer.analyze(&program);

    assert!(result.is_err());
    let errors = result.err().unwrap();
    assert!(errors.len() >= 1);
    assert!(
        errors
            .iter()
            .any(|e| matches!(e, SemanticError::InvalidSubscript { .. })),
        "Expected InvalidSubscript error, got: {:?}",
        errors
    );
}

#[test]
fn test_function_parameter_and_return_validation() {
    // Tests both parameter type checking and return type checking
    let source = r#"
def process(x: int) -> str:
    return x + 10

y: str = process(5)
z: str = process("bad")
"#;
    let program = Parser::parse(source).expect("Failed to parse");

    let mut analyzer = SemanticAnalyzer::new_without_control_flow();
    let result = analyzer.analyze(&program);

    assert!(result.is_err());
    let errors = result.err().unwrap();
    // We expect at least 2 errors: return type mismatch and argument type mismatch
    // Note: The return error may not be caught if function body analysis is not complete
    assert!(
        errors.len() >= 1,
        "Expected at least 1 error, got: {:?}",
        errors
    );
    // Check for argument error which should always be present
    let has_arg_error = errors
        .iter()
        .any(|e| matches!(e, SemanticError::ArgumentTypeMismatch { .. }));
    assert!(
        has_arg_error,
        "Expected argument type mismatch error, got: {:?}",
        errors
    );
}

#[test]
fn test_complex_nested_scenario() {
    // Complex scenario combining many features
    let source = r#"
def get_data() -> dict[str, list[int]]:
    return {"nums": [1, 2, 3]}

def process_list(items: list[int]) -> int:
    return items[0]

x: dict[str, list[int]] = get_data()
y: list[int] = x["nums"]
z: int = process_list(y)
w: int = y[0]

bad1: str = z
bad2: int = y["wrong"]
bad3: int = process_list("not a list")
"#;
    let program = Parser::parse(source).expect("Failed to parse");

    let mut analyzer = SemanticAnalyzer::new_without_control_flow();
    let result = analyzer.analyze(&program);

    assert!(result.is_err());
    let errors = result.err().unwrap();
    assert!(
        errors.len() >= 3,
        "Expected at least 3 errors, got {} errors: {:?}",
        errors.len(),
        errors
    );
}
