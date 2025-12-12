use silk_parser::Parser;

/// Test for loop with simple identifier pattern
#[test]
fn test_for_loop_range() {
    let source = r#"
for i in range(10):
    pass
"#;
    let result = Parser::parse(source);
    match result {
        Ok(_) => println!("✓ Parse succeeded"),
        Err(e) => {
            println!("✗ Parse failed: {:?}", e);
            panic!("Parser failed on 'for i in range(10)'");
        }
    }
}

/// Test for loop with tuple unpacking
#[test]
fn test_for_loop_tuple_unpack() {
    let source = r#"
for x, y in items:
    pass
"#;
    let result = Parser::parse(source);
    assert!(result.is_ok(), "Parser failed on 'for x, y in items'");
}

/// Test for loop with list unpacking
#[test]
fn test_for_loop_list_pattern() {
    let source = r#"
for [a, b] in pairs:
    pass
"#;
    let result = Parser::parse(source);
    assert!(result.is_ok(), "Parser failed on 'for [a, b] in pairs'");
}

/// Test for loop with nested tuple unpacking
#[test]
fn test_for_loop_nested_unpack() {
    let source = r#"
for (a, (b, c)) in nested:
    pass
"#;
    let result = Parser::parse(source);
    assert!(result.is_ok(), "Parser failed on 'for (a, (b, c)) in nested'");
}

/// Test for loop with else clause
#[test]
fn test_for_loop_with_else() {
    let source = r#"
for i in range(10):
    pass
else:
    print("done")
"#;
    let result = Parser::parse(source);
    assert!(result.is_ok(), "Parser failed on 'for...else'");
}

/// Test that 'in' still works as comparison operator in expressions
#[test]
fn test_in_operator_in_expression() {
    let source = r#"
x = 5 in numbers
"#;
    let result = Parser::parse(source);
    assert!(result.is_ok(), "Parser failed on 'x in numbers' expression");
}

/// Test for loop with 'in' operator in the iterator expression
#[test]
fn test_for_loop_with_in_expression() {
    let source = r#"
for item in [x for x in items if x in valid]:
    pass
"#;
    let result = Parser::parse(source);
    assert!(result.is_ok(), "Parser failed on for loop with 'in' in comprehension");
}

/// Test bare except clause (regression test - should still work)
#[test]
fn test_bare_except() {
    let source = r#"
try:
    pass
except:
    pass
"#;
    let result = Parser::parse(source);
    match result {
        Ok(_) => println!("✓ Parse succeeded"),
        Err(e) => {
            println!("✗ Parse failed: {:?}", e);
            panic!("Parser failed on bare except");
        }
    }
}
