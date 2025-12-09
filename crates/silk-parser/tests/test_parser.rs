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

// ============================================================================
// Tuple Tests
// ============================================================================

#[test]
fn test_empty_tuple() {
    let expr = parse_expr("()").unwrap();
    match expr.kind {
        ExpressionKind::Tuple { elements } => {
            assert_eq!(elements.len(), 0, "Empty tuple should have 0 elements");
        }
        _ => panic!("Expected tuple, got {:?}", expr.kind),
    }
}

#[test]
fn test_single_element_tuple() {
    let expr = parse_expr("(42,)").unwrap();
    match expr.kind {
        ExpressionKind::Tuple { elements } => {
            assert_eq!(elements.len(), 1, "Single element tuple should have 1 element");
            match &elements[0].kind {
                ExpressionKind::Integer(42) => {},
                _ => panic!("Expected integer 42"),
            }
        }
        _ => panic!("Expected tuple, got {:?}", expr.kind),
    }
}

#[test]
fn test_two_element_tuple() {
    let expr = parse_expr("(1, 2)").unwrap();
    match expr.kind {
        ExpressionKind::Tuple { elements } => {
            assert_eq!(elements.len(), 2, "Two element tuple should have 2 elements");
        }
        _ => panic!("Expected tuple, got {:?}", expr.kind),
    }
}

#[test]
fn test_multiple_element_tuple() {
    let expr = parse_expr("(1, 2, 3, 4, 5)").unwrap();
    match expr.kind {
        ExpressionKind::Tuple { elements } => {
            assert_eq!(elements.len(), 5, "Expected 5 elements");
        }
        _ => panic!("Expected tuple, got {:?}", expr.kind),
    }
}

#[test]
fn test_tuple_with_trailing_comma() {
    let expr = parse_expr("(1, 2, 3,)").unwrap();
    match expr.kind {
        ExpressionKind::Tuple { elements } => {
            assert_eq!(elements.len(), 3, "Expected 3 elements (trailing comma ignored)");
        }
        _ => panic!("Expected tuple, got {:?}", expr.kind),
    }
}

#[test]
fn test_tuple_with_strings() {
    let expr = parse_expr(r#"("hello", "world")"#).unwrap();
    match expr.kind {
        ExpressionKind::Tuple { elements } => {
            assert_eq!(elements.len(), 2);
            match &elements[0].kind {
                ExpressionKind::String(s) => assert_eq!(s, "hello"),
                _ => panic!("Expected string"),
            }
        }
        _ => panic!("Expected tuple, got {:?}", expr.kind),
    }
}

#[test]
fn test_tuple_with_expressions() {
    let expr = parse_expr("(1 + 2, 3 * 4)").unwrap();
    match expr.kind {
        ExpressionKind::Tuple { elements } => {
            assert_eq!(elements.len(), 2);
            match &elements[0].kind {
                ExpressionKind::BinaryOp { .. } => {},
                _ => panic!("Expected binary operation"),
            }
        }
        _ => panic!("Expected tuple, got {:?}", expr.kind),
    }
}

#[test]
fn test_nested_tuple() {
    let expr = parse_expr("((1, 2), (3, 4))").unwrap();
    match expr.kind {
        ExpressionKind::Tuple { elements } => {
            assert_eq!(elements.len(), 2);
            match &elements[0].kind {
                ExpressionKind::Tuple { elements: inner } => {
                    assert_eq!(inner.len(), 2);
                }
                _ => panic!("Expected nested tuple"),
            }
        }
        _ => panic!("Expected tuple, got {:?}", expr.kind),
    }
}

#[test]
fn test_tuple_with_mixed_types() {
    let expr = parse_expr(r#"(42, "hello", True, None)"#).unwrap();
    match expr.kind {
        ExpressionKind::Tuple { elements } => {
            assert_eq!(elements.len(), 4);
        }
        _ => panic!("Expected tuple, got {:?}", expr.kind),
    }
}

#[test]
fn test_parenthesized_expression_not_tuple() {
    let expr = parse_expr("(42)").unwrap();
    match expr.kind {
        ExpressionKind::Integer(42) => {}, // Should be just an integer, not a tuple
        _ => panic!("Expected integer (parenthesized expression), got {:?}", expr.kind),
    }
}

#[test]
fn test_parenthesized_expression_complex() {
    let expr = parse_expr("(1 + 2)").unwrap();
    match expr.kind {
        ExpressionKind::BinaryOp { .. } => {}, // Should be binary op, not tuple
        _ => panic!("Expected binary operation (parenthesized), got {:?}", expr.kind),
    }
}

#[test]
fn test_nested_parentheses_not_tuple() {
    let expr = parse_expr("((42))").unwrap();
    match expr.kind {
        ExpressionKind::Integer(42) => {}, // Multiple parentheses don't make a tuple
        _ => panic!("Expected integer, got {:?}", expr.kind),
    }
}

#[test]
fn test_tuple_in_list() {
    let expr = parse_expr("[(1, 2), (3, 4)]").unwrap();
    match expr.kind {
        ExpressionKind::List { elements } => {
            assert_eq!(elements.len(), 2);
            match &elements[0].kind {
                ExpressionKind::Tuple { .. } => {},
                _ => panic!("Expected tuple in list"),
            }
        }
        _ => panic!("Expected list, got {:?}", expr.kind),
    }
}

#[test]
fn test_single_element_tuple_in_expression() {
    let expr = parse_expr("(x,)").unwrap();
    match expr.kind {
        ExpressionKind::Tuple { elements } => {
            assert_eq!(elements.len(), 1);
            match &elements[0].kind {
                ExpressionKind::Identifier(name) => assert_eq!(name, "x"),
                _ => panic!("Expected identifier"),
            }
        }
        _ => panic!("Expected single-element tuple, got {:?}", expr.kind),
    }
}

#[test]
fn test_tuple_with_function_call() {
    let expr = parse_expr("(foo(), bar())").unwrap();
    match expr.kind {
        ExpressionKind::Tuple { elements } => {
            assert_eq!(elements.len(), 2);
            match &elements[0].kind {
                ExpressionKind::Call { .. } => {},
                _ => panic!("Expected function call"),
            }
        }
        _ => panic!("Expected tuple, got {:?}", expr.kind),
    }
}

// ============================================================================
// Slice Tests
// ============================================================================

#[test]
fn test_slice_start_stop() {
    let expr = parse_expr("list[1:5]").unwrap();
    match expr.kind {
        ExpressionKind::Subscript { value, index } => {
            match &value.kind {
                ExpressionKind::Identifier(name) => assert_eq!(name, "list"),
                _ => panic!("Expected identifier 'list'"),
            }
            match &index.kind {
                ExpressionKind::Slice { lower, upper, step } => {
                    assert!(lower.is_some(), "Start should be present");
                    assert!(upper.is_some(), "Stop should be present");
                    assert!(step.is_none(), "Step should be None");
                }
                _ => panic!("Expected slice as index, got {:?}", index.kind),
            }
        }
        _ => panic!("Expected subscript, got {:?}", expr.kind),
    }
}

#[test]
fn test_slice_start_stop_step() {
    let expr = parse_expr("list[0:10:2]").unwrap();
    match expr.kind {
        ExpressionKind::Subscript { index, .. } => {
            match &index.kind {
                ExpressionKind::Slice { lower, upper, step } => {
                    assert!(lower.is_some(), "Start should be present");
                    assert!(upper.is_some(), "Stop should be present");
                    assert!(step.is_some(), "Step should be present");
                }
                _ => panic!("Expected slice"),
            }
        }
        _ => panic!("Expected subscript"),
    }
}

#[test]
fn test_slice_only_stop() {
    let expr = parse_expr("list[:5]").unwrap();
    match expr.kind {
        ExpressionKind::Subscript { index, .. } => {
            match &index.kind {
                ExpressionKind::Slice { lower, upper, step } => {
                    assert!(lower.is_none(), "Start should be None");
                    assert!(upper.is_some(), "Stop should be present");
                    assert!(step.is_none(), "Step should be None");
                }
                _ => panic!("Expected slice"),
            }
        }
        _ => panic!("Expected subscript"),
    }
}

#[test]
fn test_slice_only_start() {
    let expr = parse_expr("list[5:]").unwrap();
    match expr.kind {
        ExpressionKind::Subscript { index, .. } => {
            match &index.kind {
                ExpressionKind::Slice { lower, upper, step } => {
                    assert!(lower.is_some(), "Start should be present");
                    assert!(upper.is_none(), "Stop should be None");
                    assert!(step.is_none(), "Step should be None");
                }
                _ => panic!("Expected slice"),
            }
        }
        _ => panic!("Expected subscript"),
    }
}

#[test]
fn test_slice_all_empty() {
    let expr = parse_expr("list[:]").unwrap();
    match expr.kind {
        ExpressionKind::Subscript { index, .. } => {
            match &index.kind {
                ExpressionKind::Slice { lower, upper, step } => {
                    assert!(lower.is_none(), "Start should be None");
                    assert!(upper.is_none(), "Stop should be None");
                    assert!(step.is_none(), "Step should be None");
                }
                _ => panic!("Expected slice"),
            }
        }
        _ => panic!("Expected subscript"),
    }
}

#[test]
fn test_slice_with_step_only() {
    let expr = parse_expr("list[::2]").unwrap();
    match expr.kind {
        ExpressionKind::Subscript { index, .. } => {
            match &index.kind {
                ExpressionKind::Slice { lower, upper, step } => {
                    assert!(lower.is_none(), "Start should be None");
                    assert!(upper.is_none(), "Stop should be None");
                    assert!(step.is_some(), "Step should be present");
                }
                _ => panic!("Expected slice"),
            }
        }
        _ => panic!("Expected subscript"),
    }
}

#[test]
fn test_slice_stop_and_step() {
    let expr = parse_expr("list[:10:2]").unwrap();
    match expr.kind {
        ExpressionKind::Subscript { index, .. } => {
            match &index.kind {
                ExpressionKind::Slice { lower, upper, step } => {
                    assert!(lower.is_none(), "Start should be None");
                    assert!(upper.is_some(), "Stop should be present");
                    assert!(step.is_some(), "Step should be present");
                }
                _ => panic!("Expected slice"),
            }
        }
        _ => panic!("Expected subscript"),
    }
}

#[test]
fn test_slice_start_and_step() {
    let expr = parse_expr("list[5::2]").unwrap();
    match expr.kind {
        ExpressionKind::Subscript { index, .. } => {
            match &index.kind {
                ExpressionKind::Slice { lower, upper, step } => {
                    assert!(lower.is_some(), "Start should be present");
                    assert!(upper.is_none(), "Stop should be None");
                    assert!(step.is_some(), "Step should be present");
                }
                _ => panic!("Expected slice"),
            }
        }
        _ => panic!("Expected subscript"),
    }
}

#[test]
fn test_slice_with_negative_indices() {
    let expr = parse_expr("list[-5:-1]").unwrap();
    match expr.kind {
        ExpressionKind::Subscript { index, .. } => {
            match &index.kind {
                ExpressionKind::Slice { lower, upper, .. } => {
                    assert!(lower.is_some(), "Start should be present");
                    assert!(upper.is_some(), "Stop should be present");
                    // Check they are unary minus expressions
                    match &lower.as_ref().unwrap().kind {
                        ExpressionKind::UnaryOp { op, .. } => {
                            assert_eq!(*op, UnaryOperator::USub);
                        }
                        _ => panic!("Expected unary minus for negative index"),
                    }
                }
                _ => panic!("Expected slice"),
            }
        }
        _ => panic!("Expected subscript"),
    }
}

#[test]
fn test_slice_with_expressions() {
    let expr = parse_expr("list[x:y:z]").unwrap();
    match expr.kind {
        ExpressionKind::Subscript { index, .. } => {
            match &index.kind {
                ExpressionKind::Slice { lower, upper, step } => {
                    assert!(lower.is_some());
                    assert!(upper.is_some());
                    assert!(step.is_some());
                    match &lower.as_ref().unwrap().kind {
                        ExpressionKind::Identifier(name) => assert_eq!(name, "x"),
                        _ => panic!("Expected identifier x"),
                    }
                }
                _ => panic!("Expected slice"),
            }
        }
        _ => panic!("Expected subscript"),
    }
}

#[test]
fn test_slice_with_computed_values() {
    let expr = parse_expr("list[i+1:i+10:2]").unwrap();
    match expr.kind {
        ExpressionKind::Subscript { index, .. } => {
            match &index.kind {
                ExpressionKind::Slice { lower, upper, step } => {
                    assert!(lower.is_some());
                    assert!(upper.is_some());
                    assert!(step.is_some());
                    match &lower.as_ref().unwrap().kind {
                        ExpressionKind::BinaryOp { .. } => {},
                        _ => panic!("Expected binary operation"),
                    }
                }
                _ => panic!("Expected slice"),
            }
        }
        _ => panic!("Expected subscript"),
    }
}

#[test]
fn test_slice_reverse() {
    let expr = parse_expr("list[::-1]").unwrap();
    match expr.kind {
        ExpressionKind::Subscript { index, .. } => {
            match &index.kind {
                ExpressionKind::Slice { lower, upper, step } => {
                    assert!(lower.is_none());
                    assert!(upper.is_none());
                    assert!(step.is_some(), "Step should be present for reverse");
                }
                _ => panic!("Expected slice"),
            }
        }
        _ => panic!("Expected subscript"),
    }
}

#[test]
fn test_regular_subscript_still_works() {
    let expr = parse_expr("list[5]").unwrap();
    match expr.kind {
        ExpressionKind::Subscript { value, index } => {
            match &value.kind {
                ExpressionKind::Identifier(name) => assert_eq!(name, "list"),
                _ => panic!("Expected identifier"),
            }
            match &index.kind {
                ExpressionKind::Integer(5) => {},
                _ => panic!("Expected integer index, not slice"),
            }
        }
        _ => panic!("Expected subscript"),
    }
}

#[test]
fn test_chained_subscript_with_slice() {
    let expr = parse_expr("matrix[0][1:3]").unwrap();
    match expr.kind {
        ExpressionKind::Subscript { value, index } => {
            // Outer subscript should have a slice
            match &index.kind {
                ExpressionKind::Slice { .. } => {},
                _ => panic!("Expected slice in second subscript"),
            }
            // Inner should be another subscript
            match &value.kind {
                ExpressionKind::Subscript { .. } => {},
                _ => panic!("Expected nested subscript"),
            }
        }
        _ => panic!("Expected subscript"),
    }
}

// ============================================================================
// Lambda Expression Tests
// ============================================================================

#[test]
fn test_lambda_no_params() {
    let expr = parse_expr("lambda: 42").unwrap();
    match expr.kind {
        ExpressionKind::Lambda { params, body } => {
            assert_eq!(params.len(), 0, "Lambda should have no parameters");
            match &body.kind {
                ExpressionKind::Integer(42) => {},
                _ => panic!("Expected integer 42 in lambda body"),
            }
        }
        _ => panic!("Expected lambda expression, got {:?}", expr.kind),
    }
}

#[test]
fn test_lambda_single_param() {
    let expr = parse_expr("lambda x: x + 1").unwrap();
    match expr.kind {
        ExpressionKind::Lambda { params, body } => {
            assert_eq!(params.len(), 1, "Lambda should have 1 parameter");
            assert_eq!(params[0].name, "x");
            match &body.kind {
                ExpressionKind::BinaryOp { .. } => {},
                _ => panic!("Expected binary operation in lambda body"),
            }
        }
        _ => panic!("Expected lambda expression"),
    }
}

#[test]
fn test_lambda_multiple_params() {
    let expr = parse_expr("lambda x, y: x * y").unwrap();
    match expr.kind {
        ExpressionKind::Lambda { params, body } => {
            assert_eq!(params.len(), 2, "Lambda should have 2 parameters");
            assert_eq!(params[0].name, "x");
            assert_eq!(params[1].name, "y");
            match &body.kind {
                ExpressionKind::BinaryOp { .. } => {},
                _ => panic!("Expected binary operation in lambda body"),
            }
        }
        _ => panic!("Expected lambda expression"),
    }
}

#[test]
fn test_lambda_three_params() {
    let expr = parse_expr("lambda x, y, z: x + y + z").unwrap();
    match expr.kind {
        ExpressionKind::Lambda { params, .. } => {
            assert_eq!(params.len(), 3, "Lambda should have 3 parameters");
            assert_eq!(params[0].name, "x");
            assert_eq!(params[1].name, "y");
            assert_eq!(params[2].name, "z");
        }
        _ => panic!("Expected lambda expression"),
    }
}

#[test]
fn test_lambda_with_string_body() {
    let expr = parse_expr(r#"lambda name: "Hello, " + name"#).unwrap();
    match expr.kind {
        ExpressionKind::Lambda { params, body } => {
            assert_eq!(params.len(), 1);
            assert_eq!(params[0].name, "name");
            match &body.kind {
                ExpressionKind::BinaryOp { .. } => {},
                _ => panic!("Expected binary operation"),
            }
        }
        _ => panic!("Expected lambda expression"),
    }
}

#[test]
fn test_lambda_with_comparison() {
    let expr = parse_expr("lambda x: x > 0").unwrap();
    match expr.kind {
        ExpressionKind::Lambda { params, body } => {
            assert_eq!(params.len(), 1);
            match &body.kind {
                ExpressionKind::Compare { .. } => {},
                _ => panic!("Expected comparison in lambda body"),
            }
        }
        _ => panic!("Expected lambda expression"),
    }
}

#[test]
fn test_lambda_with_function_call() {
    let expr = parse_expr("lambda x: foo(x)").unwrap();
    match expr.kind {
        ExpressionKind::Lambda { params, body } => {
            assert_eq!(params.len(), 1);
            match &body.kind {
                ExpressionKind::Call { .. } => {},
                _ => panic!("Expected function call in lambda body"),
            }
        }
        _ => panic!("Expected lambda expression"),
    }
}

#[test]
fn test_lambda_in_function_call() {
    let expr = parse_expr("map(lambda x: x * 2, numbers)").unwrap();
    match expr.kind {
        ExpressionKind::Call { func, args, .. } => {
            match &func.kind {
                ExpressionKind::Identifier(name) => assert_eq!(name, "map"),
                _ => panic!("Expected identifier 'map'"),
            }
            assert_eq!(args.len(), 2, "Expected 2 arguments");
            match &args[0].kind {
                ExpressionKind::Lambda { .. } => {},
                _ => panic!("Expected lambda as first argument"),
            }
        }
        _ => panic!("Expected function call"),
    }
}

#[test]
fn test_lambda_in_list() {
    let expr = parse_expr("[lambda x: x + 1, lambda x: x * 2]").unwrap();
    match expr.kind {
        ExpressionKind::List { elements } => {
            assert_eq!(elements.len(), 2);
            match &elements[0].kind {
                ExpressionKind::Lambda { .. } => {},
                _ => panic!("Expected lambda in list"),
            }
            match &elements[1].kind {
                ExpressionKind::Lambda { .. } => {},
                _ => panic!("Expected lambda in list"),
            }
        }
        _ => panic!("Expected list"),
    }
}

#[test]
fn test_lambda_with_complex_body() {
    let expr = parse_expr("lambda x: x * 2 + 1").unwrap();
    match expr.kind {
        ExpressionKind::Lambda { params, body: _ } => {
            assert_eq!(params.len(), 1);
            // Body is a complex expression with multiple operations
        }
        _ => panic!("Expected lambda expression"),
    }
}

#[test]
fn test_nested_lambda() {
    let expr = parse_expr("lambda x: lambda y: x + y").unwrap();
    match expr.kind {
        ExpressionKind::Lambda { params, body } => {
            assert_eq!(params.len(), 1);
            assert_eq!(params[0].name, "x");
            // Body should be another lambda
            match &body.kind {
                ExpressionKind::Lambda { params: inner_params, .. } => {
                    assert_eq!(inner_params.len(), 1);
                    assert_eq!(inner_params[0].name, "y");
                }
                _ => panic!("Expected nested lambda in body"),
            }
        }
        _ => panic!("Expected lambda expression"),
    }
}

#[test]
fn test_lambda_with_tuple_unpack() {
    let expr = parse_expr("lambda x, y: (x, y)").unwrap();
    match expr.kind {
        ExpressionKind::Lambda { params, body } => {
            assert_eq!(params.len(), 2);
            match &body.kind {
                ExpressionKind::Tuple { elements } => {
                    assert_eq!(elements.len(), 2);
                }
                _ => panic!("Expected tuple in lambda body"),
            }
        }
        _ => panic!("Expected lambda expression"),
    }
}

#[test]
fn test_lambda_with_logical_ops() {
    let expr = parse_expr("lambda x, y: x and y").unwrap();
    match expr.kind {
        ExpressionKind::Lambda { params, body } => {
            assert_eq!(params.len(), 2);
            match &body.kind {
                ExpressionKind::LogicalOp { .. } => {},
                _ => panic!("Expected logical operation in lambda body"),
            }
        }
        _ => panic!("Expected lambda expression"),
    }
}

#[test]
fn test_lambda_with_subscript() {
    let expr = parse_expr("lambda lst, i: lst[i]").unwrap();
    match expr.kind {
        ExpressionKind::Lambda { params, body } => {
            assert_eq!(params.len(), 2);
            match &body.kind {
                ExpressionKind::Subscript { .. } => {},
                _ => panic!("Expected subscript in lambda body"),
            }
        }
        _ => panic!("Expected lambda expression"),
    }
}

// ============================================================================
// Ternary/Conditional Expression Tests
// ============================================================================

#[test]
fn test_ternary_basic() {
    let expr = parse_expr("x if condition else y").unwrap();
    match expr.kind {
        ExpressionKind::IfExp { test, body, orelse } => {
            match &body.kind {
                ExpressionKind::Identifier(name) => assert_eq!(name, "x"),
                _ => panic!("Expected identifier 'x' in body"),
            }
            match &test.kind {
                ExpressionKind::Identifier(name) => assert_eq!(name, "condition"),
                _ => panic!("Expected identifier 'condition' in test"),
            }
            match &orelse.kind {
                ExpressionKind::Identifier(name) => assert_eq!(name, "y"),
                _ => panic!("Expected identifier 'y' in orelse"),
            }
        }
        _ => panic!("Expected ternary expression, got {:?}", expr.kind),
    }
}

#[test]
fn test_ternary_with_literals() {
    let expr = parse_expr("1 if True else 0").unwrap();
    match expr.kind {
        ExpressionKind::IfExp { test, body, orelse } => {
            match &body.kind {
                ExpressionKind::Integer(1) => {},
                _ => panic!("Expected integer 1 in body"),
            }
            match &test.kind {
                ExpressionKind::Boolean(true) => {},
                _ => panic!("Expected True in test"),
            }
            match &orelse.kind {
                ExpressionKind::Integer(0) => {},
                _ => panic!("Expected integer 0 in orelse"),
            }
        }
        _ => panic!("Expected ternary expression"),
    }
}

#[test]
fn test_ternary_with_comparison() {
    let expr = parse_expr("positive if x > 0 else negative").unwrap();
    match expr.kind {
        ExpressionKind::IfExp { test, .. } => {
            match &test.kind {
                ExpressionKind::Compare { .. } => {},
                _ => panic!("Expected comparison in test"),
            }
        }
        _ => panic!("Expected ternary expression"),
    }
}

#[test]
fn test_ternary_with_expressions() {
    let expr = parse_expr("x + 1 if x > 0 else x - 1").unwrap();
    match expr.kind {
        ExpressionKind::IfExp { test, body, orelse } => {
            match &body.kind {
                ExpressionKind::BinaryOp { .. } => {},
                _ => panic!("Expected binary op in body"),
            }
            match &test.kind {
                ExpressionKind::Compare { .. } => {},
                _ => panic!("Expected comparison in test"),
            }
            match &orelse.kind {
                ExpressionKind::BinaryOp { .. } => {},
                _ => panic!("Expected binary op in orelse"),
            }
        }
        _ => panic!("Expected ternary expression"),
    }
}

#[test]
fn test_nested_ternary() {
    let expr = parse_expr("a if x > 0 else b if x < 0 else c").unwrap();
    match expr.kind {
        ExpressionKind::IfExp { orelse, .. } => {
            // The orelse should be another ternary
            match &orelse.kind {
                ExpressionKind::IfExp { .. } => {},
                _ => panic!("Expected nested ternary in orelse"),
            }
        }
        _ => panic!("Expected ternary expression"),
    }
}

#[test]
fn test_ternary_in_function_call() {
    let expr = parse_expr("foo(x if cond else y)").unwrap();
    match expr.kind {
        ExpressionKind::Call { args, .. } => {
            assert_eq!(args.len(), 1);
            match &args[0].kind {
                ExpressionKind::IfExp { .. } => {},
                _ => panic!("Expected ternary in function argument"),
            }
        }
        _ => panic!("Expected function call"),
    }
}

#[test]
fn test_ternary_in_list() {
    let expr = parse_expr("[x if x > 0 else 0]").unwrap();
    match expr.kind {
        ExpressionKind::List { elements } => {
            assert_eq!(elements.len(), 1);
            match &elements[0].kind {
                ExpressionKind::IfExp { .. } => {},
                _ => panic!("Expected ternary in list"),
            }
        }
        _ => panic!("Expected list"),
    }
}

#[test]
fn test_ternary_with_strings() {
    let expr = parse_expr(r#""yes" if flag else "no""#).unwrap();
    match expr.kind {
        ExpressionKind::IfExp { body, orelse, .. } => {
            match &body.kind {
                ExpressionKind::String(s) => assert_eq!(s, "yes"),
                _ => panic!("Expected string in body"),
            }
            match &orelse.kind {
                ExpressionKind::String(s) => assert_eq!(s, "no"),
                _ => panic!("Expected string in orelse"),
            }
        }
        _ => panic!("Expected ternary expression"),
    }
}

#[test]
fn test_ternary_with_function_calls() {
    let expr = parse_expr("foo() if condition else bar()").unwrap();
    match expr.kind {
        ExpressionKind::IfExp { body, orelse, .. } => {
            match &body.kind {
                ExpressionKind::Call { .. } => {},
                _ => panic!("Expected function call in body"),
            }
            match &orelse.kind {
                ExpressionKind::Call { .. } => {},
                _ => panic!("Expected function call in orelse"),
            }
        }
        _ => panic!("Expected ternary expression"),
    }
}

#[test]
fn test_ternary_with_logical_ops() {
    let expr = parse_expr("result if x > 0 and y > 0 else default").unwrap();
    match expr.kind {
        ExpressionKind::IfExp { test, .. } => {
            match &test.kind {
                ExpressionKind::LogicalOp { .. } => {},
                _ => panic!("Expected logical operation in test"),
            }
        }
        _ => panic!("Expected ternary expression"),
    }
}

#[test]
fn test_ternary_in_assignment() {
    let stmt = parse_stmt("result = positive if x > 0 else negative").unwrap();
    match stmt.kind {
        StatementKind::Assign { value, .. } => {
            match &value.kind {
                ExpressionKind::IfExp { .. } => {},
                _ => panic!("Expected ternary in assignment"),
            }
        }
        _ => panic!("Expected assignment statement"),
    }
}

#[test]
fn test_ternary_with_subscript() {
    let expr = parse_expr("lst[0] if lst else None").unwrap();
    match expr.kind {
        ExpressionKind::IfExp { body, .. } => {
            match &body.kind {
                ExpressionKind::Subscript { .. } => {},
                _ => panic!("Expected subscript in body"),
            }
        }
        _ => panic!("Expected ternary expression"),
    }
}

#[test]
fn test_ternary_with_lambda() {
    let expr = parse_expr("(lambda: x) if flag else (lambda: y)").unwrap();
    match expr.kind {
        ExpressionKind::IfExp { body, orelse, .. } => {
            match &body.kind {
                ExpressionKind::Lambda { .. } => {},
                _ => panic!("Expected lambda in body"),
            }
            match &orelse.kind {
                ExpressionKind::Lambda { .. } => {},
                _ => panic!("Expected lambda in orelse"),
            }
        }
        _ => panic!("Expected ternary expression"),
    }
}

#[test]
fn test_ternary_max_pattern() {
    let expr = parse_expr("a if a > b else b").unwrap();
    match expr.kind {
        ExpressionKind::IfExp { test, body, orelse } => {
            // Common pattern: max(a, b) as ternary
            match &test.kind {
                ExpressionKind::Compare { .. } => {},
                _ => panic!("Expected comparison"),
            }
            match &body.kind {
                ExpressionKind::Identifier(name) => assert_eq!(name, "a"),
                _ => panic!("Expected identifier a"),
            }
            match &orelse.kind {
                ExpressionKind::Identifier(name) => assert_eq!(name, "b"),
                _ => panic!("Expected identifier b"),
            }
        }
        _ => panic!("Expected ternary expression"),
    }
}

// ============================================================================
// Keyword Argument Tests
// ============================================================================

#[test]
fn test_function_call_with_keyword_arg() {
    let expr = parse_expr("func(x=1)").unwrap();
    match expr.kind {
        ExpressionKind::Call { args, keywords, .. } => {
            assert_eq!(args.len(), 0);
            assert_eq!(keywords.len(), 1);
            assert_eq!(keywords[0].arg, Some("x".to_string()));
        }
        _ => panic!("Expected function call"),
    }
}

#[test]
fn test_function_call_mixed_args() {
    let expr = parse_expr("func(1, 2, x=3, y=4)").unwrap();
    match expr.kind {
        ExpressionKind::Call { args, keywords, .. } => {
            assert_eq!(args.len(), 2);
            assert_eq!(keywords.len(), 2);
            assert_eq!(keywords[0].arg, Some("x".to_string()));
            assert_eq!(keywords[1].arg, Some("y".to_string()));
        }
        _ => panic!("Expected function call"),
    }
}

#[test]
fn test_function_call_kwargs_unpack() {
    let expr = parse_expr("func(**options)").unwrap();
    match expr.kind {
        ExpressionKind::Call { args, keywords, .. } => {
            assert_eq!(args.len(), 0);
            assert_eq!(keywords.len(), 1);
            assert_eq!(keywords[0].arg, None); // None means **kwargs
        }
        _ => panic!("Expected function call"),
    }
}

#[test]
fn test_function_call_mixed_with_kwargs_unpack() {
    let expr = parse_expr("func(1, x=2, **options)").unwrap();
    match expr.kind {
        ExpressionKind::Call { args, keywords, .. } => {
            assert_eq!(args.len(), 1);
            assert_eq!(keywords.len(), 2);
            assert_eq!(keywords[0].arg, Some("x".to_string()));
            assert_eq!(keywords[1].arg, None);
        }
        _ => panic!("Expected function call"),
    }
}

#[test]
fn test_keyword_with_expression() {
    let expr = parse_expr("func(x=a + b, y=c * 2)").unwrap();
    match expr.kind {
        ExpressionKind::Call { keywords, .. } => {
            assert_eq!(keywords.len(), 2);
            match &keywords[0].value.kind {
                ExpressionKind::BinaryOp { .. } => {},
                _ => panic!("Expected binary op in keyword value"),
            }
        }
        _ => panic!("Expected function call"),
    }
}

#[test]
fn test_keyword_with_function_call() {
    let expr = parse_expr("func(x=other())").unwrap();
    match expr.kind {
        ExpressionKind::Call { keywords, .. } => {
            assert_eq!(keywords.len(), 1);
            match &keywords[0].value.kind {
                ExpressionKind::Call { .. } => {},
                _ => panic!("Expected function call in keyword value"),
            }
        }
        _ => panic!("Expected function call"),
    }
}

#[test]
fn test_keyword_with_string() {
    let expr = parse_expr(r#"func(name="Alice", age=30)"#).unwrap();
    match expr.kind {
        ExpressionKind::Call { keywords, .. } => {
            assert_eq!(keywords.len(), 2);
            assert_eq!(keywords[0].arg, Some("name".to_string()));
            assert_eq!(keywords[1].arg, Some("age".to_string()));
            match &keywords[0].value.kind {
                ExpressionKind::String(s) => assert_eq!(s, "Alice"),
                _ => panic!("Expected string"),
            }
        }
        _ => panic!("Expected function call"),
    }
}

#[test]
fn test_keyword_with_list() {
    let expr = parse_expr("func(items=[1, 2, 3])").unwrap();
    match expr.kind {
        ExpressionKind::Call { keywords, .. } => {
            assert_eq!(keywords.len(), 1);
            match &keywords[0].value.kind {
                ExpressionKind::List { elements } => assert_eq!(elements.len(), 3),
                _ => panic!("Expected list in keyword value"),
            }
        }
        _ => panic!("Expected function call"),
    }
}

#[test]
fn test_keyword_with_dict() {
    let expr = parse_expr("func(options={'a': 1})").unwrap();
    match expr.kind {
        ExpressionKind::Call { keywords, .. } => {
            assert_eq!(keywords.len(), 1);
            match &keywords[0].value.kind {
                ExpressionKind::Dict { .. } => {},
                _ => panic!("Expected dict in keyword value"),
            }
        }
        _ => panic!("Expected function call"),
    }
}

#[test]
fn test_keyword_with_lambda() {
    let expr = parse_expr("func(key=lambda x: x.lower())").unwrap();
    match expr.kind {
        ExpressionKind::Call { keywords, .. } => {
            assert_eq!(keywords.len(), 1);
            match &keywords[0].value.kind {
                ExpressionKind::Lambda { .. } => {},
                _ => panic!("Expected lambda in keyword value"),
            }
        }
        _ => panic!("Expected function call"),
    }
}

#[test]
fn test_keyword_with_ternary() {
    let expr = parse_expr("func(value=x if condition else y)").unwrap();
    match expr.kind {
        ExpressionKind::Call { keywords, .. } => {
            assert_eq!(keywords.len(), 1);
            match &keywords[0].value.kind {
                ExpressionKind::IfExp { .. } => {},
                _ => panic!("Expected ternary in keyword value"),
            }
        }
        _ => panic!("Expected function call"),
    }
}

#[test]
fn test_keyword_with_comprehension() {
    let expr = parse_expr("func(items=[x * 2 for x in range(10)])").unwrap();
    match expr.kind {
        ExpressionKind::Call { keywords, .. } => {
            assert_eq!(keywords.len(), 1);
            match &keywords[0].value.kind {
                ExpressionKind::ListComp { .. } => {},
                _ => panic!("Expected list comprehension in keyword value"),
            }
        }
        _ => panic!("Expected function call"),
    }
}

#[test]
fn test_many_keyword_args() {
    let expr = parse_expr("func(a=1, b=2, c=3, d=4, e=5)").unwrap();
    match expr.kind {
        ExpressionKind::Call { keywords, .. } => {
            assert_eq!(keywords.len(), 5);
            assert_eq!(keywords[0].arg, Some("a".to_string()));
            assert_eq!(keywords[4].arg, Some("e".to_string()));
        }
        _ => panic!("Expected function call"),
    }
}

#[test]
fn test_nested_call_with_keywords() {
    let expr = parse_expr("outer(inner(x=1), y=2)").unwrap();
    match expr.kind {
        ExpressionKind::Call { args, keywords, .. } => {
            assert_eq!(args.len(), 1);
            assert_eq!(keywords.len(), 1);
            // Check inner call
            match &args[0].kind {
                ExpressionKind::Call { keywords: inner_kw, .. } => {
                    assert_eq!(inner_kw.len(), 1);
                    assert_eq!(inner_kw[0].arg, Some("x".to_string()));
                }
                _ => panic!("Expected inner function call"),
            }
        }
        _ => panic!("Expected function call"),
    }
}

#[test]
fn test_keyword_in_method_call() {
    let expr = parse_expr("obj.method(x=1, y=2)").unwrap();
    match expr.kind {
        ExpressionKind::Call { func, keywords, .. } => {
            assert_eq!(keywords.len(), 2);
            match &func.kind {
                ExpressionKind::Attribute { .. } => {},
                _ => panic!("Expected attribute access"),
            }
        }
        _ => panic!("Expected function call"),
    }
}

#[test]
fn test_positional_after_keyword_error() {
    let result = parse_expr("func(x=1, 2)");
    assert!(result.is_err(), "Should error: positional after keyword");
}

// ============================================================================
// Function Parameter Tests (*args, **kwargs)
// ============================================================================

#[test]
fn test_function_with_args_vararg() {
    let stmt = parse_stmt("def func(*args):\n    pass").unwrap();
    match stmt.kind {
        StatementKind::FunctionDef { params, .. } => {
            assert_eq!(params.args.len(), 0);
            assert!(params.vararg.is_some());
            assert_eq!(params.vararg.as_ref().unwrap().name, "args");
            assert!(params.kwarg.is_none());
        }
        _ => panic!("Expected function definition"),
    }
}

#[test]
fn test_function_with_kwargs() {
    let stmt = parse_stmt("def func(**kwargs):\n    pass").unwrap();
    match stmt.kind {
        StatementKind::FunctionDef { params, .. } => {
            assert_eq!(params.args.len(), 0);
            assert!(params.vararg.is_none());
            assert!(params.kwarg.is_some());
            assert_eq!(params.kwarg.as_ref().unwrap().name, "kwargs");
        }
        _ => panic!("Expected function definition"),
    }
}

#[test]
fn test_function_mixed_params_with_vararg() {
    let stmt = parse_stmt("def func(a, b, *args):\n    pass").unwrap();
    match stmt.kind {
        StatementKind::FunctionDef { params, .. } => {
            assert_eq!(params.args.len(), 2);
            assert_eq!(params.args[0].name, "a");
            assert_eq!(params.args[1].name, "b");
            assert!(params.vararg.is_some());
            assert_eq!(params.vararg.as_ref().unwrap().name, "args");
        }
        _ => panic!("Expected function definition"),
    }
}

#[test]
fn test_function_mixed_params_with_kwargs() {
    let stmt = parse_stmt("def func(a, b, **kwargs):\n    pass").unwrap();
    match stmt.kind {
        StatementKind::FunctionDef { params, .. } => {
            assert_eq!(params.args.len(), 2);
            assert!(params.vararg.is_none());
            assert!(params.kwarg.is_some());
        }
        _ => panic!("Expected function definition"),
    }
}

#[test]
fn test_function_all_param_types() {
    let stmt = parse_stmt("def func(a, b, *args, **kwargs):\n    pass").unwrap();
    match stmt.kind {
        StatementKind::FunctionDef { params, .. } => {
            assert_eq!(params.args.len(), 2);
            assert_eq!(params.args[0].name, "a");
            assert_eq!(params.args[1].name, "b");
            assert!(params.vararg.is_some());
            assert_eq!(params.vararg.as_ref().unwrap().name, "args");
            assert!(params.kwarg.is_some());
            assert_eq!(params.kwarg.as_ref().unwrap().name, "kwargs");
        }
        _ => panic!("Expected function definition"),
    }
}

#[test]
fn test_function_vararg_with_type_annotation() {
    let stmt = parse_stmt("def func(*args: int):\n    pass").unwrap();
    match stmt.kind {
        StatementKind::FunctionDef { params, .. } => {
            assert!(params.vararg.is_some());
            let vararg = params.vararg.as_ref().unwrap();
            assert_eq!(vararg.name, "args");
            assert!(vararg.annotation.is_some());
        }
        _ => panic!("Expected function definition"),
    }
}

#[test]
fn test_function_kwargs_with_type_annotation() {
    let stmt = parse_stmt("def func(**kwargs: dict):\n    pass").unwrap();
    match stmt.kind {
        StatementKind::FunctionDef { params, .. } => {
            assert!(params.kwarg.is_some());
            let kwarg = params.kwarg.as_ref().unwrap();
            assert_eq!(kwarg.name, "kwargs");
            assert!(kwarg.annotation.is_some());
        }
        _ => panic!("Expected function definition"),
    }
}

#[test]
fn test_function_with_defaults_and_vararg() {
    let stmt = parse_stmt("def func(a, b=10, *args):\n    pass").unwrap();
    match stmt.kind {
        StatementKind::FunctionDef { params, .. } => {
            assert_eq!(params.args.len(), 2);
            assert!(params.args[0].default.is_none());
            assert!(params.args[1].default.is_some());
            assert!(params.vararg.is_some());
        }
        _ => panic!("Expected function definition"),
    }
}

// ==================== Decorator Tests ====================

#[test]
fn test_simple_decorator() {
    let stmt = parse_stmt("@decorator\ndef func():\n    pass").unwrap();
    match stmt.kind {
        StatementKind::FunctionDef { name, decorator_list, .. } => {
            assert_eq!(name, "func");
            assert_eq!(decorator_list.len(), 1);
            match &decorator_list[0].kind {
                ExpressionKind::Identifier(name) => assert_eq!(name, "decorator"),
                _ => panic!("Expected identifier decorator"),
            }
        }
        _ => panic!("Expected function definition"),
    }
}

#[test]
fn test_decorator_with_call() {
    let stmt = parse_stmt("@decorator(arg1, arg2)\ndef func():\n    pass").unwrap();
    match stmt.kind {
        StatementKind::FunctionDef { name, decorator_list, .. } => {
            assert_eq!(name, "func");
            assert_eq!(decorator_list.len(), 1);
            match &decorator_list[0].kind {
                ExpressionKind::Call { func, args, .. } => {
                    match &func.kind {
                        ExpressionKind::Identifier(name) => assert_eq!(name, "decorator"),
                        _ => panic!("Expected identifier in call"),
                    }
                    assert_eq!(args.len(), 2);
                }
                _ => panic!("Expected call decorator"),
            }
        }
        _ => panic!("Expected function definition"),
    }
}

#[test]
fn test_multiple_decorators() {
    let stmt = parse_stmt("@decorator1\n@decorator2\n@decorator3\ndef func():\n    pass").unwrap();
    match stmt.kind {
        StatementKind::FunctionDef { name, decorator_list, .. } => {
            assert_eq!(name, "func");
            assert_eq!(decorator_list.len(), 3);
            
            match &decorator_list[0].kind {
                ExpressionKind::Identifier(name) => assert_eq!(name, "decorator1"),
                _ => panic!("Expected identifier decorator"),
            }
            match &decorator_list[1].kind {
                ExpressionKind::Identifier(name) => assert_eq!(name, "decorator2"),
                _ => panic!("Expected identifier decorator"),
            }
            match &decorator_list[2].kind {
                ExpressionKind::Identifier(name) => assert_eq!(name, "decorator3"),
                _ => panic!("Expected identifier decorator"),
            }
        }
        _ => panic!("Expected function definition"),
    }
}

#[test]
fn test_decorator_with_arguments() {
    let stmt = parse_stmt("@decorator(timeout=30)\ndef func():\n    pass").unwrap();
    match stmt.kind {
        StatementKind::FunctionDef { decorator_list, .. } => {
            assert_eq!(decorator_list.len(), 1);
            match &decorator_list[0].kind {
                ExpressionKind::Call { keywords, .. } => {
                    assert_eq!(keywords.len(), 1);
                    assert_eq!(keywords[0].arg.as_ref().unwrap(), "timeout");
                }
                _ => panic!("Expected call decorator"),
            }
        }
        _ => panic!("Expected function definition"),
    }
}

#[test]
fn test_decorator_with_attribute() {
    let stmt = parse_stmt("@module.decorator\ndef func():\n    pass").unwrap();
    match stmt.kind {
        StatementKind::FunctionDef { decorator_list, .. } => {
            assert_eq!(decorator_list.len(), 1);
            match &decorator_list[0].kind {
                ExpressionKind::Attribute { value, attr, .. } => {
                    match &value.kind {
                        ExpressionKind::Identifier(name) => assert_eq!(name, "module"),
                        _ => panic!("Expected identifier"),
                    }
                    assert_eq!(attr, "decorator");
                }
                _ => panic!("Expected attribute decorator"),
            }
        }
        _ => panic!("Expected function definition"),
    }
}

#[test]
fn test_class_decorator() {
    let stmt = parse_stmt("@dataclass\nclass MyClass:\n    pass").unwrap();
    match stmt.kind {
        StatementKind::ClassDef { name, decorator_list, .. } => {
            assert_eq!(name, "MyClass");
            assert_eq!(decorator_list.len(), 1);
            match &decorator_list[0].kind {
                ExpressionKind::Identifier(name) => assert_eq!(name, "dataclass"),
                _ => panic!("Expected identifier decorator"),
            }
        }
        _ => panic!("Expected class definition"),
    }
}

#[test]
fn test_class_multiple_decorators() {
    let stmt = parse_stmt("@decorator1\n@decorator2\nclass MyClass:\n    pass").unwrap();
    match stmt.kind {
        StatementKind::ClassDef { name, decorator_list, .. } => {
            assert_eq!(name, "MyClass");
            assert_eq!(decorator_list.len(), 2);
        }
        _ => panic!("Expected class definition"),
    }
}

#[test]
fn test_decorator_with_complex_call() {
    let stmt = parse_stmt("@decorator(1, 2, x=3, **opts)\ndef func():\n    pass").unwrap();
    match stmt.kind {
        StatementKind::FunctionDef { decorator_list, .. } => {
            assert_eq!(decorator_list.len(), 1);
            match &decorator_list[0].kind {
                ExpressionKind::Call { args, keywords, .. } => {
                    assert_eq!(args.len(), 2);
                    assert_eq!(keywords.len(), 2); // x=3 and **opts
                }
                _ => panic!("Expected call decorator"),
            }
        }
        _ => panic!("Expected function definition"),
    }
}

// ==================== Walrus Operator Tests ====================

#[test]
fn test_walrus_basic() {
    let expr = parse_expr("x := 10").unwrap();
    match expr.kind {
        ExpressionKind::NamedExpr { target, value } => {
            match target.kind {
                ExpressionKind::Identifier(ref name) => assert_eq!(name, "x"),
                _ => panic!("Expected identifier target"),
            }
            match value.kind {
                ExpressionKind::Integer(v) => assert_eq!(v, 10),
                _ => panic!("Expected integer value"),
            }
        }
        _ => panic!("Expected named expression"),
    }
}

#[test]
fn test_walrus_in_if() {
    let stmt = parse_stmt("if x := get_value():\n    pass").unwrap();
    match stmt.kind {
        StatementKind::If { test, .. } => {
            match test.kind {
                ExpressionKind::NamedExpr { target, value } => {
                    match target.kind {
                        ExpressionKind::Identifier(ref name) => assert_eq!(name, "x"),
                        _ => panic!("Expected identifier"),
                    }
                    match value.kind {
                        ExpressionKind::Call { .. } => {},
                        _ => panic!("Expected call"),
                    }
                }
                _ => panic!("Expected named expression in if test"),
            }
        }
        _ => panic!("Expected if statement"),
    }
}

#[test]
fn test_walrus_in_while() {
    let stmt = parse_stmt("while line := file.readline():\n    pass").unwrap();
    match stmt.kind {
        StatementKind::While { test, .. } => {
            match test.kind {
                ExpressionKind::NamedExpr { .. } => {},
                _ => panic!("Expected named expression in while test"),
            }
        }
        _ => panic!("Expected while statement"),
    }
}

#[test]
fn test_walrus_in_list() {
    let expr = parse_expr("[y := 5, y + 1, y + 2]").unwrap();
    match expr.kind {
        ExpressionKind::List { elements } => {
            assert_eq!(elements.len(), 3);
            match &elements[0].kind {
                ExpressionKind::NamedExpr { .. } => {},
                _ => panic!("Expected named expression as first element"),
            }
        }
        _ => panic!("Expected list"),
    }
}

#[test]
fn test_walrus_with_expression() {
    let expr = parse_expr("total := x + y").unwrap();
    match expr.kind {
        ExpressionKind::NamedExpr { target, value } => {
            match target.kind {
                ExpressionKind::Identifier(ref name) => assert_eq!(name, "total"),
                _ => panic!("Expected identifier"),
            }
            match value.kind {
                ExpressionKind::BinaryOp { .. } => {},
                _ => panic!("Expected binary operation"),
            }
        }
        _ => panic!("Expected named expression"),
    }
}

#[test]
fn test_walrus_nested() {
    let expr = parse_expr("(a := (b := 5))").unwrap();
    match expr.kind {
        ExpressionKind::NamedExpr { target, value } => {
            match target.kind {
                ExpressionKind::Identifier(ref name) => assert_eq!(name, "a"),
                _ => panic!("Expected identifier"),
            }
            match value.kind {
                ExpressionKind::NamedExpr { .. } => {},
                _ => panic!("Expected nested named expression"),
            }
        }
        _ => panic!("Expected named expression"),
    }
}

#[test]
fn test_walrus_with_comparison() {
    let expr = parse_expr("(n := len(data)) > 10").unwrap();
    match expr.kind {
        ExpressionKind::Compare { left, .. } => {
            match left.kind {
                ExpressionKind::NamedExpr { .. } => {},
                _ => panic!("Expected named expression in comparison"),
            }
        }
        _ => panic!("Expected comparison"),
    }
}

#[test]
fn test_walrus_in_function_call() {
    let expr = parse_expr("print(result := calculate())").unwrap();
    match expr.kind {
        ExpressionKind::Call { args, .. } => {
            assert_eq!(args.len(), 1);
            match &args[0].kind {
                ExpressionKind::NamedExpr { .. } => {},
                _ => panic!("Expected named expression in function call"),
            }
        }
        _ => panic!("Expected function call"),
    }
}

// ============================================================================
// F-String Tests
// ============================================================================

#[test]
fn test_fstring_basic() {
    let expr = parse_expr(r#"f"Hello {name}""#).unwrap();
    match expr.kind {
        ExpressionKind::FString { parts } => {
            assert_eq!(parts.len(), 2);
        }
        _ => panic!("Expected f-string, got {:?}", expr.kind),
    }
}

#[test]
fn test_fstring_multiple_expressions() {
    let expr = parse_expr(r#"f"{x} + {y} = {x + y}""#).unwrap();
    match expr.kind {
        ExpressionKind::FString { parts } => {
            assert_eq!(parts.len(), 5); // x, " + ", y, " = ", x+y
        }
        _ => panic!("Expected f-string"),
    }
}

#[test]
fn test_fstring_with_format_spec() {
    let expr = parse_expr(r#"f"{value:.2f}""#).unwrap();
    match expr.kind {
        ExpressionKind::FString { parts } => {
            assert_eq!(parts.len(), 1);
        }
        _ => panic!("Expected f-string"),
    }
}

#[test]
fn test_fstring_in_assignment() {
    let stmt = parse_stmt(r#"message = f"Hello {name}""#).unwrap();
    match stmt.kind {
        StatementKind::Assign { value, .. } => {
            match value.kind {
                ExpressionKind::FString { .. } => {},
                _ => panic!("Expected f-string in assignment"),
            }
        }
        _ => panic!("Expected assignment"),
    }
}

#[test]
fn test_fstring_in_function_call() {
    let expr = parse_expr(r#"print(f"Value: {x}")"#).unwrap();
    match expr.kind {
        ExpressionKind::Call { args, .. } => {
            assert_eq!(args.len(), 1);
            match &args[0].kind {
                ExpressionKind::FString { .. } => {},
                _ => panic!("Expected f-string in function call"),
            }
        }
        _ => panic!("Expected function call"),
    }
}

#[test]
fn test_fstring_only_text() {
    let expr = parse_expr(r#"f"Just text, no expressions""#).unwrap();
    match expr.kind {
        ExpressionKind::FString { parts } => {
            assert_eq!(parts.len(), 1);
        }
        _ => panic!("Expected f-string"),
    }
}

#[test]
fn test_fstring_complex_expression() {
    let expr = parse_expr(r#"f"Result: {func(a, b) * 2}""#).unwrap();
    match expr.kind {
        ExpressionKind::FString { parts } => {
            assert_eq!(parts.len(), 2);
        }
        _ => panic!("Expected f-string"),
    }
}

#[test]
fn test_fstring_in_list() {
    let expr = parse_expr(r#"[f"Item {i}", f"Value {v}"]"#).unwrap();
    match expr.kind {
        ExpressionKind::List { elements } => {
            assert_eq!(elements.len(), 2);
            match &elements[0].kind {
                ExpressionKind::FString { .. } => {},
                _ => panic!("Expected f-string in list"),
            }
            match &elements[1].kind {
                ExpressionKind::FString { .. } => {},
                _ => panic!("Expected f-string in list"),
            }
        }
        _ => panic!("Expected list"),
    }
}

// ============================================================================
// Raw String Tests
// ============================================================================

#[test]
fn test_raw_string_basic() {
    let expr = parse_expr(r#"r"Hello\nWorld""#).unwrap();
    match expr.kind {
        ExpressionKind::RawString(ref value) => {
            assert_eq!(value, r"Hello\nWorld");
        }
        _ => panic!("Expected raw string, got {:?}", expr.kind),
    }
}

#[test]
fn test_raw_string_backslashes() {
    let expr = parse_expr(r#"r"C:\Users\name\file.txt""#).unwrap();
    match expr.kind {
        ExpressionKind::RawString(ref value) => {
            assert_eq!(value, r"C:\Users\name\file.txt");
        }
        _ => panic!("Expected raw string"),
    }
}

#[test]
fn test_raw_string_regex() {
    let expr = parse_expr(r#"r"\d+\.\d+""#).unwrap();
    match expr.kind {
        ExpressionKind::RawString(ref value) => {
            assert_eq!(value, r"\d+\.\d+");
        }
        _ => panic!("Expected raw string"),
    }
}

#[test]
fn test_raw_string_in_assignment() {
    let stmt = parse_stmt(r#"pattern = r"\w+""#).unwrap();
    match stmt.kind {
        StatementKind::Assign { value, .. } => {
            match value.kind {
                ExpressionKind::RawString(ref s) => assert_eq!(s, r"\w+"),
                _ => panic!("Expected raw string in assignment"),
            }
        }
        _ => panic!("Expected assignment"),
    }
}

#[test]
fn test_raw_string_in_function_call() {
    let expr = parse_expr(r#"compile(r"[a-z]+")"#).unwrap();
    match expr.kind {
        ExpressionKind::Call { args, .. } => {
            assert_eq!(args.len(), 1);
            match &args[0].kind {
                ExpressionKind::RawString(ref s) => assert_eq!(s, r"[a-z]+"),
                _ => panic!("Expected raw string in function call"),
            }
        }
        _ => panic!("Expected function call"),
    }
}

#[test]
fn test_raw_string_in_list() {
    let expr = parse_expr(r#"[r"\n", r"\t", r"\r"]"#).unwrap();
    match expr.kind {
        ExpressionKind::List { elements } => {
            assert_eq!(elements.len(), 3);
            for elem in &elements {
                match &elem.kind {
                    ExpressionKind::RawString(_) => {},
                    _ => panic!("Expected raw string in list"),
                }
            }
        }
        _ => panic!("Expected list"),
    }
}

#[test]
fn test_raw_vs_regular_string_parser() {
    let stmts = parse_program(r#"r"\n"
"\n""#).unwrap();
    
    assert_eq!(stmts.len(), 2);
    
    // First should be raw string
    match &stmts[0].kind {
        StatementKind::Expr(expr) => {
            match &expr.kind {
                ExpressionKind::RawString(s) => assert_eq!(s, r"\n"),
                _ => panic!("Expected raw string"),
            }
        }
        _ => panic!("Expected expression statement"),
    }
    
    // Second should be regular string
    match &stmts[1].kind {
        StatementKind::Expr(expr) => {
            match &expr.kind {
                ExpressionKind::String(s) => assert_eq!(s, "\n"),
                _ => panic!("Expected regular string"),
            }
        }
        _ => panic!("Expected expression statement"),
    }
}

// ============================================================================
// Byte String Tests
// ============================================================================

#[test]
fn test_byte_string_basic() {
    let expr = parse_expr(r#"b"Hello""#).unwrap();
    match expr.kind {
        ExpressionKind::ByteString(ref bytes) => {
            assert_eq!(bytes, b"Hello");
        }
        _ => panic!("Expected byte string, got {:?}", expr.kind),
    }
}

#[test]
fn test_byte_string_with_escapes() {
    let expr = parse_expr(r#"b"Line1\nLine2""#).unwrap();
    match expr.kind {
        ExpressionKind::ByteString(ref bytes) => {
            assert_eq!(bytes, b"Line1\nLine2");
        }
        _ => panic!("Expected byte string"),
    }
}

#[test]
fn test_byte_string_hex_escape() {
    let expr = parse_expr(r#"b"\x48\x69""#).unwrap();
    match expr.kind {
        ExpressionKind::ByteString(ref bytes) => {
            assert_eq!(bytes, b"Hi");
        }
        _ => panic!("Expected byte string"),
    }
}

#[test]
fn test_byte_string_in_assignment() {
    let stmt = parse_stmt(r#"data = b"binary""#).unwrap();
    match stmt.kind {
        StatementKind::Assign { value, .. } => {
            match value.kind {
                ExpressionKind::ByteString(ref bytes) => assert_eq!(bytes, b"binary"),
                _ => panic!("Expected byte string in assignment"),
            }
        }
        _ => panic!("Expected assignment"),
    }
}

#[test]
fn test_byte_string_in_function_call() {
    let expr = parse_expr(r#"write(b"data")"#).unwrap();
    match expr.kind {
        ExpressionKind::Call { args, .. } => {
            assert_eq!(args.len(), 1);
            match &args[0].kind {
                ExpressionKind::ByteString(ref bytes) => assert_eq!(bytes, b"data"),
                _ => panic!("Expected byte string in function call"),
            }
        }
        _ => panic!("Expected function call"),
    }
}

#[test]
fn test_byte_string_in_list() {
    let expr = parse_expr(r#"[b"a", b"b", b"c"]"#).unwrap();
    match expr.kind {
        ExpressionKind::List { elements } => {
            assert_eq!(elements.len(), 3);
            for elem in &elements {
                match &elem.kind {
                    ExpressionKind::ByteString(_) => {},
                    _ => panic!("Expected byte string in list"),
                }
            }
        }
        _ => panic!("Expected list"),
    }
}

#[test]
fn test_byte_string_empty() {
    let expr = parse_expr(r#"b"""#).unwrap();
    match expr.kind {
        ExpressionKind::ByteString(ref bytes) => {
            assert_eq!(bytes.len(), 0);
        }
        _ => panic!("Expected byte string"),
    }
}

// ============================================================================
// Byte Raw String Tests (br"..." or rb"...")
// ============================================================================

#[test]
fn test_byte_raw_string_basic_br() {
    let expr = parse_expr(r#"br"Hello\nWorld""#).unwrap();
    match expr.kind {
        ExpressionKind::ByteRawString(ref bytes) => {
            // Raw string preserves \n literally (backslash and 'n')
            assert_eq!(bytes, b"Hello\\nWorld");
        }
        _ => panic!("Expected byte raw string"),
    }
}

#[test]
fn test_byte_raw_string_basic_rb() {
    let expr = parse_expr(r#"rb"Hello\tWorld""#).unwrap();
    match expr.kind {
        ExpressionKind::ByteRawString(ref bytes) => {
            // Raw string preserves \t literally
            assert_eq!(bytes, b"Hello\\tWorld");
        }
        _ => panic!("Expected byte raw string"),
    }
}

#[test]
fn test_byte_raw_string_windows_path() {
    let expr = parse_expr(r#"br"C:\Users\username\file.txt""#).unwrap();
    match expr.kind {
        ExpressionKind::ByteRawString(ref bytes) => {
            // All backslashes preserved
            assert_eq!(bytes, b"C:\\Users\\username\\file.txt");
        }
        _ => panic!("Expected byte raw string"),
    }
}

#[test]
fn test_byte_raw_string_regex_pattern() {
    let expr = parse_expr(r#"br"\d+\.\d+""#).unwrap();
    match expr.kind {
        ExpressionKind::ByteRawString(ref bytes) => {
            // Regex escapes preserved
            assert_eq!(bytes, b"\\d+\\.\\d+");
        }
        _ => panic!("Expected byte raw string"),
    }
}

#[test]
fn test_byte_raw_string_in_assignment() {
    let stmt = parse_stmt(r#"pattern = br"\w+@\w+\.\w+""#).unwrap();
    match stmt.kind {
        StatementKind::Assign { ref targets, ref value, .. } => {
            assert_eq!(targets.len(), 1);
            match value.kind {
                ExpressionKind::ByteRawString(ref bytes) => {
                    assert_eq!(bytes, b"\\w+@\\w+\\.\\w+");
                }
                _ => panic!("Expected byte raw string in assignment"),
            }
        }
        _ => panic!("Expected assignment"),
    }
}

#[test]
fn test_byte_raw_string_in_function_call() {
    let expr = parse_expr(r#"compile(br"[a-z]+\d+")"#).unwrap();
    match expr.kind {
        ExpressionKind::Call { ref args, .. } => {
            assert_eq!(args.len(), 1);
            match args[0].kind {
                ExpressionKind::ByteRawString(ref bytes) => {
                    assert_eq!(bytes, b"[a-z]+\\d+");
                }
                _ => panic!("Expected byte raw string argument"),
            }
        }
        _ => panic!("Expected function call"),
    }
}

#[test]
fn test_byte_raw_string_in_list() {
    let expr = parse_expr(r#"[br"path1\n", rb"path2\t", br"path3\r"]"#).unwrap();
    match expr.kind {
        ExpressionKind::List { ref elements } => {
            assert_eq!(elements.len(), 3);
            
            // First element: br"path1\n"
            match elements[0].kind {
                ExpressionKind::ByteRawString(ref bytes) => {
                    assert_eq!(bytes, b"path1\\n");
                }
                _ => panic!("Expected byte raw string"),
            }
            
            // Second element: rb"path2\t"
            match elements[1].kind {
                ExpressionKind::ByteRawString(ref bytes) => {
                    assert_eq!(bytes, b"path2\\t");
                }
                _ => panic!("Expected byte raw string"),
            }
        }
        _ => panic!("Expected list"),
    }
}

#[test]
fn test_byte_raw_string_empty() {
    let expr = parse_expr(r#"br"""#).unwrap();
    match expr.kind {
        ExpressionKind::ByteRawString(ref bytes) => {
            assert_eq!(bytes.len(), 0);
        }
        _ => panic!("Expected byte raw string"),
    }
}

#[test]
fn test_byte_raw_string_uppercase_variants() {
    // Test BR
    let expr = parse_expr(r#"BR"Test\n""#).unwrap();
    match expr.kind {
        ExpressionKind::ByteRawString(ref bytes) => {
            assert_eq!(bytes, b"Test\\n");
        }
        _ => panic!("Expected byte raw string"),
    }
    
    // Test RB
    let expr = parse_expr(r#"RB"Test\t""#).unwrap();
    match expr.kind {
        ExpressionKind::ByteRawString(ref bytes) => {
            assert_eq!(bytes, b"Test\\t");
        }
        _ => panic!("Expected byte raw string"),
    }
}

// ============================================================================
// Ellipsis Literal Tests (...)
// ============================================================================

#[test]
fn test_ellipsis_literal() {
    let expr = parse_expr("...").unwrap();
    match expr.kind {
        ExpressionKind::Ellipsis => {},
        _ => panic!("Expected ellipsis literal"),
    }
}

#[test]
fn test_ellipsis_in_assignment() {
    let stmt = parse_stmt("x = ...").unwrap();
    match stmt.kind {
        StatementKind::Assign { ref value, .. } => {
            match value.kind {
                ExpressionKind::Ellipsis => {},
                _ => panic!("Expected ellipsis in assignment"),
            }
        }
        _ => panic!("Expected assignment"),
    }
}

#[test]
fn test_ellipsis_in_function_body() {
    let stmt = parse_stmt("def foo():\n    ...").unwrap();
    match stmt.kind {
        StatementKind::FunctionDef { ref body, .. } => {
            assert_eq!(body.len(), 1);
            match body[0].kind {
                StatementKind::Expr(ref expr) => {
                    match expr.kind {
                        ExpressionKind::Ellipsis => {},
                        _ => panic!("Expected ellipsis in function body"),
                    }
                }
                _ => panic!("Expected expression statement"),
            }
        }
        _ => panic!("Expected function definition"),
    }
}

#[test]
fn test_ellipsis_in_list() {
    let expr = parse_expr("[1, 2, ..., 5]").unwrap();
    match expr.kind {
        ExpressionKind::List { ref elements } => {
            assert_eq!(elements.len(), 4);
            match elements[2].kind {
                ExpressionKind::Ellipsis => {},
                _ => panic!("Expected ellipsis in list"),
            }
        }
        _ => panic!("Expected list"),
    }
}

#[test]
fn test_ellipsis_in_tuple() {
    let expr = parse_expr("(1, ..., 3)").unwrap();
    match expr.kind {
        ExpressionKind::Tuple { ref elements } => {
            assert_eq!(elements.len(), 3);
            match elements[1].kind {
                ExpressionKind::Ellipsis => {},
                _ => panic!("Expected ellipsis in tuple"),
            }
        }
        _ => panic!("Expected tuple"),
    }
}

#[test]
fn test_ellipsis_as_function_argument() {
    let expr = parse_expr("func(...)").unwrap();
    match expr.kind {
        ExpressionKind::Call { ref args, .. } => {
            assert_eq!(args.len(), 1);
            match args[0].kind {
                ExpressionKind::Ellipsis => {},
                _ => panic!("Expected ellipsis as argument"),
            }
        }
        _ => panic!("Expected function call"),
    }
}

#[test]
fn test_ellipsis_in_return() {
    let stmt = parse_stmt("return ...").unwrap();
    match stmt.kind {
        StatementKind::Return { ref value } => {
            match value {
                Some(expr) => {
                    match expr.kind {
                        ExpressionKind::Ellipsis => {},
                        _ => panic!("Expected ellipsis in return"),
                    }
                }
                None => panic!("Expected return value"),
            }
        }
        _ => panic!("Expected return statement"),
    }
}

// ============================================================================
// NotImplemented Singleton Tests
// ============================================================================

#[test]
fn test_notimplemented_literal() {
    let expr = parse_expr("NotImplemented").unwrap();
    match expr.kind {
        ExpressionKind::NotImplemented => {},
        _ => panic!("Expected NotImplemented literal"),
    }
}

#[test]
fn test_notimplemented_in_assignment() {
    let stmt = parse_stmt("result = NotImplemented").unwrap();
    match stmt.kind {
        StatementKind::Assign { ref value, .. } => {
            match value.kind {
                ExpressionKind::NotImplemented => {},
                _ => panic!("Expected NotImplemented in assignment"),
            }
        }
        _ => panic!("Expected assignment"),
    }
}

#[test]
fn test_notimplemented_in_return() {
    let stmt = parse_stmt("return NotImplemented").unwrap();
    match stmt.kind {
        StatementKind::Return { ref value } => {
            match value {
                Some(expr) => {
                    match expr.kind {
                        ExpressionKind::NotImplemented => {},
                        _ => panic!("Expected NotImplemented in return"),
                    }
                }
                None => panic!("Expected return value"),
            }
        }
        _ => panic!("Expected return statement"),
    }
}

#[test]
fn test_notimplemented_in_comparison() {
    let expr = parse_expr("x == NotImplemented").unwrap();
    match expr.kind {
        ExpressionKind::Compare { ref comparators, .. } => {
            assert_eq!(comparators.len(), 1);
            match comparators[0].kind {
                ExpressionKind::NotImplemented => {},
                _ => panic!("Expected NotImplemented in comparison"),
            }
        }
        _ => panic!("Expected comparison"),
    }
}

#[test]
fn test_notimplemented_in_list() {
    let expr = parse_expr("[1, NotImplemented, 3]").unwrap();
    match expr.kind {
        ExpressionKind::List { ref elements } => {
            assert_eq!(elements.len(), 3);
            match elements[1].kind {
                ExpressionKind::NotImplemented => {},
                _ => panic!("Expected NotImplemented in list"),
            }
        }
        _ => panic!("Expected list"),
    }
}

#[test]
fn test_notimplemented_in_function_call() {
    let expr = parse_expr("process(NotImplemented)").unwrap();
    match expr.kind {
        ExpressionKind::Call { ref args, .. } => {
            assert_eq!(args.len(), 1);
            match args[0].kind {
                ExpressionKind::NotImplemented => {},
                _ => panic!("Expected NotImplemented as argument"),
            }
        }
        _ => panic!("Expected function call"),
    }
}

#[test]
fn test_notimplemented_in_dict_value() {
    let expr = parse_expr("{'key': NotImplemented}").unwrap();
    match expr.kind {
        ExpressionKind::Dict { ref values, .. } => {
            assert_eq!(values.len(), 1);
            match values[0].kind {
                ExpressionKind::NotImplemented => {},
                _ => panic!("Expected NotImplemented as dict value"),
            }
        }
        _ => panic!("Expected dict"),
    }
}

#[test]
fn test_notimplemented_in_tuple() {
    let expr = parse_expr("(NotImplemented, None, True)").unwrap();
    match expr.kind {
        ExpressionKind::Tuple { ref elements } => {
            assert_eq!(elements.len(), 3);
            match elements[0].kind {
                ExpressionKind::NotImplemented => {},
                _ => panic!("Expected NotImplemented in tuple"),
            }
        }
        _ => panic!("Expected tuple"),
    }
}

#[test]
fn test_notimplemented_in_conditional() {
    let expr = parse_expr("NotImplemented if condition else value").unwrap();
    match expr.kind {
        ExpressionKind::IfExp { ref body, .. } => {
            match body.kind {
                ExpressionKind::NotImplemented => {},
                _ => panic!("Expected NotImplemented in conditional body"),
            }
        }
        _ => panic!("Expected conditional expression"),
    }
}

// ============================================================================
// Comprehension Tests - Step by Step Implementation
// ============================================================================

#[test]
fn test_list_comp_detection() {
    // Test 1: Regular list should still work
    let expr = parse_expr("[x]").unwrap();
    match expr.kind {
        ExpressionKind::List { ref elements } => {
            assert_eq!(elements.len(), 1);
        }
        _ => panic!("Expected regular list for [x]"),
    }
    
    // Test 2: List comprehension should now parse successfully!
    let result = parse_expr("[x for x in items]");
    match result {
        Ok(expr) => {
            match expr.kind {
                ExpressionKind::ListComp { .. } => {
                    // Success! Comprehension detected and parsed
                }
                _ => panic!("Expected list comprehension"),
            }
        }
        Err(e) => panic!("Comprehension should parse successfully now, got error: {:?}", e),
    }
}

#[test]
fn test_list_comp_simplest() {
    // Simplest possible: [x for x in items]
    // First verify parsing doesn't hang - add timeout expectation
    let source = "[x for x in items]";
    println!("Parsing: {}", source);
    let expr = parse_expr(source).unwrap();
    match expr.kind {
        ExpressionKind::ListComp { ref element, ref generators } => {
            // Check element is 'x'
            match element.kind {
                ExpressionKind::Identifier(ref name) => assert_eq!(name, "x"),
                _ => panic!("Expected identifier 'x' as element"),
            }
            
            // Check we have exactly one generator
            assert_eq!(generators.len(), 1);
            
            let gen = &generators[0];
            
            // Check target is 'x'
            match &gen.target.kind {
                silk_ast::PatternKind::Name(name) => assert_eq!(name, "x"),
                _ => panic!("Expected name pattern for target"),
            }
            
            // Check iterator is 'items'
            match gen.iter.kind {
                ExpressionKind::Identifier(ref name) => assert_eq!(name, "items"),
                _ => panic!("Expected identifier 'items' as iterator"),
            }
            
            // No filters
            assert_eq!(gen.ifs.len(), 0);
            assert_eq!(gen.is_async, false);
        }
        _ => panic!("Expected list comprehension, got: {:?}", expr.kind),
    }
}

#[test]
fn test_list_comp_single_filter() {
    let source = "[x for x in items if x > 0]";
    println!("Parsing: {}", source);
    
    let expr = parse_expr(source).unwrap();
    
    match expr.kind {
        ExpressionKind::ListComp { ref element, ref generators } => {
            // Check element is 'x'
            match element.kind {
                ExpressionKind::Identifier(ref name) => assert_eq!(name, "x"),
                _ => panic!("Expected identifier 'x' as element"),
            }
            
            assert_eq!(generators.len(), 1);
            let gen = &generators[0];
            
            // Check target pattern
            match &gen.target.kind {
                silk_ast::PatternKind::Name(name) => assert_eq!(name, "x"),
                _ => panic!("Expected name pattern"),
            }
            
            // Check iterator
            match gen.iter.kind {
                ExpressionKind::Identifier(ref name) => assert_eq!(name, "items"),
                _ => panic!("Expected identifier 'items'"),
            }
            
            // Check single filter: x > 0
            assert_eq!(gen.ifs.len(), 1);
            match &gen.ifs[0].kind {
                ExpressionKind::Compare { left, ops, comparators } => {
                    match left.kind {
                        ExpressionKind::Identifier(ref name) => assert_eq!(name, "x"),
                        _ => panic!("Expected 'x' on left"),
                    }
                    assert_eq!(ops.len(), 1);
                    assert_eq!(ops[0], silk_ast::CompareOperator::Gt);
                    assert_eq!(comparators.len(), 1);
                    match comparators[0].kind {
                        ExpressionKind::Integer(val) => assert_eq!(val, 0),
                        _ => panic!("Expected 0 on right"),
                    }
                }
                _ => panic!("Expected compare op for filter"),
            }
            
            assert_eq!(gen.is_async, false);
        }
        _ => panic!("Expected list comprehension, got: {:?}", expr.kind),
    }
}

#[test]
fn test_list_comp_multiple_filters() {
    let source = "[x for x in items if x > 0 if x < 10]";
    let expr = parse_expr(source).unwrap();
    
    match expr.kind {
        ExpressionKind::ListComp { element: _, ref generators } => {
            assert_eq!(generators.len(), 1);
            let gen = &generators[0];
            
            // Check two filters
            assert_eq!(gen.ifs.len(), 2);
            
            // First filter: x > 0
            match &gen.ifs[0].kind {
                ExpressionKind::Compare { left, ops, comparators } => {
                    match left.kind {
                        ExpressionKind::Identifier(ref name) => assert_eq!(name, "x"),
                        _ => panic!("Expected 'x'"),
                    }
                    assert_eq!(ops[0], silk_ast::CompareOperator::Gt);
                    match comparators[0].kind {
                        ExpressionKind::Integer(0) => {},
                        _ => panic!("Expected 0"),
                    }
                }
                _ => panic!("Expected compare"),
            }
            
            // Second filter: x < 10
            match &gen.ifs[1].kind {
                ExpressionKind::Compare { left, ops, comparators } => {
                    match left.kind {
                        ExpressionKind::Identifier(ref name) => assert_eq!(name, "x"),
                        _ => panic!("Expected 'x'"),
                    }
                    assert_eq!(ops[0], silk_ast::CompareOperator::Lt);
                    match comparators[0].kind {
                        ExpressionKind::Integer(10) => {},
                        _ => panic!("Expected 10"),
                    }
                }
                _ => panic!("Expected compare"),
            }
        }
        _ => panic!("Expected list comprehension"),
    }
}

#[test]
fn test_list_comp_nested_simple() {
    let source = "[x + y for x in range(3) for y in range(3)]";
    let expr = parse_expr(source).unwrap();
    
    match expr.kind {
        ExpressionKind::ListComp { ref element, ref generators } => {
            // Check element is 'x + y'
            match &element.kind {
                ExpressionKind::BinaryOp { left, op, right } => {
                    assert!(matches!(left.kind, ExpressionKind::Identifier(ref name) if name == "x"));
                    assert_eq!(*op, silk_ast::BinaryOperator::Add);
                    assert!(matches!(right.kind, ExpressionKind::Identifier(ref name) if name == "y"));
                }
                _ => panic!("Expected binary op for element"),
            }
            
            // Check two generators
            assert_eq!(generators.len(), 2);
            
            // First: for x in range(3)
            let gen1 = &generators[0];
            assert!(matches!(gen1.target.kind, silk_ast::PatternKind::Name(ref name) if name == "x"));
            assert!(matches!(gen1.iter.kind, ExpressionKind::Call { .. }));
            assert_eq!(gen1.ifs.len(), 0);
            
            // Second: for y in range(3)
            let gen2 = &generators[1];
            assert!(matches!(gen2.target.kind, silk_ast::PatternKind::Name(ref name) if name == "y"));
            assert!(matches!(gen2.iter.kind, ExpressionKind::Call { .. }));
            assert_eq!(gen2.ifs.len(), 0);
        }
        _ => panic!("Expected list comprehension"),
    }
}

#[test]
fn test_list_comp_nested_with_filter() {
    let source = "[x for x in range(10) for y in range(10) if x == y]";
    let expr = parse_expr(source).unwrap();
    
    match expr.kind {
        ExpressionKind::ListComp { element: _, ref generators } => {
            assert_eq!(generators.len(), 2);
            
            // First generator: for x in range(10) - no filter
            assert_eq!(generators[0].ifs.len(), 0);
            
            // Second generator: for y in range(10) if x == y - one filter
            assert_eq!(generators[1].ifs.len(), 1);
            
            // Check the filter is x == y
            match &generators[1].ifs[0].kind {
                ExpressionKind::Compare { left, ops, comparators } => {
                    assert!(matches!(left.kind, ExpressionKind::Identifier(ref name) if name == "x"));
                    assert_eq!(ops[0], silk_ast::CompareOperator::Eq);
                    assert!(matches!(comparators[0].kind, ExpressionKind::Identifier(ref name) if name == "y"));
                }
                _ => panic!("Expected compare for filter"),
            }
        }
        _ => panic!("Expected list comprehension"),
    }
}

#[test]
fn test_dict_comp_simple() {
    let source = "{x: x * 2 for x in items}";
    let expr = parse_expr(source).unwrap();
    
    match expr.kind {
        ExpressionKind::DictComp { ref key, ref value, ref generators } => {
            // Check key is 'x'
            assert!(matches!(key.kind, ExpressionKind::Identifier(ref name) if name == "x"));
            
            // Check value is 'x * 2'
            assert!(matches!(value.kind, ExpressionKind::BinaryOp { .. }));
            
            // Check one generator
            assert_eq!(generators.len(), 1);
            assert!(matches!(generators[0].target.kind, silk_ast::PatternKind::Name(ref name) if name == "x"));
        }
        _ => panic!("Expected dict comprehension"),
    }
}

#[test]
fn test_set_comp_simple() {
    let source = "{x * 2 for x in items}";
    let expr = parse_expr(source).unwrap();
    
    match expr.kind {
        ExpressionKind::SetComp { ref element, ref generators } => {
            // Check element is 'x * 2'
            assert!(matches!(element.kind, ExpressionKind::BinaryOp { .. }));
            
            // Check one generator
            assert_eq!(generators.len(), 1);
            assert!(matches!(generators[0].target.kind, silk_ast::PatternKind::Name(ref name) if name == "x"));
        }
        _ => panic!("Expected set comprehension"),
    }
}

#[test]
fn test_dict_comp_with_filter() {
    let source = "{x: x * 2 for x in items if x > 0}";
    let expr = parse_expr(source).unwrap();
    
    match expr.kind {
        ExpressionKind::DictComp { key: _, value: _, ref generators } => {
            assert_eq!(generators.len(), 1);
            
            // Check simple name target
            assert!(matches!(generators[0].target.kind, silk_ast::PatternKind::Name(ref name) if name == "x"));
            
            // Check one filter
            assert_eq!(generators[0].ifs.len(), 1);
        }
        _ => panic!("Expected dict comprehension"),
    }
}

#[test]
fn test_generator_exp_simple() {
    let source = "(x for x in items)";
    let expr = parse_expr(source).unwrap();
    
    match expr.kind {
        ExpressionKind::GeneratorExp { ref element, ref generators } => {
            // Check element is 'x'
            assert!(matches!(element.kind, ExpressionKind::Identifier(ref name) if name == "x"));
            
            // Check one generator
            assert_eq!(generators.len(), 1);
            assert!(matches!(generators[0].target.kind, silk_ast::PatternKind::Name(ref name) if name == "x"));
        }
        _ => panic!("Expected generator expression"),
    }
}

#[test]
fn test_generator_exp_with_filter() {
    let source = "(x for x in items if x > 0)";
    let expr = parse_expr(source).unwrap();
    
    match expr.kind {
        ExpressionKind::GeneratorExp { element: _, ref generators } => {
            assert_eq!(generators.len(), 1);
            assert_eq!(generators[0].ifs.len(), 1);
        }
        _ => panic!("Expected generator expression"),
    }
}

#[test]
fn test_generator_exp_in_function_call() {
    let source = "sum(x * x for x in range(100))";
    let expr = parse_expr(source).unwrap();
    
    match expr.kind {
        ExpressionKind::Call { ref func, ref args, keywords: _ } => {
            // Check function name is 'sum'
            assert!(matches!(func.kind, ExpressionKind::Identifier(ref name) if name == "sum"));
            
            // Check one argument
            assert_eq!(args.len(), 1);
            
            // Check argument is generator expression
            match &args[0].kind {
                ExpressionKind::GeneratorExp { element, generators } => {
                    // Check element is x * x
                    assert!(matches!(element.kind, ExpressionKind::BinaryOp { .. }));
                    
                    // Check one generator
                    assert_eq!(generators.len(), 1);
                    assert!(matches!(generators[0].target.kind, silk_ast::PatternKind::Name(ref name) if name == "x"));
                }
                _ => panic!("Expected generator expression as argument"),
            }
        }
        _ => panic!("Expected function call"),
    }
}

// Step 10: Edge Cases & Polish

#[test]
fn test_comp_empty_sequence() {
    let source = "[x for x in []]";
    let expr = parse_expr(source).unwrap();
    
    match expr.kind {
        ExpressionKind::ListComp { element: _, generators } => {
            assert_eq!(generators.len(), 1);
            // Iterator is empty list
            assert!(matches!(generators[0].iter.kind, ExpressionKind::List { .. }));
        }
        _ => panic!("Expected list comprehension"),
    }
}

#[test]
fn test_comp_nested_comprehension() {
    let source = "[[y for y in row] for row in matrix]";
    let expr = parse_expr(source).unwrap();
    
    match expr.kind {
        ExpressionKind::ListComp { ref element, ref generators } => {
            // Outer comprehension has one generator
            assert_eq!(generators.len(), 1);
            
            // Element is itself a list comprehension
            match &element.kind {
                ExpressionKind::ListComp { element: inner_elem, generators: inner_gens } => {
                    // Inner comprehension: [y for y in row]
                    assert!(matches!(inner_elem.kind, ExpressionKind::Identifier(ref name) if name == "y"));
                    assert_eq!(inner_gens.len(), 1);
                    assert!(matches!(inner_gens[0].target.kind, silk_ast::PatternKind::Name(ref name) if name == "y"));
                }
                _ => panic!("Expected inner list comprehension"),
            }
        }
        _ => panic!("Expected outer list comprehension"),
    }
}

#[test]
fn test_comp_in_function_call() {
    let source = "func([x for x in items])";
    let expr = parse_expr(source).unwrap();
    
    match expr.kind {
        ExpressionKind::Call { func: _, ref args, keywords: _ } => {
            assert_eq!(args.len(), 1);
            assert!(matches!(args[0].kind, ExpressionKind::ListComp { .. }));
        }
        _ => panic!("Expected function call"),
    }
}

#[test]
fn test_comp_complex_filter() {
    let source = "[x for x in items if x > 0 if x < 10]";
    let expr = parse_expr(source).unwrap();
    
    match expr.kind {
        ExpressionKind::ListComp { element: _, ref generators } => {
            assert_eq!(generators.len(), 1);
            // Two filters
            assert_eq!(generators[0].ifs.len(), 2);
        }
        _ => panic!("Expected list comprehension"),
    }
}

#[test]
fn test_comp_with_call_in_iterator() {
    let source = "[x for x in range(10)]";
    let expr = parse_expr(source).unwrap();
    
    match expr.kind {
        ExpressionKind::ListComp { element: _, ref generators } => {
            assert_eq!(generators.len(), 1);
            // Iterator is a function call
            assert!(matches!(generators[0].iter.kind, ExpressionKind::Call { .. }));
        }
        _ => panic!("Expected list comprehension"),
    }
}

#[test]
fn test_comp_with_attribute_access() {
    let source = "[obj.name for obj in objects]";
    let expr = parse_expr(source).unwrap();
    
    match expr.kind {
        ExpressionKind::ListComp { ref element, ref generators } => {
            // Element is attribute access
            assert!(matches!(element.kind, ExpressionKind::Attribute { .. }));
            assert_eq!(generators.len(), 1);
        }
        _ => panic!("Expected list comprehension"),
    }
}


