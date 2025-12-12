//! Tests for type inference from literals

use silk_parser::Parser;
use silk_semantic::{SemanticAnalyzer, Type};

#[test]
fn test_integer_literal_type() {
    let source = "x = 42";
    let program = Parser::parse(source).unwrap();

    let mut analyzer = SemanticAnalyzer::new_without_control_flow();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("x").unwrap();
    assert_eq!(symbol.ty, Type::Int);
}

#[test]
fn test_float_literal_type() {
    let source = "pi = 3.14";
    let program = Parser::parse(source).unwrap();

    let mut analyzer = SemanticAnalyzer::new_without_control_flow();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("pi").unwrap();
    assert_eq!(symbol.ty, Type::Float);
}

#[test]
fn test_string_literal_type() {
    let source = "name = 'hello'";
    let program = Parser::parse(source).unwrap();

    let mut analyzer = SemanticAnalyzer::new_without_control_flow();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("name").unwrap();
    assert_eq!(symbol.ty, Type::Str);
}

#[test]
fn test_raw_string_literal_type() {
    let source = r#"pattern = r'\d+'"#;
    let program = Parser::parse(source).unwrap();

    let mut analyzer = SemanticAnalyzer::new_without_control_flow();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("pattern").unwrap();
    assert_eq!(symbol.ty, Type::Str);
}

#[test]
fn test_fstring_literal_type() {
    let source = r#"name = 'test'
msg = f'hello {name}'"#;
    let program = Parser::parse(source).unwrap();

    let mut analyzer = SemanticAnalyzer::new_without_control_flow();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("msg").unwrap();
    assert_eq!(symbol.ty, Type::Str);
}

#[test]
fn test_boolean_true_type() {
    let source = "flag = True";
    let program = Parser::parse(source).unwrap();

    let mut analyzer = SemanticAnalyzer::new_without_control_flow();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("flag").unwrap();
    assert_eq!(symbol.ty, Type::Bool);
}

#[test]
fn test_boolean_false_type() {
    let source = "flag = False";
    let program = Parser::parse(source).unwrap();

    let mut analyzer = SemanticAnalyzer::new_without_control_flow();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("flag").unwrap();
    assert_eq!(symbol.ty, Type::Bool);
}

#[test]
fn test_none_literal_type() {
    let source = "result = None";
    let program = Parser::parse(source).unwrap();

    let mut analyzer = SemanticAnalyzer::new_without_control_flow();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("result").unwrap();
    assert_eq!(symbol.ty, Type::None);
}

#[test]
fn test_multiple_assignments_different_types() {
    let source = r#"
x = 42
y = 3.14
z = 'hello'
flag = True
empty = None
"#;
    let program = Parser::parse(source).unwrap();

    let mut analyzer = SemanticAnalyzer::new_without_control_flow();
    analyzer.analyze(&program).unwrap();

    assert_eq!(
        analyzer.symbol_table().resolve_symbol("x").unwrap().ty,
        Type::Int
    );
    assert_eq!(
        analyzer.symbol_table().resolve_symbol("y").unwrap().ty,
        Type::Float
    );
    assert_eq!(
        analyzer.symbol_table().resolve_symbol("z").unwrap().ty,
        Type::Str
    );
    assert_eq!(
        analyzer.symbol_table().resolve_symbol("flag").unwrap().ty,
        Type::Bool
    );
    assert_eq!(
        analyzer.symbol_table().resolve_symbol("empty").unwrap().ty,
        Type::None
    );
}

#[test]
fn test_walrus_operator_type_inference() {
    let source = "if (x := 42):
    pass";
    let program = Parser::parse(source).unwrap();

    let mut analyzer = SemanticAnalyzer::new_without_control_flow();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("x").unwrap();
    assert_eq!(symbol.ty, Type::Int);
}

#[test]
fn test_walrus_operator_string_type() {
    let source = "if (name := 'test'):
    pass";
    let program = Parser::parse(source).unwrap();

    let mut analyzer = SemanticAnalyzer::new_without_control_flow();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("name").unwrap();
    assert_eq!(symbol.ty, Type::Str);
}

#[test]
fn test_variable_reference_preserves_type() {
    let source = r#"
x = 42
y = x
"#;
    let program = Parser::parse(source).unwrap();

    let mut analyzer = SemanticAnalyzer::new_without_control_flow();
    analyzer.analyze(&program).unwrap();

    let x_symbol = analyzer.symbol_table().resolve_symbol("x").unwrap();
    let y_symbol = analyzer.symbol_table().resolve_symbol("y").unwrap();
    assert_eq!(x_symbol.ty, Type::Int);
    assert_eq!(y_symbol.ty, Type::Int);
}

#[test]
fn test_negative_integer_type() {
    let source = "x = -42";
    let program = Parser::parse(source).unwrap();

    let mut analyzer = SemanticAnalyzer::new_without_control_flow();
    analyzer.analyze(&program).unwrap();

    // Negative numbers are parsed as UnaryOp with USub
    // Unary minus on Int returns Int
    let symbol = analyzer.symbol_table().resolve_symbol("x").unwrap();
    assert_eq!(symbol.ty, Type::Int);
}

#[test]
fn test_zero_integer_type() {
    let source = "x = 0";
    let program = Parser::parse(source).unwrap();

    let mut analyzer = SemanticAnalyzer::new_without_control_flow();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("x").unwrap();
    assert_eq!(symbol.ty, Type::Int);
}

#[test]
fn test_large_integer_type() {
    let source = "x = 999999999999";
    let program = Parser::parse(source).unwrap();

    let mut analyzer = SemanticAnalyzer::new_without_control_flow();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("x").unwrap();
    assert_eq!(symbol.ty, Type::Int);
}

#[test]
fn test_scientific_notation_float_type() {
    let source = "x = 1.5e10";
    let program = Parser::parse(source).unwrap();

    let mut analyzer = SemanticAnalyzer::new_without_control_flow();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("x").unwrap();
    assert_eq!(symbol.ty, Type::Float);
}

#[test]
fn test_empty_string_type() {
    let source = "x = ''";
    let program = Parser::parse(source).unwrap();

    let mut analyzer = SemanticAnalyzer::new_without_control_flow();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("x").unwrap();
    assert_eq!(symbol.ty, Type::Str);
}

#[test]
fn test_undefined_variable_reference_gets_unknown_type() {
    let source = r#"
y = x
"#;
    let program = Parser::parse(source).unwrap();

    let mut analyzer = SemanticAnalyzer::new_without_control_flow();
    let result = analyzer.analyze(&program);

    // Should error because x is undefined
    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 1);
    assert!(errors[0].to_string().contains("x"));

    // y is still defined with Unknown type (since x was undefined/Unknown)
    let y_symbol = analyzer.symbol_table().resolve_symbol("y").unwrap();
    assert_eq!(y_symbol.ty, Type::Unknown);
}

#[test]
fn test_int_variable_addition_infers_int_type() {
    let source = r#"
x = 10
y = 20
z = x + y
"#;
    let program = Parser::parse(source).unwrap();

    let mut analyzer = SemanticAnalyzer::new_without_control_flow();
    analyzer.analyze(&program).unwrap();

    // x + y is a binary operation with type inference
    // Int + Int = Int
    let z_symbol = analyzer.symbol_table().resolve_symbol("z").unwrap();
    assert_eq!(z_symbol.ty, Type::Int);
}

#[test]
fn test_function_call_result_gets_unknown_type() {
    let source = r#"
def foo():
    return 42

result = foo()
"#;
    let program = Parser::parse(source).unwrap();

    let mut analyzer = SemanticAnalyzer::new_without_control_flow();
    analyzer.analyze(&program).unwrap();

    // Function without return type annotation returns Unknown
    let result_symbol = analyzer.symbol_table().resolve_symbol("result").unwrap();
    assert_eq!(result_symbol.ty, Type::Unknown);
}

#[test]
fn test_list_literal_gets_list_type() {
    let source = "x = [1, 2, 3]";
    let program = Parser::parse(source).unwrap();

    let mut analyzer = SemanticAnalyzer::new_without_control_flow();
    analyzer.analyze(&program).unwrap();

    // List literals now infer to list[ElementType]
    let symbol = analyzer.symbol_table().resolve_symbol("x").unwrap();
    assert_eq!(symbol.ty, Type::List(Box::new(Type::Int)));
}

#[test]
fn test_dict_literal_gets_dict_type() {
    let source = "x = {'key': 'value'}";
    let program = Parser::parse(source).unwrap();

    let mut analyzer = SemanticAnalyzer::new_without_control_flow();
    analyzer.analyze(&program).unwrap();

    // Dict literals now infer to dict[KeyType, ValueType]
    let symbol = analyzer.symbol_table().resolve_symbol("x").unwrap();
    assert_eq!(
        symbol.ty,
        Type::Dict {
            key_type: Box::new(Type::Str),
            value_type: Box::new(Type::Str)
        }
    );
}

#[test]
fn test_tuple_literal_gets_tuple_type() {
    let source = "x = (1, 2, 3)";
    let program = Parser::parse(source).unwrap();

    let mut analyzer = SemanticAnalyzer::new_without_control_flow();
    analyzer.analyze(&program).unwrap();

    // Tuple literals now infer to tuple[Type1, Type2, ...]
    let symbol = analyzer.symbol_table().resolve_symbol("x").unwrap();
    assert_eq!(
        symbol.ty,
        Type::Tuple(vec![Type::Int, Type::Int, Type::Int])
    );
}

#[test]
fn test_reassignment_updates_type() {
    let source = r#"
x = 42
x = 'hello'
"#;
    let program = Parser::parse(source).unwrap();

    let mut analyzer = SemanticAnalyzer::new_without_control_flow();
    analyzer.analyze(&program).unwrap();

    // After reassignment, x should have the new type
    let symbol = analyzer.symbol_table().resolve_symbol("x").unwrap();
    assert_eq!(symbol.ty, Type::Str);
}

#[test]
fn test_conditional_assignment_uses_last_type() {
    let source = r#"
if True:
    x = 42
else:
    x = 3.14
"#;
    let program = Parser::parse(source).unwrap();

    let mut analyzer = SemanticAnalyzer::new_without_control_flow();
    analyzer.analyze(&program).unwrap();

    // x is defined in both branches, should have the type from the last definition
    let symbol = analyzer.symbol_table().resolve_symbol("x").unwrap();
    assert_eq!(symbol.ty, Type::Float); // Last definition wins in our current implementation
}

#[test]
fn test_ternary_expression_gets_unknown_type() {
    let source = r#"
x = 42 if True else 3.14
"#;
    let program = Parser::parse(source).unwrap();

    let mut analyzer = SemanticAnalyzer::new_without_control_flow();
    analyzer.analyze(&program).unwrap();

    // Ternary expressions aren't literals, should be Unknown for now
    let symbol = analyzer.symbol_table().resolve_symbol("x").unwrap();
    assert_eq!(symbol.ty, Type::Unknown);
}

#[test]
fn test_lambda_gets_unknown_type() {
    let source = "f = lambda x: x + 1";
    let program = Parser::parse(source).unwrap();

    let mut analyzer = SemanticAnalyzer::new_without_control_flow();
    analyzer.analyze(&program).unwrap();

    // Lambda expressions should be Unknown for now
    let symbol = analyzer.symbol_table().resolve_symbol("f").unwrap();
    assert_eq!(symbol.ty, Type::Unknown);
}

#[test]
fn test_comprehension_gets_unknown_type() {
    let source = "squares = [x*x for x in range(10)]";
    let program = Parser::parse(source).unwrap();

    let mut analyzer = SemanticAnalyzer::new_without_control_flow();
    let _ = analyzer.analyze(&program); // May error on undefined 'range'

    // List comprehensions should be Unknown for now
    if let Some(symbol) = analyzer.symbol_table().resolve_symbol("squares") {
        assert_eq!(symbol.ty, Type::Unknown);
    }
}
