/// Comprehensive tests for the Silk parser

use silk_parser::{Parser, ParseError};
use silk_ast::{Expression, ExpressionKind, Statement, StatementKind, BinaryOperator, UnaryOperator, CompareOperator, LogicalOperator, AugAssignOperator};
use pretty_assertions::assert_eq;

// ============================================================================
// Helper Functions
// ============================================================================

fn parse_expr(source: &str) -> Result<Expression, ParseError> {
    let program = Parser::parse(source)?;
    assert_eq!(program.statements.len(), 1, "Expected exactly one statement");
    
    match &program.statements[0].kind {
        StatementKind::Expr(expr) => Ok(expr.clone()),
        _ => panic!("Expected expression statement, got {:?}", program.statements[0].kind),
    }
}

fn parse_stmt(source: &str) -> Result<Statement, ParseError> {
    let program = Parser::parse(source)?;
    assert_eq!(program.statements.len(), 1, "Expected exactly one statement");
    Ok(program.statements[0].clone())
}

fn parse_program(source: &str) -> Result<Vec<Statement>, ParseError> {
    let program = Parser::parse(source)?;
    Ok(program.statements)
}

// ============================================================================
// Literal Tests
// ============================================================================

#[test]
fn test_integer_literal() {
    let expr = parse_expr("42").unwrap();
    match expr.kind {
        ExpressionKind::Integer(value) => assert_eq!(value, 42),
        _ => panic!("Expected integer literal, got {:?}", expr.kind),
    }
}

#[test]
fn test_negative_integer() {
    let expr = parse_expr("-42").unwrap();
    match expr.kind {
        ExpressionKind::UnaryOp { op, operand } => {
            assert_eq!(op, UnaryOperator::USub);
            match operand.kind {
                ExpressionKind::Integer(value) => assert_eq!(value, 42),
                _ => panic!("Expected integer in unary op"),
            }
        }
        _ => panic!("Expected unary op, got {:?}", expr.kind),
    }
}

#[test]
fn test_float_literal() {
    let expr = parse_expr("3.14").unwrap();
    match expr.kind {
        ExpressionKind::Float(value) => assert!((value - 3.14).abs() < 0.0001),
        _ => panic!("Expected float literal, got {:?}", expr.kind),
    }
}

#[test]
fn test_string_literal() {
    let expr = parse_expr("\"hello\"").unwrap();
    match expr.kind {
        ExpressionKind::String(ref value) => assert_eq!(value, "hello"),
        _ => panic!("Expected string literal, got {:?}", expr.kind),
    }
}

#[test]
fn test_boolean_true() {
    let expr = parse_expr("True").unwrap();
    match expr.kind {
        ExpressionKind::Boolean(value) => assert!(value),
        _ => panic!("Expected boolean literal, got {:?}", expr.kind),
    }
}

#[test]
fn test_boolean_false() {
    let expr = parse_expr("False").unwrap();
    match expr.kind {
        ExpressionKind::Boolean(value) => assert!(!value),
        _ => panic!("Expected boolean literal, got {:?}", expr.kind),
    }
}

#[test]
fn test_none_literal() {
    let expr = parse_expr("None").unwrap();
    match expr.kind {
        ExpressionKind::None => (),
        _ => panic!("Expected None literal, got {:?}", expr.kind),
    }
}

// ============================================================================
// Identifier Tests
// ============================================================================

#[test]
fn test_identifier() {
    let expr = parse_expr("variable").unwrap();
    match expr.kind {
        ExpressionKind::Identifier(ref name) => assert_eq!(name, "variable"),
        _ => panic!("Expected identifier, got {:?}", expr.kind),
    }
}

#[test]
fn test_identifier_with_underscore() {
    let expr = parse_expr("_private_var").unwrap();
    match expr.kind {
        ExpressionKind::Identifier(ref name) => assert_eq!(name, "_private_var"),
        _ => panic!("Expected identifier, got {:?}", expr.kind),
    }
}

// ============================================================================
// Binary Operator Tests
// ============================================================================

#[test]
fn test_addition() {
    let expr = parse_expr("1 + 2").unwrap();
    match expr.kind {
        ExpressionKind::BinaryOp { left, op, right } => {
            assert_eq!(op, BinaryOperator::Add);
            assert!(matches!(left.kind, ExpressionKind::Integer(1)));
            assert!(matches!(right.kind, ExpressionKind::Integer(2)));
        }
        _ => panic!("Expected binary operation, got {:?}", expr.kind),
    }
}

#[test]
fn test_subtraction() {
    let expr = parse_expr("10 - 5").unwrap();
    match expr.kind {
        ExpressionKind::BinaryOp { left, op, right } => {
            assert_eq!(op, BinaryOperator::Sub);
            assert!(matches!(left.kind, ExpressionKind::Integer(10)));
            assert!(matches!(right.kind, ExpressionKind::Integer(5)));
        }
        _ => panic!("Expected binary operation, got {:?}", expr.kind),
    }
}

#[test]
fn test_multiplication() {
    let expr = parse_expr("3 * 4").unwrap();
    match expr.kind {
        ExpressionKind::BinaryOp { left, op, right } => {
            assert_eq!(op, BinaryOperator::Mult);
            assert!(matches!(left.kind, ExpressionKind::Integer(3)));
            assert!(matches!(right.kind, ExpressionKind::Integer(4)));
        }
        _ => panic!("Expected binary operation, got {:?}", expr.kind),
    }
}

#[test]
fn test_division() {
    let expr = parse_expr("8 / 2").unwrap();
    match expr.kind {
        ExpressionKind::BinaryOp { left, op, right } => {
            assert_eq!(op, BinaryOperator::Div);
            assert!(matches!(left.kind, ExpressionKind::Integer(8)));
            assert!(matches!(right.kind, ExpressionKind::Integer(2)));
        }
        _ => panic!("Expected binary operation, got {:?}", expr.kind),
    }
}

#[test]
fn test_power() {
    let expr = parse_expr("2 ** 3").unwrap();
    match expr.kind {
        ExpressionKind::BinaryOp { left, op, right } => {
            assert_eq!(op, BinaryOperator::Pow);
            assert!(matches!(left.kind, ExpressionKind::Integer(2)));
            assert!(matches!(right.kind, ExpressionKind::Integer(3)));
        }
        _ => panic!("Expected binary operation, got {:?}", expr.kind),
    }
}

// ============================================================================
// Operator Precedence Tests
// ============================================================================

#[test]
fn test_precedence_addition_multiplication() {
    // 1 + 2 * 3 should be parsed as 1 + (2 * 3)
    let expr = parse_expr("1 + 2 * 3").unwrap();
    match expr.kind {
        ExpressionKind::BinaryOp { left, op, right } => {
            assert_eq!(op, BinaryOperator::Add);
            assert!(matches!(left.kind, ExpressionKind::Integer(1)));
            
            match right.kind {
                ExpressionKind::BinaryOp { left: mult_left, op: mult_op, right: mult_right } => {
                    assert_eq!(mult_op, BinaryOperator::Mult);
                    assert!(matches!(mult_left.kind, ExpressionKind::Integer(2)));
                    assert!(matches!(mult_right.kind, ExpressionKind::Integer(3)));
                }
                _ => panic!("Expected multiplication on right side"),
            }
        }
        _ => panic!("Expected binary operation, got {:?}", expr.kind),
    }
}

#[test]
fn test_precedence_parentheses() {
    // (1 + 2) * 3 should be parsed as (1 + 2) * 3
    let expr = parse_expr("(1 + 2) * 3").unwrap();
    match expr.kind {
        ExpressionKind::BinaryOp { left, op, right } => {
            assert_eq!(op, BinaryOperator::Mult);
            assert!(matches!(right.kind, ExpressionKind::Integer(3)));
            
            match left.kind {
                ExpressionKind::BinaryOp { left: add_left, op: add_op, right: add_right } => {
                    assert_eq!(add_op, BinaryOperator::Add);
                    assert!(matches!(add_left.kind, ExpressionKind::Integer(1)));
                    assert!(matches!(add_right.kind, ExpressionKind::Integer(2)));
                }
                _ => panic!("Expected addition on left side"),
            }
        }
        _ => panic!("Expected binary operation, got {:?}", expr.kind),
    }
}

#[test]
fn test_power_right_associative() {
    // 2 ** 3 ** 2 should be parsed as 2 ** (3 ** 2) = 2 ** 9 = 512
    let expr = parse_expr("2 ** 3 ** 2").unwrap();
    match expr.kind {
        ExpressionKind::BinaryOp { left, op, right } => {
            assert_eq!(op, BinaryOperator::Pow);
            assert!(matches!(left.kind, ExpressionKind::Integer(2)));
            
            match right.kind {
                ExpressionKind::BinaryOp { left: pow_left, op: pow_op, right: pow_right } => {
                    assert_eq!(pow_op, BinaryOperator::Pow);
                    assert!(matches!(pow_left.kind, ExpressionKind::Integer(3)));
                    assert!(matches!(pow_right.kind, ExpressionKind::Integer(2)));
                }
                _ => panic!("Expected power operation on right side"),
            }
        }
        _ => panic!("Expected binary operation, got {:?}", expr.kind),
    }
}

// ============================================================================
// Unary Operator Tests
// ============================================================================

#[test]
fn test_unary_plus() {
    let expr = parse_expr("+5").unwrap();
    match expr.kind {
        ExpressionKind::UnaryOp { op, operand } => {
            assert_eq!(op, UnaryOperator::UAdd);
            assert!(matches!(operand.kind, ExpressionKind::Integer(5)));
        }
        _ => panic!("Expected unary operation, got {:?}", expr.kind),
    }
}

#[test]
fn test_unary_minus() {
    let expr = parse_expr("-5").unwrap();
    match expr.kind {
        ExpressionKind::UnaryOp { op, operand } => {
            assert_eq!(op, UnaryOperator::USub);
            assert!(matches!(operand.kind, ExpressionKind::Integer(5)));
        }
        _ => panic!("Expected unary operation, got {:?}", expr.kind),
    }
}

#[test]
fn test_unary_not() {
    let expr = parse_expr("not True").unwrap();
    match expr.kind {
        ExpressionKind::UnaryOp { op, operand } => {
            assert_eq!(op, UnaryOperator::Not);
            assert!(matches!(operand.kind, ExpressionKind::Boolean(true)));
        }
        _ => panic!("Expected unary operation, got {:?}", expr.kind),
    }
}

#[test]
fn test_unary_invert() {
    let expr = parse_expr("~42").unwrap();
    match expr.kind {
        ExpressionKind::UnaryOp { op, operand } => {
            assert_eq!(op, UnaryOperator::Invert);
            assert!(matches!(operand.kind, ExpressionKind::Integer(42)));
        }
        _ => panic!("Expected unary operation, got {:?}", expr.kind),
    }
}

// ============================================================================
// Comparison Operator Tests
// ============================================================================

#[test]
fn test_equal_comparison() {
    let expr = parse_expr("5 == 5").unwrap();
    match expr.kind {
        ExpressionKind::Compare { left, ops, comparators } => {
            assert!(matches!(left.kind, ExpressionKind::Integer(5)));
            assert_eq!(ops.len(), 1);
            assert_eq!(ops[0], CompareOperator::Eq);
            assert_eq!(comparators.len(), 1);
            assert!(matches!(comparators[0].kind, ExpressionKind::Integer(5)));
        }
        _ => panic!("Expected comparison, got {:?}", expr.kind),
    }
}

#[test]
fn test_not_equal_comparison() {
    let expr = parse_expr("5 != 3").unwrap();
    match expr.kind {
        ExpressionKind::Compare { left, ops, comparators } => {
            assert!(matches!(left.kind, ExpressionKind::Integer(5)));
            assert_eq!(ops[0], CompareOperator::NotEq);
            assert!(matches!(comparators[0].kind, ExpressionKind::Integer(3)));
        }
        _ => panic!("Expected comparison, got {:?}", expr.kind),
    }
}

#[test]
fn test_less_than() {
    let expr = parse_expr("3 < 5").unwrap();
    match expr.kind {
        ExpressionKind::Compare { left, ops, comparators } => {
            assert!(matches!(left.kind, ExpressionKind::Integer(3)));
            assert_eq!(ops[0], CompareOperator::Lt);
            assert!(matches!(comparators[0].kind, ExpressionKind::Integer(5)));
        }
        _ => panic!("Expected comparison, got {:?}", expr.kind),
    }
}

#[test]
fn test_greater_than() {
    let expr = parse_expr("10 > 2").unwrap();
    match expr.kind {
        ExpressionKind::Compare { left, ops, comparators } => {
            assert!(matches!(left.kind, ExpressionKind::Integer(10)));
            assert_eq!(ops[0], CompareOperator::Gt);
            assert!(matches!(comparators[0].kind, ExpressionKind::Integer(2)));
        }
        _ => panic!("Expected comparison, got {:?}", expr.kind),
    }
}

#[test]
fn test_less_equal() {
    let expr = parse_expr("5 <= 5").unwrap();
    match expr.kind {
        ExpressionKind::Compare { left, ops, comparators } => {
            assert!(matches!(left.kind, ExpressionKind::Integer(5)));
            assert_eq!(ops[0], CompareOperator::LtE);
            assert!(matches!(comparators[0].kind, ExpressionKind::Integer(5)));
        }
        _ => panic!("Expected comparison, got {:?}", expr.kind),
    }
}

#[test]
fn test_greater_equal() {
    let expr = parse_expr("10 >= 5").unwrap();
    match expr.kind {
        ExpressionKind::Compare { left, ops, comparators } => {
            assert!(matches!(left.kind, ExpressionKind::Integer(10)));
            assert_eq!(ops[0], CompareOperator::GtE);
            assert!(matches!(comparators[0].kind, ExpressionKind::Integer(5)));
        }
        _ => panic!("Expected comparison, got {:?}", expr.kind),
    }
}

// ============================================================================
// Logical Operator Tests
// ============================================================================

#[test]
fn test_logical_and() {
    let expr = parse_expr("True and False").unwrap();
    match expr.kind {
        ExpressionKind::LogicalOp { left, op, right } => {
            assert_eq!(op, LogicalOperator::And);
            assert!(matches!(left.kind, ExpressionKind::Boolean(true)));
            assert!(matches!(right.kind, ExpressionKind::Boolean(false)));
        }
        _ => panic!("Expected logical operation, got {:?}", expr.kind),
    }
}

#[test]
fn test_logical_or() {
    let expr = parse_expr("True or False").unwrap();
    match expr.kind {
        ExpressionKind::LogicalOp { left, op, right } => {
            assert_eq!(op, LogicalOperator::Or);
            assert!(matches!(left.kind, ExpressionKind::Boolean(true)));
            assert!(matches!(right.kind, ExpressionKind::Boolean(false)));
        }
        _ => panic!("Expected logical operation, got {:?}", expr.kind),
    }
}

#[test]
fn test_logical_precedence() {
    // True or False and False should be parsed as True or (False and False)
    let expr = parse_expr("True or False and False").unwrap();
    match expr.kind {
        ExpressionKind::LogicalOp { left, op, right } => {
            assert_eq!(op, LogicalOperator::Or);
            assert!(matches!(left.kind, ExpressionKind::Boolean(true)));
            
            match right.kind {
                ExpressionKind::LogicalOp { left: and_left, op: and_op, right: and_right } => {
                    assert_eq!(and_op, LogicalOperator::And);
                    assert!(matches!(and_left.kind, ExpressionKind::Boolean(false)));
                    assert!(matches!(and_right.kind, ExpressionKind::Boolean(false)));
                }
                _ => panic!("Expected logical and on right side"),
            }
        }
        _ => panic!("Expected logical operation, got {:?}", expr.kind),
    }
}

// ============================================================================
// Function Call Tests
// ============================================================================

#[test]
fn test_function_call_no_args() {
    let expr = parse_expr("func()").unwrap();
    match expr.kind {
        ExpressionKind::Call { func, args, keywords } => {
            assert!(matches!(func.kind, ExpressionKind::Identifier(_)));
            assert_eq!(args.len(), 0);
            assert_eq!(keywords.len(), 0);
        }
        _ => panic!("Expected function call, got {:?}", expr.kind),
    }
}

#[test]
fn test_function_call_single_arg() {
    let expr = parse_expr("func(42)").unwrap();
    match expr.kind {
        ExpressionKind::Call { func, args, keywords } => {
            assert!(matches!(func.kind, ExpressionKind::Identifier(_)));
            assert_eq!(args.len(), 1);
            assert!(matches!(args[0].kind, ExpressionKind::Integer(42)));
            assert_eq!(keywords.len(), 0);
        }
        _ => panic!("Expected function call, got {:?}", expr.kind),
    }
}

#[test]
fn test_function_call_multiple_args() {
    let expr = parse_expr("func(1, 2, 3)").unwrap();
    match expr.kind {
        ExpressionKind::Call { func, args, keywords: _ } => {
            assert!(matches!(func.kind, ExpressionKind::Identifier(_)));
            assert_eq!(args.len(), 3);
            assert!(matches!(args[0].kind, ExpressionKind::Integer(1)));
            assert!(matches!(args[1].kind, ExpressionKind::Integer(2)));
            assert!(matches!(args[2].kind, ExpressionKind::Integer(3)));
        }
        _ => panic!("Expected function call, got {:?}", expr.kind),
    }
}

#[test]
fn test_nested_function_calls() {
    let expr = parse_expr("outer(inner(5))").unwrap();
    match expr.kind {
        ExpressionKind::Call { func, args, .. } => {
            assert!(matches!(func.kind, ExpressionKind::Identifier(_)));
            assert_eq!(args.len(), 1);
            
            match &args[0].kind {
                ExpressionKind::Call { args: inner_args, .. } => {
                    assert_eq!(inner_args.len(), 1);
                    assert!(matches!(inner_args[0].kind, ExpressionKind::Integer(5)));
                }
                _ => panic!("Expected nested call"),
            }
        }
        _ => panic!("Expected function call, got {:?}", expr.kind),
    }
}

// ============================================================================
// Subscript Tests
// ============================================================================

#[test]
fn test_subscript_integer() {
    let expr = parse_expr("arr[0]").unwrap();
    match expr.kind {
        ExpressionKind::Subscript { value, index } => {
            assert!(matches!(value.kind, ExpressionKind::Identifier(_)));
            assert!(matches!(index.kind, ExpressionKind::Integer(0)));
        }
        _ => panic!("Expected subscript, got {:?}", expr.kind),
    }
}

#[test]
fn test_subscript_expression() {
    let expr = parse_expr("arr[i + 1]").unwrap();
    match expr.kind {
        ExpressionKind::Subscript { value, index } => {
            assert!(matches!(value.kind, ExpressionKind::Identifier(_)));
            assert!(matches!(index.kind, ExpressionKind::BinaryOp { .. }));
        }
        _ => panic!("Expected subscript, got {:?}", expr.kind),
    }
}

#[test]
fn test_chained_subscript() {
    let expr = parse_expr("matrix[0][1]").unwrap();
    match expr.kind {
        ExpressionKind::Subscript { value, index } => {
            assert!(matches!(index.kind, ExpressionKind::Integer(1)));
            
            match value.kind {
                ExpressionKind::Subscript { value: inner_value, index: inner_index } => {
                    assert!(matches!(inner_value.kind, ExpressionKind::Identifier(_)));
                    assert!(matches!(inner_index.kind, ExpressionKind::Integer(0)));
                }
                _ => panic!("Expected nested subscript"),
            }
        }
        _ => panic!("Expected subscript, got {:?}", expr.kind),
    }
}

// ============================================================================
// Attribute Access Tests
// ============================================================================

#[test]
fn test_attribute_access() {
    let expr = parse_expr("obj.attr").unwrap();
    match expr.kind {
        ExpressionKind::Attribute { value, attr } => {
            assert!(matches!(value.kind, ExpressionKind::Identifier(_)));
            assert_eq!(attr, "attr");
        }
        _ => panic!("Expected attribute access, got {:?}", expr.kind),
    }
}

#[test]
fn test_chained_attribute_access() {
    let expr = parse_expr("obj.attr1.attr2").unwrap();
    match expr.kind {
        ExpressionKind::Attribute { value, attr } => {
            assert_eq!(attr, "attr2");
            
            match value.kind {
                ExpressionKind::Attribute { value: inner_value, attr: inner_attr } => {
                    assert!(matches!(inner_value.kind, ExpressionKind::Identifier(_)));
                    assert_eq!(inner_attr, "attr1");
                }
                _ => panic!("Expected nested attribute access"),
            }
        }
        _ => panic!("Expected attribute access, got {:?}", expr.kind),
    }
}

#[test]
fn test_attribute_method_call() {
    let expr = parse_expr("obj.method()").unwrap();
    match expr.kind {
        ExpressionKind::Call { func, .. } => {
            match func.kind {
                ExpressionKind::Attribute { value, attr } => {
                    assert!(matches!(value.kind, ExpressionKind::Identifier(_)));
                    assert_eq!(attr, "method");
                }
                _ => panic!("Expected attribute in function call"),
            }
        }
        _ => panic!("Expected function call, got {:?}", expr.kind),
    }
}

// ============================================================================
// List Literal Tests
// ============================================================================

#[test]
fn test_empty_list() {
    let expr = parse_expr("[]").unwrap();
    match expr.kind {
        ExpressionKind::List { elements } => {
            assert_eq!(elements.len(), 0);
        }
        _ => panic!("Expected list, got {:?}", expr.kind),
    }
}

#[test]
fn test_list_with_elements() {
    let expr = parse_expr("[1, 2, 3]").unwrap();
    match expr.kind {
        ExpressionKind::List { elements } => {
            assert_eq!(elements.len(), 3);
            assert!(matches!(elements[0].kind, ExpressionKind::Integer(1)));
            assert!(matches!(elements[1].kind, ExpressionKind::Integer(2)));
            assert!(matches!(elements[2].kind, ExpressionKind::Integer(3)));
        }
        _ => panic!("Expected list, got {:?}", expr.kind),
    }
}

#[test]
fn test_list_with_expressions() {
    let expr = parse_expr("[1 + 2, 3 * 4]").unwrap();
    match expr.kind {
        ExpressionKind::List { elements } => {
            assert_eq!(elements.len(), 2);
            assert!(matches!(elements[0].kind, ExpressionKind::BinaryOp { .. }));
            assert!(matches!(elements[1].kind, ExpressionKind::BinaryOp { .. }));
        }
        _ => panic!("Expected list, got {:?}", expr.kind),
    }
}

#[test]
fn test_nested_list() {
    let expr = parse_expr("[[1, 2], [3, 4]]").unwrap();
    match expr.kind {
        ExpressionKind::List { elements } => {
            assert_eq!(elements.len(), 2);
            assert!(matches!(elements[0].kind, ExpressionKind::List { .. }));
            assert!(matches!(elements[1].kind, ExpressionKind::List { .. }));
        }
        _ => panic!("Expected list, got {:?}", expr.kind),
    }
}

// ============================================================================
// Statement Tests
// ============================================================================

#[test]
fn test_expression_statement() {
    let stmt = parse_stmt("42").unwrap();
    match stmt.kind {
        StatementKind::Expr(expr) => {
            assert!(matches!(expr.kind, ExpressionKind::Integer(42)));
        }
        _ => panic!("Expected expression statement, got {:?}", stmt.kind),
    }
}

#[test]
fn test_simple_assignment() {
    let stmt = parse_stmt("x = 5").unwrap();
    match stmt.kind {
        StatementKind::Assign { targets, value, type_annotation } => {
            assert_eq!(targets.len(), 1);
            assert!(matches!(targets[0].kind, ExpressionKind::Identifier(_)));
            assert!(matches!(value.kind, ExpressionKind::Integer(5)));
            assert!(type_annotation.is_none());
        }
        _ => panic!("Expected assignment, got {:?}", stmt.kind),
    }
}

#[test]
fn test_assignment_with_expression() {
    let stmt = parse_stmt("result = 1 + 2").unwrap();
    match stmt.kind {
        StatementKind::Assign { targets, value, .. } => {
            assert_eq!(targets.len(), 1);
            assert!(matches!(value.kind, ExpressionKind::BinaryOp { .. }));
        }
        _ => panic!("Expected assignment, got {:?}", stmt.kind),
    }
}

#[test]
fn test_augmented_assignment_add() {
    let stmt = parse_stmt("x += 5").unwrap();
    match stmt.kind {
        StatementKind::AugAssign { target, op, value } => {
            assert!(matches!(target.kind, ExpressionKind::Identifier(_)));
            assert_eq!(op, AugAssignOperator::Add);
            assert!(matches!(value.kind, ExpressionKind::Integer(5)));
        }
        _ => panic!("Expected augmented assignment, got {:?}", stmt.kind),
    }
}

#[test]
fn test_augmented_assignment_sub() {
    let stmt = parse_stmt("x -= 3").unwrap();
    match stmt.kind {
        StatementKind::AugAssign { op, .. } => {
            assert_eq!(op, AugAssignOperator::Sub);
        }
        _ => panic!("Expected augmented assignment, got {:?}", stmt.kind),
    }
}

#[test]
fn test_augmented_assignment_mult() {
    let stmt = parse_stmt("x *= 2").unwrap();
    match stmt.kind {
        StatementKind::AugAssign { op, .. } => {
            assert_eq!(op, AugAssignOperator::Mult);
        }
        _ => panic!("Expected augmented assignment, got {:?}", stmt.kind),
    }
}

#[test]
fn test_return_statement_with_value() {
    let stmt = parse_stmt("return 42").unwrap();
    match stmt.kind {
        StatementKind::Return { value } => {
            assert!(value.is_some());
            let expr = value.unwrap();
            assert!(matches!(expr.kind, ExpressionKind::Integer(42)));
        }
        _ => panic!("Expected return statement, got {:?}", stmt.kind),
    }
}

#[test]
fn test_return_statement_without_value() {
    let stmt = parse_stmt("return").unwrap();
    match stmt.kind {
        StatementKind::Return { value } => {
            assert!(value.is_none());
        }
        _ => panic!("Expected return statement, got {:?}", stmt.kind),
    }
}

#[test]
fn test_pass_statement() {
    let stmt = parse_stmt("pass").unwrap();
    match stmt.kind {
        StatementKind::Pass => (),
        _ => panic!("Expected pass statement, got {:?}", stmt.kind),
    }
}

#[test]
fn test_break_statement() {
    let stmt = parse_stmt("break").unwrap();
    match stmt.kind {
        StatementKind::Break => (),
        _ => panic!("Expected break statement, got {:?}", stmt.kind),
    }
}

#[test]
fn test_continue_statement() {
    let stmt = parse_stmt("continue").unwrap();
    match stmt.kind {
        StatementKind::Continue => (),
        _ => panic!("Expected continue statement, got {:?}", stmt.kind),
    }
}

// ============================================================================
// Multiple Statement Tests
// ============================================================================

#[test]
fn test_multiple_statements() {
    let stmts = parse_program("x = 5\ny = 10\nz = x + y").unwrap();
    assert_eq!(stmts.len(), 3);
    assert!(matches!(stmts[0].kind, StatementKind::Assign { .. }));
    assert!(matches!(stmts[1].kind, StatementKind::Assign { .. }));
    assert!(matches!(stmts[2].kind, StatementKind::Assign { .. }));
}

#[test]
fn test_statements_with_blank_lines() {
    let stmts = parse_program("x = 5\n\ny = 10\n\n\nz = 15").unwrap();
    assert_eq!(stmts.len(), 3);
}

// ============================================================================
// Error Tests
// ============================================================================

#[test]
fn test_error_unexpected_token() {
    let result = parse_expr("1 +");
    assert!(result.is_err());
}

#[test]
fn test_error_missing_closing_paren() {
    let result = parse_expr("(1 + 2");
    assert!(result.is_err());
}

#[test]
fn test_error_missing_closing_bracket() {
    let result = parse_expr("[1, 2, 3");
    assert!(result.is_err());
}

#[test]
fn test_error_invalid_syntax() {
    let result = parse_expr("@");
    assert!(result.is_err());
}

// ============================================================================
// Edge Case Tests
// ============================================================================

#[test]
fn test_complex_expression() {
    // Test: (a + b) * c - d / e ** 2
    let expr = parse_expr("(a + b) * c - d / e ** 2").unwrap();
    assert!(matches!(expr.kind, ExpressionKind::BinaryOp { .. }));
}

#[test]
fn test_deeply_nested_parentheses() {
    let expr = parse_expr("((((1))))").unwrap();
    match expr.kind {
        ExpressionKind::Integer(value) => assert_eq!(value, 1),
        _ => panic!("Expected integer after unwrapping parentheses"),
    }
}

#[test]
fn test_whitespace_handling() {
    let expr1 = parse_expr("1+2").unwrap();
    let expr2 = parse_expr("1 + 2").unwrap();
    let expr3 = parse_expr("1  +  2").unwrap();
    
    // All should parse the same
    assert!(matches!(expr1.kind, ExpressionKind::BinaryOp { .. }));
    assert!(matches!(expr2.kind, ExpressionKind::BinaryOp { .. }));
    assert!(matches!(expr3.kind, ExpressionKind::BinaryOp { .. }));
}

#[test]
fn test_trailing_comma_in_list() {
    let expr = parse_expr("[1, 2, 3,]").unwrap();
    match expr.kind {
        ExpressionKind::List { elements } => {
            assert_eq!(elements.len(), 3);
        }
        _ => panic!("Expected list, got {:?}", expr.kind),
    }
}

#[test]
fn test_empty_program() {
    let stmts = parse_program("").unwrap();
    assert_eq!(stmts.len(), 0);
}

#[test]
fn test_only_newlines() {
    let stmts = parse_program("\n\n\n").unwrap();
    assert_eq!(stmts.len(), 0);
}

// ============================================================================
// Dict Literal Tests
// ============================================================================

#[test]
fn test_empty_dict() {
    let expr = parse_expr("{}").unwrap();
    match expr.kind {
        ExpressionKind::Dict { keys, values } => {
            assert_eq!(keys.len(), 0);
            assert_eq!(values.len(), 0);
        }
        _ => panic!("Expected dict literal, got {:?}", expr.kind),
    }
}

#[test]
fn test_dict_with_one_pair() {
    let expr = parse_expr(r#"{"key": "value"}"#).unwrap();
    match expr.kind {
        ExpressionKind::Dict { keys, values } => {
            assert_eq!(keys.len(), 1);
            assert_eq!(values.len(), 1);
            
            match &keys[0].kind {
                ExpressionKind::String(s) => assert_eq!(s, "key"),
                _ => panic!("Expected string key"),
            }
            
            match &values[0].kind {
                ExpressionKind::String(s) => assert_eq!(s, "value"),
                _ => panic!("Expected string value"),
            }
        }
        _ => panic!("Expected dict literal, got {:?}", expr.kind),
    }
}

#[test]
fn test_dict_with_multiple_pairs() {
    let expr = parse_expr(r#"{"a": 1, "b": 2, "c": 3}"#).unwrap();
    match expr.kind {
        ExpressionKind::Dict { keys, values } => {
            assert_eq!(keys.len(), 3);
            assert_eq!(values.len(), 3);
            
            match &keys[0].kind {
                ExpressionKind::String(s) => assert_eq!(s, "a"),
                _ => panic!("Expected string key"),
            }
            match &values[0].kind {
                ExpressionKind::Integer(n) => assert_eq!(*n, 1),
                _ => panic!("Expected integer value"),
            }
        }
        _ => panic!("Expected dict literal, got {:?}", expr.kind),
    }
}

#[test]
fn test_dict_with_trailing_comma() {
    let expr = parse_expr(r#"{"x": 10, "y": 20,}"#).unwrap();
    match expr.kind {
        ExpressionKind::Dict { keys, values } => {
            assert_eq!(keys.len(), 2);
            assert_eq!(values.len(), 2);
        }
        _ => panic!("Expected dict literal, got {:?}", expr.kind),
    }
}

#[test]
fn test_dict_with_expression_keys() {
    let expr = parse_expr("{1 + 1: 2, 3: 4}").unwrap();
    match expr.kind {
        ExpressionKind::Dict { keys, values } => {
            assert_eq!(keys.len(), 2);
            assert_eq!(values.len(), 2);
            
            // First key is 1 + 1 (a binary op)
            match &keys[0].kind {
                ExpressionKind::BinaryOp { .. } => {},
                _ => panic!("Expected binary op as key"),
            }
        }
        _ => panic!("Expected dict literal, got {:?}", expr.kind),
    }
}

#[test]
fn test_nested_dict() {
    let expr = parse_expr(r#"{"outer": {"inner": 42}}"#).unwrap();
    match expr.kind {
        ExpressionKind::Dict { keys, values } => {
            assert_eq!(keys.len(), 1);
            assert_eq!(values.len(), 1);
            
            // Value should be another dict
            match &values[0].kind {
                ExpressionKind::Dict { keys: inner_keys, values: inner_values } => {
                    assert_eq!(inner_keys.len(), 1);
                    assert_eq!(inner_values.len(), 1);
                }
                _ => panic!("Expected nested dict"),
            }
        }
        _ => panic!("Expected dict literal, got {:?}", expr.kind),
    }
}

// ============================================================================
// Set Literal Tests
// ============================================================================

#[test]
fn test_set_with_one_element() {
    let expr = parse_expr("{1}").unwrap();
    match expr.kind {
        ExpressionKind::Set { elements } => {
            assert_eq!(elements.len(), 1);
            match &elements[0].kind {
                ExpressionKind::Integer(n) => assert_eq!(*n, 1),
                _ => panic!("Expected integer element"),
            }
        }
        _ => panic!("Expected set literal, got {:?}", expr.kind),
    }
}

#[test]
fn test_set_with_multiple_elements() {
    let expr = parse_expr("{1, 2, 3, 4, 5}").unwrap();
    match expr.kind {
        ExpressionKind::Set { elements } => {
            assert_eq!(elements.len(), 5);
        }
        _ => panic!("Expected set literal, got {:?}", expr.kind),
    }
}

#[test]
fn test_set_with_trailing_comma() {
    let expr = parse_expr("{1, 2, 3,}").unwrap();
    match expr.kind {
        ExpressionKind::Set { elements } => {
            assert_eq!(elements.len(), 3);
        }
        _ => panic!("Expected set literal, got {:?}", expr.kind),
    }
}

#[test]
fn test_set_with_strings() {
    let expr = parse_expr(r#"{"apple", "banana", "cherry"}"#).unwrap();
    match expr.kind {
        ExpressionKind::Set { elements } => {
            assert_eq!(elements.len(), 3);
            match &elements[0].kind {
                ExpressionKind::String(s) => assert_eq!(s, "apple"),
                _ => panic!("Expected string element"),
            }
        }
        _ => panic!("Expected set literal, got {:?}", expr.kind),
    }
}

#[test]
fn test_set_with_expressions() {
    let expr = parse_expr("{1 + 1, 2 * 3, 10 - 5}").unwrap();
    match expr.kind {
        ExpressionKind::Set { elements } => {
            assert_eq!(elements.len(), 3);
            // All elements should be binary operations
            for elem in &elements {
                match &elem.kind {
                    ExpressionKind::BinaryOp { .. } => {},
                    _ => panic!("Expected binary op in set"),
                }
            }
        }
        _ => panic!("Expected set literal, got {:?}", expr.kind),
    }
}

// ============================================================================
// Dict vs Set Disambiguation Tests
// ============================================================================

#[test]
fn test_empty_braces_is_dict() {
    // In Python, {} is an empty dict, not an empty set
    let expr = parse_expr("{}").unwrap();
    match expr.kind {
        ExpressionKind::Dict { .. } => {},
        _ => panic!("Expected empty dict, got {:?}", expr.kind),
    }
}

#[test]
fn test_colon_makes_it_dict() {
    let expr = parse_expr("{1: 2}").unwrap();
    match expr.kind {
        ExpressionKind::Dict { .. } => {},
        _ => panic!("Expected dict (has colon), got {:?}", expr.kind),
    }
}

#[test]
fn test_no_colon_makes_it_set() {
    let expr = parse_expr("{1, 2}").unwrap();
    match expr.kind {
        ExpressionKind::Set { .. } => {},
        _ => panic!("Expected set (no colon), got {:?}", expr.kind),
    }
}

