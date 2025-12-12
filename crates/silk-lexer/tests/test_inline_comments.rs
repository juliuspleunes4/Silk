/// Tests for inline comment support in the lexer
use silk_lexer::{Lexer, TokenKind};

#[test]
fn test_inline_comment_after_assignment() {
    let source = "x = 10  # this is a comment\n";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    
    // Should get: x, =, 10, Newline, EOF (comment is skipped)
    assert_eq!(tokens.len(), 5);
    assert_eq!(tokens[0].kind, TokenKind::Identifier);
    assert_eq!(tokens[1].kind, TokenKind::Assign);
    assert!(matches!(tokens[2].kind, TokenKind::Integer(_)));
    assert_eq!(tokens[3].kind, TokenKind::Newline);
    assert_eq!(tokens[4].kind, TokenKind::Eof);
}

#[test]
fn test_inline_comment_after_expression() {
    let source = "result = x + y  # calculate sum\n";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    
    // Should get: result, =, x, +, y, Newline, EOF
    assert_eq!(tokens.len(), 7);
    assert_eq!(tokens[0].kind, TokenKind::Identifier);
    assert_eq!(tokens[1].kind, TokenKind::Assign);
    assert_eq!(tokens[2].kind, TokenKind::Identifier);
    assert_eq!(tokens[3].kind, TokenKind::Plus);
    assert_eq!(tokens[4].kind, TokenKind::Identifier);
    assert_eq!(tokens[5].kind, TokenKind::Newline);
    assert_eq!(tokens[6].kind, TokenKind::Eof);
}

#[test]
fn test_inline_comment_on_function_definition() {
    let source = "def add(a, b):  # adds two numbers\n";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    
    // Should get: def, add, (, a, ,, b, ), :, Newline, EOF
    assert_eq!(tokens.len(), 10);
    assert_eq!(tokens[0].kind, TokenKind::Def);
    assert_eq!(tokens[1].kind, TokenKind::Identifier);
    assert_eq!(tokens[8].kind, TokenKind::Newline);
    assert_eq!(tokens[9].kind, TokenKind::Eof);
}

#[test]
fn test_inline_comment_with_special_characters() {
    let source = "x = 5  # TODO: fix this!!! @important\n";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    
    // Comment content doesn't matter, should be skipped
    assert_eq!(tokens.len(), 5);
    assert_eq!(tokens[0].kind, TokenKind::Identifier);
    assert_eq!(tokens[1].kind, TokenKind::Assign);
    assert!(matches!(tokens[2].kind, TokenKind::Integer(_)));
    assert_eq!(tokens[3].kind, TokenKind::Newline);
    assert_eq!(tokens[4].kind, TokenKind::Eof);
}

#[test]
fn test_multiple_inline_comments_in_code() {
    let source = r#"
x = 1  # first variable
y = 2  # second variable
z = x + y  # sum
"#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    
    // Each line: var, =, value, newline
    // Plus initial newline and EOF
    let mut newline_count = 0;
    let mut identifier_count = 0;
    
    for token in &tokens {
        match token.kind {
            TokenKind::Newline => newline_count += 1,
            TokenKind::Identifier => identifier_count += 1,
            _ => {}
        }
    }
    
    // Should have 4 newlines (initial blank line + 3 lines of code)
    assert_eq!(newline_count, 4);
    // Should have 5 identifiers (x, y, z, x, y)
    assert_eq!(identifier_count, 5);
}

#[test]
fn test_inline_comment_after_string() {
    let source = r#"msg = "hello"  # greeting message"#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    
    // No newline at end since raw string doesn't include \n
    assert_eq!(tokens.len(), 4);
    assert_eq!(tokens[0].kind, TokenKind::Identifier);
    assert_eq!(tokens[1].kind, TokenKind::Assign);
    assert!(matches!(tokens[2].kind, TokenKind::String(_)));
    assert_eq!(tokens[2].lexeme, "\"hello\"");
    assert_eq!(tokens[3].kind, TokenKind::Eof);
}

#[test]
fn test_inline_comment_after_number() {
    let source = "pi = 3.14159  # approximate value\n";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    
    assert_eq!(tokens.len(), 5);
    assert!(matches!(tokens[2].kind, TokenKind::Float(_)));
}

#[test]
fn test_inline_comment_after_parenthesis() {
    let source = "result = func()  # call function\n";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    
    // result, =, func, (, ), Newline, EOF
    assert_eq!(tokens.len(), 7);
    assert_eq!(tokens[3].kind, TokenKind::LeftParen);
    assert_eq!(tokens[4].kind, TokenKind::RightParen);
    assert_eq!(tokens[5].kind, TokenKind::Newline);
}

#[test]
fn test_inline_comment_after_bracket() {
    let source = "items = [1, 2, 3]  # list of numbers\n";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    
    // Should end with ], Newline, EOF
    let len = tokens.len();
    assert_eq!(tokens[len - 3].kind, TokenKind::RightBracket);
    assert_eq!(tokens[len - 2].kind, TokenKind::Newline);
    assert_eq!(tokens[len - 1].kind, TokenKind::Eof);
}

#[test]
fn test_inline_comment_with_hash_in_string() {
    let source = r#"text = "use # for comments"  # this is real comment"#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    
    // The # inside the string should be part of the string
    // No newline at end since raw string doesn't include \n
    assert_eq!(tokens.len(), 4);
    assert!(matches!(tokens[2].kind, TokenKind::String(_)));
    assert_eq!(tokens[2].lexeme, r#""use # for comments""#);
    assert_eq!(tokens[3].kind, TokenKind::Eof);
}

#[test]
fn test_standalone_comment_still_works() {
    let source = r#"
# This is a standalone comment
x = 5
"#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    
    // Should have: Newline (blank), Comment, Newline, x, =, 5, Newline, EOF
    assert_eq!(tokens[1].kind, TokenKind::Comment);
    assert_eq!(tokens[1].lexeme, "# This is a standalone comment");
}

#[test]
fn test_inline_comment_on_class_definition() {
    let source = "class Person:  # represents a person\n";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    
    // class, Person, :, Newline, EOF
    assert_eq!(tokens.len(), 5);
    assert_eq!(tokens[0].kind, TokenKind::Class);
    assert_eq!(tokens[1].kind, TokenKind::Identifier);
    assert_eq!(tokens[2].kind, TokenKind::Colon);
    assert_eq!(tokens[3].kind, TokenKind::Newline);
}

#[test]
fn test_inline_comment_after_return() {
    let source = "return x + 1  # increment and return\n";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    
    // return, x, +, 1, Newline, EOF
    assert_eq!(tokens.len(), 6);
    assert_eq!(tokens[0].kind, TokenKind::Return);
    assert_eq!(tokens[4].kind, TokenKind::Newline);
}

#[test]
fn test_inline_comment_with_no_space_before_hash() {
    let source = "x = 10# comment with no space\n";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    
    // Should still work - comment should be skipped
    assert_eq!(tokens.len(), 5);
    assert_eq!(tokens[0].kind, TokenKind::Identifier);
    assert!(matches!(tokens[2].kind, TokenKind::Integer(_)));
    assert_eq!(tokens[3].kind, TokenKind::Newline);
}

#[test]
fn test_inline_comment_on_import() {
    let source = "import math  # for mathematical functions\n";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    
    // import, math, Newline, EOF
    assert_eq!(tokens.len(), 4);
    assert_eq!(tokens[0].kind, TokenKind::Import);
    assert_eq!(tokens[1].kind, TokenKind::Identifier);
    assert_eq!(tokens[2].kind, TokenKind::Newline);
}

#[test]
fn test_inline_comment_on_if_statement() {
    let source = "if x > 0:  # check if positive\n";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    
    // if, x, >, 0, :, Newline, EOF
    assert_eq!(tokens.len(), 7);
    assert_eq!(tokens[0].kind, TokenKind::If);
    assert_eq!(tokens[4].kind, TokenKind::Colon);
    assert_eq!(tokens[5].kind, TokenKind::Newline);
}

#[test]
fn test_comment_at_end_of_file_no_newline() {
    let source = "x = 1  # comment at end";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    
    // x, =, 1, EOF (no newline when file doesn't end with one)
    // The comment is skipped
    assert_eq!(tokens.len(), 4);
    assert_eq!(tokens[0].kind, TokenKind::Identifier);
    assert_eq!(tokens[1].kind, TokenKind::Assign);
    assert!(matches!(tokens[2].kind, TokenKind::Integer(_)));
    assert_eq!(tokens[3].kind, TokenKind::Eof);
}

#[test]
fn test_empty_inline_comment() {
    let source = "x = 5  #";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    
    // Should work fine - empty comment is valid
    // No newline at end
    assert_eq!(tokens.len(), 4);
    assert_eq!(tokens[0].kind, TokenKind::Identifier);
    assert_eq!(tokens[3].kind, TokenKind::Eof);
}

#[test]
fn test_inline_comment_preserves_line_numbers() {
    let source = r#"
x = 1  # line 2
y = 2  # line 3
z = 3  # line 4
"#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    
    // Find identifier tokens and check their line numbers
    let identifiers: Vec<_> = tokens.iter()
        .filter(|t| t.kind == TokenKind::Identifier)
        .collect();
    
    assert_eq!(identifiers[0].span.line, 2); // x on line 2
    assert_eq!(identifiers[1].span.line, 3); // y on line 3
    assert_eq!(identifiers[2].span.line, 4); // z on line 4
}

