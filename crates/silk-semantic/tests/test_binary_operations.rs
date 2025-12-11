//! Tests for binary operation type inference

use silk_parser::Parser;
use silk_semantic::{SemanticAnalyzer, SemanticError, Type};

// ========== ARITHMETIC OPERATIONS - INT ==========

#[test]
fn test_int_add_int() {
    let source = "result = 5 + 3";
    let program = Parser::parse(source).unwrap();
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("result").unwrap();
    assert_eq!(symbol.ty, Type::Int);
}

#[test]
fn test_int_subtract_int() {
    let source = "result = 10 - 3";
    let program = Parser::parse(source).unwrap();
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("result").unwrap();
    assert_eq!(symbol.ty, Type::Int);
}

#[test]
fn test_int_multiply_int() {
    let source = "result = 4 * 5";
    let program = Parser::parse(source).unwrap();
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("result").unwrap();
    assert_eq!(symbol.ty, Type::Int);
}

#[test]
fn test_int_divide_int() {
    let source = "result = 10 / 2";
    let program = Parser::parse(source).unwrap();
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("result").unwrap();
    assert_eq!(symbol.ty, Type::Int);
}

#[test]
fn test_int_floordiv_int() {
    let source = "result = 10 // 3";
    let program = Parser::parse(source).unwrap();
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("result").unwrap();
    assert_eq!(symbol.ty, Type::Int);
}

#[test]
fn test_int_mod_int() {
    let source = "result = 10 % 3";
    let program = Parser::parse(source).unwrap();
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("result").unwrap();
    assert_eq!(symbol.ty, Type::Int);
}

#[test]
fn test_int_pow_int() {
    let source = "result = 2 ** 8";
    let program = Parser::parse(source).unwrap();
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("result").unwrap();
    assert_eq!(symbol.ty, Type::Int);
}

// ========== ARITHMETIC OPERATIONS - FLOAT ==========

#[test]
fn test_float_add_float() {
    let source = "result = 5.0 + 3.0";
    let program = Parser::parse(source).unwrap();
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("result").unwrap();
    assert_eq!(symbol.ty, Type::Float);
}

#[test]
fn test_float_multiply_float() {
    let source = "result = 2.5 * 4.0";
    let program = Parser::parse(source).unwrap();
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("result").unwrap();
    assert_eq!(symbol.ty, Type::Float);
}

// ========== MIXED INT/FLOAT OPERATIONS ==========

#[test]
fn test_int_add_float() {
    let source = "result = 5 + 3.0";
    let program = Parser::parse(source).unwrap();
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("result").unwrap();
    assert_eq!(symbol.ty, Type::Float, "Int + Float should be Float");
}

#[test]
fn test_float_add_int() {
    let source = "result = 3.0 + 5";
    let program = Parser::parse(source).unwrap();
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("result").unwrap();
    assert_eq!(symbol.ty, Type::Float, "Float + Int should be Float");
}

#[test]
fn test_int_multiply_float() {
    let source = "result = 2 * 3.5";
    let program = Parser::parse(source).unwrap();
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("result").unwrap();
    assert_eq!(symbol.ty, Type::Float);
}

#[test]
fn test_float_subtract_int() {
    let source = "result = 10.5 - 3";
    let program = Parser::parse(source).unwrap();
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("result").unwrap();
    assert_eq!(symbol.ty, Type::Float);
}

// ========== STRING OPERATIONS ==========

#[test]
fn test_string_concat() {
    let source = r#"result = "hello" + "world""#;
    let program = Parser::parse(source).unwrap();
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("result").unwrap();
    assert_eq!(symbol.ty, Type::Str);
}

#[test]
fn test_string_concat_with_variables() {
    let source = r#"
first = "hello"
second = "world"
result = first + second
"#;
    let program = Parser::parse(source).unwrap();
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("result").unwrap();
    assert_eq!(symbol.ty, Type::Str);
}

// ========== BITWISE OPERATIONS ==========

#[test]
fn test_int_bitor_int() {
    let source = "result = 5 | 3";
    let program = Parser::parse(source).unwrap();
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("result").unwrap();
    assert_eq!(symbol.ty, Type::Int);
}

#[test]
fn test_int_bitand_int() {
    let source = "result = 5 & 3";
    let program = Parser::parse(source).unwrap();
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("result").unwrap();
    assert_eq!(symbol.ty, Type::Int);
}

#[test]
fn test_int_bitxor_int() {
    let source = "result = 5 ^ 3";
    let program = Parser::parse(source).unwrap();
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("result").unwrap();
    assert_eq!(symbol.ty, Type::Int);
}

#[test]
fn test_int_lshift_int() {
    let source = "result = 5 << 2";
    let program = Parser::parse(source).unwrap();
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("result").unwrap();
    assert_eq!(symbol.ty, Type::Int);
}

#[test]
fn test_int_rshift_int() {
    let source = "result = 20 >> 2";
    let program = Parser::parse(source).unwrap();
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("result").unwrap();
    assert_eq!(symbol.ty, Type::Int);
}

// ========== COMPARISON OPERATIONS ==========

#[test]
fn test_comparison_equal() {
    let source = "result = 5 == 3";
    let program = Parser::parse(source).unwrap();
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("result").unwrap();
    assert_eq!(symbol.ty, Type::Bool);
}

#[test]
fn test_comparison_not_equal() {
    let source = "result = 5 != 3";
    let program = Parser::parse(source).unwrap();
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("result").unwrap();
    assert_eq!(symbol.ty, Type::Bool);
}

#[test]
fn test_comparison_less_than() {
    let source = "result = 5 < 10";
    let program = Parser::parse(source).unwrap();
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("result").unwrap();
    assert_eq!(symbol.ty, Type::Bool);
}

#[test]
fn test_comparison_greater_than() {
    let source = "result = 10 > 5";
    let program = Parser::parse(source).unwrap();
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("result").unwrap();
    assert_eq!(symbol.ty, Type::Bool);
}

#[test]
fn test_comparison_less_equal() {
    let source = "result = 5 <= 10";
    let program = Parser::parse(source).unwrap();
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("result").unwrap();
    assert_eq!(symbol.ty, Type::Bool);
}

#[test]
fn test_comparison_greater_equal() {
    let source = "result = 10 >= 5";
    let program = Parser::parse(source).unwrap();
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("result").unwrap();
    assert_eq!(symbol.ty, Type::Bool);
}

#[test]
fn test_comparison_is() {
    let source = "result = x is None";
    let program = Parser::parse(source).unwrap();
    let mut analyzer = SemanticAnalyzer::new();
    let _ = analyzer.analyze(&program); // Will have error for undefined x, but that's ok

    let symbol = analyzer.symbol_table().resolve_symbol("result").unwrap();
    assert_eq!(symbol.ty, Type::Bool);
}

#[test]
fn test_comparison_in() {
    let source = "result = 5 in [1, 2, 3]";
    let program = Parser::parse(source).unwrap();
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("result").unwrap();
    assert_eq!(symbol.ty, Type::Bool);
}

#[test]
fn test_comparison_chained() {
    let source = "result = 1 < 5 < 10";
    let program = Parser::parse(source).unwrap();
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("result").unwrap();
    assert_eq!(symbol.ty, Type::Bool);
}

// ========== UNARY OPERATIONS ==========

#[test]
fn test_unary_not() {
    let source = "result = not True";
    let program = Parser::parse(source).unwrap();
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("result").unwrap();
    assert_eq!(symbol.ty, Type::Bool);
}

#[test]
fn test_unary_minus_int() {
    let source = "result = -42";
    let program = Parser::parse(source).unwrap();
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("result").unwrap();
    assert_eq!(symbol.ty, Type::Int);
}

#[test]
fn test_unary_minus_float() {
    let source = "result = -3.14";
    let program = Parser::parse(source).unwrap();
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("result").unwrap();
    assert_eq!(symbol.ty, Type::Float);
}

#[test]
fn test_unary_plus_int() {
    let source = "result = +42";
    let program = Parser::parse(source).unwrap();
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("result").unwrap();
    assert_eq!(symbol.ty, Type::Int);
}

#[test]
fn test_unary_invert() {
    let source = "result = ~5";
    let program = Parser::parse(source).unwrap();
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("result").unwrap();
    assert_eq!(symbol.ty, Type::Int);
}

// ========== COMPLEX EXPRESSIONS ==========

#[test]
fn test_nested_arithmetic() {
    let source = "result = (5 + 3) * 2";
    let program = Parser::parse(source).unwrap();
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("result").unwrap();
    assert_eq!(symbol.ty, Type::Int);
}

#[test]
fn test_mixed_float_propagation() {
    let source = "result = 1 + 2 + 3.0";
    let program = Parser::parse(source).unwrap();
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("result").unwrap();
    assert_eq!(
        symbol.ty,
        Type::Float,
        "Adding float to int chain should result in Float"
    );
}

#[test]
fn test_variable_operations() {
    let source = r#"
x = 10
y = 5
result = x + y
"#;
    let program = Parser::parse(source).unwrap();
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("result").unwrap();
    assert_eq!(symbol.ty, Type::Int);
}

// ========== EDGE CASES AND UNSUPPORTED OPERATIONS ==========

#[test]
fn test_string_multiply_unsupported() {
    // String * Int is not supported and should produce a validation error
    let source = r#"result = "hello" * 3"#;
    let program = Parser::parse(source).unwrap();
    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&program);

    // Should have a binary operation error
    assert!(result.is_err(), "Expected error for str * int");
    let errors = result.err().unwrap();
    assert_eq!(errors.len(), 1);
    assert!(
        matches!(&errors[0], SemanticError::InvalidBinaryOperation { operator, .. } if operator == "*"),
        "Expected InvalidBinaryOperation error"
    );
}

#[test]
fn test_bitwise_on_float_unsupported() {
    // Bitwise operations on floats should be rejected
    let source = "result = 5.0 | 3.0";
    let program = Parser::parse(source).unwrap();
    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&program);

    // Should produce InvalidBinaryOperation error
    assert!(result.is_err());
    let errors = result.err().unwrap();
    assert!(errors.iter().any(|e| matches!(
        e,
        SemanticError::InvalidBinaryOperation { operator, .. } if operator == "|"
    )));
}

#[test]
fn test_logical_and_returns_unknown() {
    // In Python, 'and' returns one of the operands, not Bool
    // We simplify to Unknown for now
    let source = "result = True and False";
    let program = Parser::parse(source).unwrap();
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("result").unwrap();
    assert_eq!(
        symbol.ty,
        Type::Unknown,
        "Logical 'and' simplified to Unknown"
    );
}

#[test]
fn test_logical_or_returns_unknown() {
    let source = "result = True or False";
    let program = Parser::parse(source).unwrap();
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("result").unwrap();
    assert_eq!(
        symbol.ty,
        Type::Unknown,
        "Logical 'or' simplified to Unknown"
    );
}
