use silk_parser::Parser;
use silk_semantic::{SemanticAnalyzer, SemanticError};

// ========== VALID SUBSCRIPT OPERATIONS ==========

#[test]
fn test_valid_list_subscript_int() {
    let source = r#"
x: list[int] = [1, 2, 3]
y: int = x[0]
"#;
    let program = Parser::parse(source).expect("Failed to parse");

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&program);

    assert!(
        result.is_ok(),
        "Expected no errors, got: {:?}",
        result.err()
    );
}

#[test]
fn test_valid_tuple_subscript_int() {
    let source = r#"
x = (1, 2, 3)
y = x[1]
"#;
    let program = Parser::parse(source).expect("Failed to parse");

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&program);

    assert!(
        result.is_ok(),
        "Expected no errors, got: {:?}",
        result.err()
    );
}

#[test]
fn test_valid_dict_subscript_str() {
    let source = r#"
x: dict[str, int] = {"a": 1, "b": 2}
y: int = x["a"]
"#;
    let program = Parser::parse(source).expect("Failed to parse");

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&program);

    assert!(
        result.is_ok(),
        "Expected no errors, got: {:?}",
        result.err()
    );
}

#[test]
fn test_valid_string_subscript_int() {
    let source = r#"
x: str = "hello"
y: str = x[0]
"#;
    let program = Parser::parse(source).expect("Failed to parse");

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&program);

    assert!(
        result.is_ok(),
        "Expected no errors, got: {:?}",
        result.err()
    );
}

#[test]
fn test_subscript_on_unknown_passes() {
    // Unknown types should pass validation (gradual typing)
    let source = r#"
x = some_function()
y = x[0]
"#;
    let program = Parser::parse(source).expect("Failed to parse");

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&program);

    // Should have error for undefined function, but NOT for subscript
    assert!(result.is_err());
    let errors = result.err().unwrap();
    assert!(
        errors
            .iter()
            .all(|e| !matches!(e, SemanticError::InvalidSubscript { .. })),
        "Should not have InvalidSubscript error for Unknown type"
    );
}

// ========== INVALID SUBSCRIPT OPERATIONS ==========

#[test]
fn test_invalid_list_subscript_str() {
    let source = r#"
x: list[int] = [1, 2, 3]
y: int = x["a"]
"#;
    let program = Parser::parse(source).expect("Failed to parse");

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&program);

    assert!(result.is_err(), "Expected InvalidSubscript error");
    let errors = result.err().unwrap();
    // Might have multiple errors if assignment type checking also fails
    assert!(errors.len() >= 1, "Expected at least one error");
    assert!(
        errors.iter().any(
            |e| matches!(e, SemanticError::InvalidSubscript { collection_type, index_type, .. }
            if collection_type.contains("list") && index_type == "str")
        ),
        "Expected InvalidSubscript error for list[str], got: {:?}",
        errors
    );
}

#[test]
fn test_invalid_tuple_subscript_str() {
    let source = r#"
x = (1, 2, 3)
y = x["test"]
"#;
    let program = Parser::parse(source).expect("Failed to parse");

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&program);

    assert!(result.is_err(), "Expected InvalidSubscript error");
    let errors = result.err().unwrap();
    assert_eq!(errors.len(), 1);
    assert!(
        matches!(&errors[0], SemanticError::InvalidSubscript { collection_type, index_type, .. }
            if collection_type.contains("tuple") && index_type == "str"),
        "Expected InvalidSubscript error for tuple[str], got: {:?}",
        errors[0]
    );
}

#[test]
fn test_invalid_dict_subscript_int() {
    let source = r#"
x: dict[str, int] = {"a": 1}
y: int = x[0]
"#;
    let program = Parser::parse(source).expect("Failed to parse");

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&program);

    assert!(
        result.is_err(),
        "Expected InvalidSubscript error, got: {:?}",
        result
    );
    let errors = result.err().unwrap();
    assert!(errors.len() >= 1, "Expected at least one error");
    assert!(
        errors.iter().any(
            |e| matches!(e, SemanticError::InvalidSubscript { collection_type, index_type, .. }
            if collection_type.contains("dict") && index_type == "int")
        ),
        "Expected InvalidSubscript error for dict[str,int] with int index, got: {:?}",
        errors
    );
}

#[test]
fn test_invalid_string_subscript_str() {
    let source = r#"
x: str = "hello"
y = x["test"]
"#;
    let program = Parser::parse(source).expect("Failed to parse");

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&program);

    assert!(result.is_err(), "Expected InvalidSubscript error");
    let errors = result.err().unwrap();
    assert_eq!(errors.len(), 1);
    assert!(
        matches!(&errors[0], SemanticError::InvalidSubscript { collection_type, index_type, .. }
            if collection_type == "str" && index_type == "str"),
        "Expected InvalidSubscript error for str[str], got: {:?}",
        errors[0]
    );
}

#[test]
fn test_invalid_set_subscript() {
    // Sets don't support subscripting at all
    let source = r#"
x: set[int] = {1, 2, 3}
y: int = x[0]
"#;
    let program = Parser::parse(source).expect("Failed to parse");

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&program);

    assert!(
        result.is_err(),
        "Expected InvalidSubscript error, got: {:?}",
        result
    );
    let errors = result.err().unwrap();
    assert!(errors.len() >= 1, "Expected at least one error");
    assert!(
        errors.iter().any(
            |e| matches!(e, SemanticError::InvalidSubscript { collection_type, .. }
            if collection_type.contains("set"))
        ),
        "Expected InvalidSubscript error for set subscript, got: {:?}",
        errors
    );
}

#[test]
fn test_invalid_int_subscript() {
    // Can't subscript an int
    let source = r#"
x: int = 42
y = x[0]
"#;
    let program = Parser::parse(source).expect("Failed to parse");

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&program);

    assert!(result.is_err(), "Expected InvalidSubscript error");
    let errors = result.err().unwrap();
    assert_eq!(errors.len(), 1);
    assert!(
        matches!(&errors[0], SemanticError::InvalidSubscript { collection_type, .. }
            if collection_type == "int"),
        "Expected InvalidSubscript error for int subscript, got: {:?}",
        errors[0]
    );
}

// ========== NESTED AND COMPLEX OPERATIONS ==========

#[test]
fn test_nested_subscripts_valid() {
    let source = r#"
x: list[list[int]] = [[1, 2], [3, 4]]
y: int = x[0][1]
"#;
    let program = Parser::parse(source).expect("Failed to parse");

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&program);

    assert!(
        result.is_ok(),
        "Expected no errors, got: {:?}",
        result.err()
    );
}

#[test]
fn test_nested_subscripts_invalid() {
    let source = r#"
x: list[list[int]] = [[1, 2], [3, 4]]
y: int = x["a"][0]
"#;
    let program = Parser::parse(source).expect("Failed to parse");

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&program);

    assert!(result.is_err(), "Expected InvalidSubscript error");
    let errors = result.err().unwrap();
    assert!(errors.len() >= 1);
    assert!(
        errors
            .iter()
            .any(|e| matches!(e, SemanticError::InvalidSubscript { .. })),
        "Expected at least one InvalidSubscript error, got: {:?}",
        errors
    );
}

#[test]
fn test_subscript_in_expression() {
    let source = r#"
x: list[int] = [1, 2, 3]
y: int = x[0] + x[1]
"#;
    let program = Parser::parse(source).expect("Failed to parse");

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&program);

    assert!(
        result.is_ok(),
        "Expected no errors, got: {:?}",
        result.err()
    );
}

#[test]
fn test_subscript_in_function_call() {
    let source = r#"
def f(x: int) -> int:
    return x

x: list[int] = [1, 2, 3]
y: int = f(x[0])
"#;
    let program = Parser::parse(source).expect("Failed to parse");

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&program);

    assert!(
        result.is_ok(),
        "Expected no errors, got: {:?}",
        result.err()
    );
}

#[test]
fn test_multiple_subscript_errors() {
    let source = r#"
x: list[int] = [1, 2, 3]
y: dict[str, int] = {"a": 1}
a: int = x["bad"]
b: int = y[999]
"#;
    let program = Parser::parse(source).expect("Failed to parse");

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&program);

    assert!(result.is_err(), "Expected errors, got: {:?}", result);
    let errors = result.err().unwrap();
    let subscript_errors: Vec<_> = errors
        .iter()
        .filter(|e| matches!(e, SemanticError::InvalidSubscript { .. }))
        .collect();
    assert!(
        subscript_errors.len() >= 2,
        "Expected at least 2 InvalidSubscript errors, got: {:?}",
        errors
    );
}

// Regression test for dict subscript type compatibility bug
// Previously the check was backwards: key_type.is_compatible_with(index_type)
// This prevented valid numeric widening like using int to index dict[float, str]
#[test]
fn test_dict_subscript_int_to_float_widening() {
    let source = r#"
d: dict[float, str] = {1.5: "hello", 2.5: "world"}
result: str = d[42]
"#;
    let program = Parser::parse(source).expect("Failed to parse");

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&program);

    assert!(
        result.is_ok(),
        "Expected no errors (int should widen to float for dict key), got: {:?}",
        result.err()
    );
}
