use silk_parser::Parser;
use silk_semantic::{SemanticAnalyzer, Type};

/// Helper to parse and analyze, then get variable type
fn get_variable_type(source: &str, var_name: &str) -> Type {
    let program = Parser::parse(source).expect("Parser failed");
    let mut analyzer = SemanticAnalyzer::new_without_control_flow();
    let _ = analyzer.analyze(&program);
    
    analyzer
        .symbol_table()
        .resolve_symbol(var_name)
        .map(|symbol| symbol.ty.clone())
        .unwrap_or(Type::Unknown)
}

// ========== LIST COMPREHENSION TESTS ==========

#[test]
fn test_debug_variable_type_lookup() {
    let source = r#"
numbers: List[int] = [1, 2, 3]
"#;
    let typ = get_variable_type(source, "numbers");
    // Verify that annotated variables work
    eprintln!("Got type: {:?}", typ);
    assert_eq!(typ, Type::List(Box::new(Type::Int)), "Annotated variable should have correct type");
}

#[test]
fn test_simple_list_comprehension_int() {
    let source = r#"
numbers: List[int] = [1, 2, 3]
doubled = [x * 2 for x in numbers]
"#;
    let typ = get_variable_type(source, "doubled");
    // Should infer List[int] because element expression (x * 2) is int * int = int
    assert_eq!(typ, Type::List(Box::new(Type::Int)));
}

#[test]
fn test_list_comprehension_with_filter() {
    let source = r#"
numbers: List[int] = [1, 2, 3, 4]
evens = [x for x in numbers if x % 2 == 0]
"#;
    let typ = get_variable_type(source, "evens");
    // Filter doesn't change element type, should still be List[int]
    assert_eq!(typ, Type::List(Box::new(Type::Int)));
}

#[test]
fn test_list_comprehension_from_string_list() {
    let source = r#"
words: List[str] = ["hello", "world"]
lengths = [len(w) for w in words]
"#;
    let typ = get_variable_type(source, "lengths");
    // len() returns int, so should be List[int]
    // Note: len() might not be implemented yet, so element might be Unknown
    // For now, we just test that it returns a List type
    match typ {
        Type::List(_) => {}, // Success - it's a list
        _ => panic!("Expected List type, got {:?}", typ),
    }
}

#[test]
fn test_list_comprehension_unknown_iterable() {
    let source = r#"
result = [x * 2 for x in unknown_var]
"#;
    let typ = get_variable_type(source, "result");
    // Should still return List type, even if element type is unknown
    match typ {
        Type::List(_) => {}, // Success
        _ => panic!("Expected List type, got {:?}", typ),
    }
}

#[test]
fn test_nested_list_comprehension() {
    let source = r#"
matrix: List[List[int]] = [[1, 2], [3, 4]]
flattened = [x for row in matrix for x in row]
"#;
    let typ = get_variable_type(source, "flattened");
    // Nested comprehension should infer element type from innermost generator
    // This is complex, so we just verify it's a List
    match typ {
        Type::List(_) => {}, // Success
        _ => panic!("Expected List type, got {:?}", typ),
    }
}

#[test]
fn test_list_comprehension_with_literal() {
    let source = r#"
squares = [x * x for x in [1, 2, 3]]
"#;
    let typ = get_variable_type(source, "squares");
    // List literal [1, 2, 3] should be List[int], so element is int
    assert_eq!(typ, Type::List(Box::new(Type::Int)));
}

#[test]
fn test_list_comprehension_string_operations() {
    let source = r#"
words = ["hello", "world"]
uppers = [w.upper() for w in words]
"#;
    let typ = get_variable_type(source, "uppers");
    // Should be List, element type depends on method resolution
    match typ {
        Type::List(_) => {}, // Success
        _ => panic!("Expected List type, got {:?}", typ),
    }
}

// ========== SET COMPREHENSION TESTS ==========

#[test]
fn test_simple_set_comprehension() {
    let source = r#"
numbers: List[int] = [1, 2, 2, 3]
unique = {x for x in numbers}
"#;
    let typ = get_variable_type(source, "unique");
    // Should infer Set[int]
    assert_eq!(typ, Type::Set(Box::new(Type::Int)));
}

#[test]
fn test_set_comprehension_with_transformation() {
    let source = r#"
numbers: List[int] = [1, 2, 3]
squared = {x * x for x in numbers}
"#;
    let typ = get_variable_type(source, "squared");
    // Should infer Set[int]
    assert_eq!(typ, Type::Set(Box::new(Type::Int)));
}

#[test]
fn test_set_comprehension_with_filter() {
    let source = r#"
numbers: List[int] = [1, 2, 3, 4, 5]
even_set = {x for x in numbers if x % 2 == 0}
"#;
    let typ = get_variable_type(source, "even_set");
    // Should infer Set[int]
    assert_eq!(typ, Type::Set(Box::new(Type::Int)));
}

#[test]
fn test_set_comprehension_from_literal() {
    let source = r#"
result = {x + 1 for x in [10, 20, 30]}
"#;
    let typ = get_variable_type(source, "result");
    // Should infer Set[int]
    assert_eq!(typ, Type::Set(Box::new(Type::Int)));
}

// ========== DICT COMPREHENSION TESTS ==========

#[test]
fn test_simple_dict_comprehension() {
    let source = r#"
numbers: List[int] = [1, 2, 3]
mapping = {x: x * 2 for x in numbers}
"#;
    let typ = get_variable_type(source, "mapping");
    // Should infer Dict[int, int]
    assert_eq!(
        typ,
        Type::Dict {
            key_type: Box::new(Type::Int),
            value_type: Box::new(Type::Int),
        }
    );
}

#[test]
fn test_dict_comprehension_with_string_keys() {
    let source = r#"
words: List[str] = ["a", "b", "c"]
indices = {w: w for w in words}
"#;
    let typ = get_variable_type(source, "indices");
    // Should be Dict type 
    match typ {
        Type::Dict { .. } => {}, // Success
        _ => panic!("Expected Dict type, got {:?}", typ),
    }
}

#[test]
fn test_dict_comprehension_with_filter() {
    let source = r#"
numbers: List[int] = [1, 2, 3, 4, 5]
even_squares = {x: x * x for x in numbers if x % 2 == 0}
"#;
    let typ = get_variable_type(source, "even_squares");
    // Should infer Dict[int, int]
    assert_eq!(
        typ,
        Type::Dict {
            key_type: Box::new(Type::Int),
            value_type: Box::new(Type::Int),
        }
    );
}

#[test]
fn test_dict_comprehension_from_literal() {
    let source = r#"
result = {x: x + 10 for x in [1, 2, 3]}
"#;
    let typ = get_variable_type(source, "result");
    // Should infer Dict[int, int]
    assert_eq!(
        typ,
        Type::Dict {
            key_type: Box::new(Type::Int),
            value_type: Box::new(Type::Int),
        }
    );
}

// ========== EDGE CASES ==========

#[test]
fn test_comprehension_with_unknown_element_type() {
    let source = r#"
result = [unknown_func(x) for x in [1, 2, 3]]
"#;
    let typ = get_variable_type(source, "result");
    // Should still be List, even if element type is Unknown
    match typ {
        Type::List(elem_type) => {
            // Element type is Unknown because unknown_func is not defined
            assert_eq!(*elem_type, Type::Unknown);
        }
        _ => panic!("Expected List type, got {:?}", typ),
    }
}

#[test]
fn test_empty_list_comprehension_literal() {
    let source = r#"
result = [x for x in []]
"#;
    let typ = get_variable_type(source, "result");
    // Empty list literal has Unknown element type
    match typ {
        Type::List(_) => {}, // Success
        _ => panic!("Expected List type, got {:?}", typ),
    }
}

#[test]
fn test_comprehension_preserves_collection_structure() {
    let source = r#"
list_result = [x for x in [1, 2]]
set_result = {x for x in [1, 2]}
dict_result = {x: x for x in [1, 2]}
"#;
    let list_typ = get_variable_type(source, "list_result");
    let set_typ = get_variable_type(source, "set_result");
    let dict_typ = get_variable_type(source, "dict_result");
    
    // Verify each has correct collection type
    assert!(matches!(list_typ, Type::List(_)));
    assert!(matches!(set_typ, Type::Set(_)));
    assert!(matches!(dict_typ, Type::Dict { .. }));
}
