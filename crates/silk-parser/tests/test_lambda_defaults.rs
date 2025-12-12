/// Tests for lambda expressions with default parameter values
use silk_ast::ExpressionKind;
use silk_parser::{ParseError, Parser};

// Helper function to parse an expression from a single-expression source
fn parse_expr(source: &str) -> Result<silk_ast::Expression, ParseError> {
    let program = Parser::parse(source)?;
    assert_eq!(
        program.statements.len(),
        1,
        "Expected exactly one statement"
    );

    match &program.statements[0].kind {
        silk_ast::StatementKind::Expr(expr) => Ok(expr.clone()),
        _ => panic!(
            "Expected expression statement, got {:?}",
            program.statements[0].kind
        ),
    }
}

#[test]
fn test_lambda_single_default_parameter() {
    let source = "lambda x=10: x * 2";
    let expr = parse_expr(source).unwrap();

    match expr.kind {
        ExpressionKind::Lambda { params, body } => {
            assert_eq!(params.len(), 1);
            assert_eq!(params[0].name, "x");
            assert!(params[0].default.is_some());
            
            // Check default value is 10
            if let Some(default) = &params[0].default {
                if let ExpressionKind::Integer(val) = default.kind {
                    assert_eq!(val, 10);
                } else {
                    panic!("Expected integer default value");
                }
            }

            // Check body is x * 2
            if let ExpressionKind::BinaryOp { .. } = body.kind {
                // Body is correct
            } else {
                panic!("Expected binary operation in lambda body");
            }
        }
        _ => panic!("Expected lambda expression"),
    }
}

#[test]
fn test_lambda_multiple_parameters_mixed_defaults() {
    let source = "lambda x, y=5: x + y";
    let expr = parse_expr(source).unwrap();

    match expr.kind {
        ExpressionKind::Lambda { params, .. } => {
            assert_eq!(params.len(), 2);
            
            // First param has no default
            assert_eq!(params[0].name, "x");
            assert!(params[0].default.is_none());
            
            // Second param has default of 5
            assert_eq!(params[1].name, "y");
            assert!(params[1].default.is_some());
            
            if let Some(default) = &params[1].default {
                if let ExpressionKind::Integer(val) = default.kind {
                    assert_eq!(val, 5);
                } else {
                    panic!("Expected integer default value");
                }
            }
        }
        _ => panic!("Expected lambda expression"),
    }
}

#[test]
fn test_lambda_all_parameters_with_defaults() {
    let source = "lambda x=1, y=2, z=3: x + y + z";
    let expr = parse_expr(source).unwrap();

    match expr.kind {
        ExpressionKind::Lambda { params, .. } => {
            assert_eq!(params.len(), 3);
            
            // All params should have defaults
            for (i, param) in params.iter().enumerate() {
                assert!(param.default.is_some(), "Param {} should have default", i);
                
                if let Some(default) = &param.default {
                    if let ExpressionKind::Integer(val) = default.kind {
                        assert_eq!(val, (i + 1) as i64);
                    } else {
                        panic!("Expected integer default value");
                    }
                }
            }
        }
        _ => panic!("Expected lambda expression"),
    }
}

#[test]
fn test_lambda_complex_default_expressions() {
    // Test with list literal as default
    let source = "lambda x=[1, 2, 3]: len(x)";
    let expr = parse_expr(source).unwrap();

    match expr.kind {
        ExpressionKind::Lambda { params, .. } => {
            assert_eq!(params.len(), 1);
            assert!(params[0].default.is_some());
            
            if let Some(default) = &params[0].default {
                if let ExpressionKind::List { elements } = &default.kind {
                    assert_eq!(elements.len(), 3);
                } else {
                    panic!("Expected list default value");
                }
            }
        }
        _ => panic!("Expected lambda expression"),
    }
}

#[test]
fn test_lambda_string_default() {
    let source = r#"lambda msg="hello": msg"#;
    let expr = parse_expr(source).unwrap();

    match expr.kind {
        ExpressionKind::Lambda { params, .. } => {
            assert_eq!(params.len(), 1);
            assert_eq!(params[0].name, "msg");
            assert!(params[0].default.is_some());
            
            if let Some(default) = &params[0].default {
                if let ExpressionKind::String(s) = &default.kind {
                    assert_eq!(s, "hello");
                } else {
                    panic!("Expected string default value");
                }
            }
        }
        _ => panic!("Expected lambda expression"),
    }
}

#[test]
fn test_lambda_boolean_and_none_defaults() {
    let source = "lambda flag=True, value=None: flag";
    let expr = parse_expr(source).unwrap();

    match expr.kind {
        ExpressionKind::Lambda { params, .. } => {
            assert_eq!(params.len(), 2);
            
            // First param: flag=True
            assert!(params[0].default.is_some());
            if let Some(default) = &params[0].default {
                if let ExpressionKind::Boolean(val) = default.kind {
                    assert!(val);
                } else {
                    panic!("Expected boolean default value");
                }
            }
            
            // Second param: value=None
            assert!(params[1].default.is_some());
            if let Some(default) = &params[1].default {
                if let ExpressionKind::None = default.kind {
                    // Correct
                } else {
                    panic!("Expected None default value");
                }
            }
        }
        _ => panic!("Expected lambda expression"),
    }
}

#[test]
fn test_lambda_expression_default() {
    // Default can be an expression
    let source = "lambda x=10+5: x";
    let expr = parse_expr(source).unwrap();

    match expr.kind {
        ExpressionKind::Lambda { params, .. } => {
            assert_eq!(params.len(), 1);
            assert!(params[0].default.is_some());
            
            if let Some(default) = &params[0].default {
                if let ExpressionKind::BinaryOp { .. } = default.kind {
                    // Correct - it's a binary operation
                } else {
                    panic!("Expected binary operation default value");
                }
            }
        }
        _ => panic!("Expected lambda expression"),
    }
}

#[test]
fn test_lambda_non_default_after_default_error() {
    // This should fail: non-default parameter after default parameter
    let source = "lambda x=10, y: x + y";
    let result = parse_expr(source);

    assert!(result.is_err(), "Should reject non-default param after default");
    
    if let Err(e) = result {
        let error_msg = format!("{}", e);
        assert!(
            error_msg.contains("Non-default parameter follows default parameter"),
            "Expected non-default after default error, got: {}",
            error_msg
        );
    }
}

#[test]
fn test_lambda_no_parameters_still_works() {
    // Lambda with no parameters should still work
    let source = "lambda: 42";
    let expr = parse_expr(source).unwrap();

    match expr.kind {
        ExpressionKind::Lambda { params, body } => {
            assert_eq!(params.len(), 0);
            
            if let ExpressionKind::Integer(val) = body.kind {
                assert_eq!(val, 42);
            } else {
                panic!("Expected integer in lambda body");
            }
        }
        _ => panic!("Expected lambda expression"),
    }
}

#[test]
fn test_lambda_default_with_trailing_comma() {
    // Python allows trailing comma in lambda params
    let source = "lambda x=10,: x";
    let expr = parse_expr(source).unwrap();

    match expr.kind {
        ExpressionKind::Lambda { params, .. } => {
            assert_eq!(params.len(), 1);
            assert_eq!(params[0].name, "x");
            assert!(params[0].default.is_some());
        }
        _ => panic!("Expected lambda expression"),
    }
}

#[test]
fn test_lambda_nested_lambda_as_default() {
    // Lambda as default value
    let source = "lambda f=lambda y: y*2: f(10)";
    let expr = parse_expr(source).unwrap();

    match expr.kind {
        ExpressionKind::Lambda { params, .. } => {
            assert_eq!(params.len(), 1);
            assert_eq!(params[0].name, "f");
            assert!(params[0].default.is_some());
            
            if let Some(default) = &params[0].default {
                if let ExpressionKind::Lambda { .. } = default.kind {
                    // Correct - nested lambda
                } else {
                    panic!("Expected lambda default value");
                }
            }
        }
        _ => panic!("Expected lambda expression"),
    }
}

