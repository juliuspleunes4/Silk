use silk_ast::{ExpressionKind, StatementKind, TypeKind};
/// Tests for annotated assignment (AnnAssign) parsing
use silk_parser::Parser;

#[test]
fn test_ann_assign_with_value() {
    let source = "x: int = 10";
    let program = Parser::parse(source).unwrap();

    assert_eq!(program.statements.len(), 1);

    match &program.statements[0].kind {
        StatementKind::AnnAssign {
            target,
            annotation,
            value,
        } => {
            // Check target is identifier 'x'
            match &target.kind {
                ExpressionKind::Identifier(name) => assert_eq!(name, "x"),
                _ => panic!("Expected Identifier expression"),
            }

            // Check annotation is 'int'
            match &annotation.kind {
                TypeKind::Name(type_name) => assert_eq!(type_name, "int"),
                _ => panic!("Expected Name type"),
            }

            // Check value is 10
            assert!(value.is_some());
            match &value.as_ref().unwrap().kind {
                ExpressionKind::Integer(val) => assert_eq!(*val, 10),
                _ => panic!("Expected Integer"),
            }
        }
        _ => panic!("Expected AnnAssign statement"),
    }
}

#[test]
fn test_ann_assign_without_value() {
    let source = "x: int";
    let program = Parser::parse(source).unwrap();

    assert_eq!(program.statements.len(), 1);

    match &program.statements[0].kind {
        StatementKind::AnnAssign {
            target,
            annotation,
            value,
        } => {
            // Check target is identifier 'x'
            match &target.kind {
                ExpressionKind::Identifier(name) => assert_eq!(name, "x"),
                _ => panic!("Expected Identifier expression"),
            }

            // Check annotation is 'int'
            match &annotation.kind {
                TypeKind::Name(type_name) => assert_eq!(type_name, "int"),
                _ => panic!("Expected Name type"),
            }

            // Check value is None
            assert!(value.is_none());
        }
        _ => panic!("Expected AnnAssign statement"),
    }
}

#[test]
fn test_ann_assign_string_type() {
    let source = r#"name: str = "Alice""#;
    let program = Parser::parse(source).unwrap();

    assert_eq!(program.statements.len(), 1);

    match &program.statements[0].kind {
        StatementKind::AnnAssign {
            target,
            annotation,
            value,
        } => {
            // Check target
            match &target.kind {
                ExpressionKind::Identifier(name) => assert_eq!(name, "name"),
                _ => panic!("Expected Identifier expression"),
            }

            // Check annotation is 'str'
            match &annotation.kind {
                TypeKind::Name(type_name) => assert_eq!(type_name, "str"),
                _ => panic!("Expected Name type"),
            }

            // Check value is "Alice"
            assert!(value.is_some());
            match &value.as_ref().unwrap().kind {
                ExpressionKind::String(val) => assert_eq!(val, "Alice"),
                _ => panic!("Expected String"),
            }
        }
        _ => panic!("Expected AnnAssign statement"),
    }
}

#[test]
fn test_ann_assign_float_type() {
    let source = "pi: float = 3.14";
    let program = Parser::parse(source).unwrap();

    assert_eq!(program.statements.len(), 1);

    match &program.statements[0].kind {
        StatementKind::AnnAssign {
            target,
            annotation,
            value,
        } => {
            // Check target
            match &target.kind {
                ExpressionKind::Identifier(name) => assert_eq!(name, "pi"),
                _ => panic!("Expected Identifier expression"),
            }

            // Check annotation is 'float'
            match &annotation.kind {
                TypeKind::Name(type_name) => assert_eq!(type_name, "float"),
                _ => panic!("Expected Name type"),
            }

            // Check value is 3.14
            assert!(value.is_some());
            match &value.as_ref().unwrap().kind {
                ExpressionKind::Float(val) => assert_eq!(*val, 3.14),
                _ => panic!("Expected Float"),
            }
        }
        _ => panic!("Expected AnnAssign statement"),
    }
}

#[test]
fn test_ann_assign_bool_type() {
    let source = "flag: bool = True";
    let program = Parser::parse(source).unwrap();

    assert_eq!(program.statements.len(), 1);

    match &program.statements[0].kind {
        StatementKind::AnnAssign {
            target,
            annotation,
            value,
        } => {
            // Check target
            match &target.kind {
                ExpressionKind::Identifier(name) => assert_eq!(name, "flag"),
                _ => panic!("Expected Identifier expression"),
            }

            // Check annotation is 'bool'
            match &annotation.kind {
                TypeKind::Name(type_name) => assert_eq!(type_name, "bool"),
                _ => panic!("Expected Name type"),
            }

            // Check value is True
            assert!(value.is_some());
            match &value.as_ref().unwrap().kind {
                ExpressionKind::Boolean(val) => assert_eq!(*val, true),
                _ => panic!("Expected Boolean"),
            }
        }
        _ => panic!("Expected AnnAssign statement"),
    }
}

#[test]
fn test_ann_assign_generic_type() {
    let source = "items: list[int] = []";
    let program = Parser::parse(source).unwrap();

    assert_eq!(program.statements.len(), 1);

    match &program.statements[0].kind {
        StatementKind::AnnAssign {
            target,
            annotation,
            value,
        } => {
            // Check target
            match &target.kind {
                ExpressionKind::Identifier(name) => assert_eq!(name, "items"),
                _ => panic!("Expected Identifier expression"),
            }

            // Check annotation is generic type list[int]
            match &annotation.kind {
                TypeKind::Generic { base, args } => {
                    match &base.kind {
                        TypeKind::Name(type_name) => assert_eq!(type_name, "list"),
                        _ => panic!("Expected Name for base type"),
                    }
                    assert_eq!(args.len(), 1);
                    match &args[0].kind {
                        TypeKind::Name(arg_type) => assert_eq!(arg_type, "int"),
                        _ => panic!("Expected Name for generic arg"),
                    }
                }
                _ => panic!("Expected Generic type"),
            }

            // Check value is []
            assert!(value.is_some());
            match &value.as_ref().unwrap().kind {
                ExpressionKind::List { elements } => assert_eq!(elements.len(), 0),
                _ => panic!("Expected List"),
            }
        }
        _ => panic!("Expected AnnAssign statement"),
    }
}

#[test]
fn test_ann_assign_complex_expr() {
    let source = "result: int = 1 + 2 + 3";
    let program = Parser::parse(source).unwrap();

    assert_eq!(program.statements.len(), 1);

    match &program.statements[0].kind {
        StatementKind::AnnAssign {
            target,
            annotation,
            value,
        } => {
            // Check target
            match &target.kind {
                ExpressionKind::Identifier(name) => assert_eq!(name, "result"),
                _ => panic!("Expected Identifier expression"),
            }

            // Check annotation is 'int'
            match &annotation.kind {
                TypeKind::Name(type_name) => assert_eq!(type_name, "int"),
                _ => panic!("Expected Name type"),
            }

            // Check value is a binary operation
            assert!(value.is_some());
            match &value.as_ref().unwrap().kind {
                ExpressionKind::BinaryOp { .. } => {} // Just verify it's a binary op
                _ => panic!("Expected BinaryOp"),
            }
        }
        _ => panic!("Expected AnnAssign statement"),
    }
}

#[test]
fn test_ann_assign_multiple_statements() {
    let source = r#"
x: int = 10
y: str = "hello"
z: float
"#;
    let program = Parser::parse(source).unwrap();

    assert_eq!(program.statements.len(), 3);

    // First statement
    match &program.statements[0].kind {
        StatementKind::AnnAssign {
            target,
            annotation,
            value,
        } => {
            match &target.kind {
                ExpressionKind::Identifier(name) => assert_eq!(name, "x"),
                _ => panic!("Expected Identifier"),
            }
            match &annotation.kind {
                TypeKind::Name(type_name) => assert_eq!(type_name, "int"),
                _ => panic!("Expected Name type"),
            }
            assert!(value.is_some());
        }
        _ => panic!("Expected AnnAssign"),
    }

    // Second statement
    match &program.statements[1].kind {
        StatementKind::AnnAssign {
            target,
            annotation,
            value,
        } => {
            match &target.kind {
                ExpressionKind::Identifier(name) => assert_eq!(name, "y"),
                _ => panic!("Expected Identifier"),
            }
            match &annotation.kind {
                TypeKind::Name(type_name) => assert_eq!(type_name, "str"),
                _ => panic!("Expected Name type"),
            }
            assert!(value.is_some());
        }
        _ => panic!("Expected AnnAssign"),
    }

    // Third statement
    match &program.statements[2].kind {
        StatementKind::AnnAssign {
            target,
            annotation,
            value,
        } => {
            match &target.kind {
                ExpressionKind::Identifier(name) => assert_eq!(name, "z"),
                _ => panic!("Expected Identifier"),
            }
            match &annotation.kind {
                TypeKind::Name(type_name) => assert_eq!(type_name, "float"),
                _ => panic!("Expected Name type"),
            }
            assert!(value.is_none());
        }
        _ => panic!("Expected AnnAssign"),
    }
}

#[test]
fn test_ann_assign_in_function() {
    let source = r#"
def my_func():
    x: int = 42
    return x
"#;
    let program = Parser::parse(source).unwrap();

    assert_eq!(program.statements.len(), 1);

    match &program.statements[0].kind {
        StatementKind::FunctionDef { body, .. } => {
            assert_eq!(body.len(), 2); // x: int = 42 and return x

            // Check first statement in function body is AnnAssign
            match &body[0].kind {
                StatementKind::AnnAssign {
                    target,
                    annotation,
                    value,
                } => {
                    match &target.kind {
                        ExpressionKind::Identifier(name) => assert_eq!(name, "x"),
                        _ => panic!("Expected Identifier"),
                    }
                    match &annotation.kind {
                        TypeKind::Name(type_name) => assert_eq!(type_name, "int"),
                        _ => panic!("Expected Name type"),
                    }
                    assert!(value.is_some());
                }
                _ => panic!("Expected AnnAssign in function body"),
            }
        }
        _ => panic!("Expected FunctionDef"),
    }
}
