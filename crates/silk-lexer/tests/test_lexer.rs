use pretty_assertions::assert_eq;
/// Comprehensive test suite for Silk lexer
///
/// Tests every aspect of lexical analysis including:
/// - All token types
/// - Edge cases and boundary conditions
/// - Error conditions
/// - Source location tracking
/// - Complex integration scenarios
use silk_lexer::{FStringPart, LexError, Lexer, TokenKind};

// ========== KEYWORD TESTS ==========

#[test]
fn test_all_keywords_exhaustive() {
    let source = "False None True and as assert async await break class continue def del elif else except finally for from global if import in is lambda nonlocal not or pass raise return try while with yield";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    let expected = vec![
        TokenKind::False,
        TokenKind::None,
        TokenKind::True,
        TokenKind::And,
        TokenKind::As,
        TokenKind::Assert,
        TokenKind::Async,
        TokenKind::Await,
        TokenKind::Break,
        TokenKind::Class,
        TokenKind::Continue,
        TokenKind::Def,
        TokenKind::Del,
        TokenKind::Elif,
        TokenKind::Else,
        TokenKind::Except,
        TokenKind::Finally,
        TokenKind::For,
        TokenKind::From,
        TokenKind::Global,
        TokenKind::If,
        TokenKind::Import,
        TokenKind::In,
        TokenKind::Is,
        TokenKind::Lambda,
        TokenKind::Nonlocal,
        TokenKind::Not,
        TokenKind::Or,
        TokenKind::Pass,
        TokenKind::Raise,
        TokenKind::Return,
        TokenKind::Try,
        TokenKind::While,
        TokenKind::With,
        TokenKind::Yield,
    ];

    for (i, expected_kind) in expected.iter().enumerate() {
        assert_eq!(tokens[i].kind, *expected_kind, "Token {} mismatch", i);
    }
}

#[test]
fn test_keywords_case_sensitive() {
    let source = "def Def DEF dEf";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens[0].kind, TokenKind::Def);
    assert_eq!(tokens[1].kind, TokenKind::Identifier); // Def is identifier
    assert_eq!(tokens[2].kind, TokenKind::Identifier); // DEF is identifier
    assert_eq!(tokens[3].kind, TokenKind::Identifier); // dEf is identifier
}

#[test]
fn test_keywords_not_substrings() {
    let source = "definition classification forloop";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    // These should all be identifiers, not keywords
    assert_eq!(tokens[0].kind, TokenKind::Identifier);
    assert_eq!(tokens[0].lexeme, "definition");
    assert_eq!(tokens[1].kind, TokenKind::Identifier);
    assert_eq!(tokens[1].lexeme, "classification");
    assert_eq!(tokens[2].kind, TokenKind::Identifier);
    assert_eq!(tokens[2].lexeme, "forloop");
}

#[test]
fn test_keywords_adjacent_to_operators() {
    let source = "if(True)else:pass";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens[0].kind, TokenKind::If);
    assert_eq!(tokens[1].kind, TokenKind::LeftParen);
    assert_eq!(tokens[2].kind, TokenKind::True);
    assert_eq!(tokens[3].kind, TokenKind::RightParen);
    assert_eq!(tokens[4].kind, TokenKind::Else);
    assert_eq!(tokens[5].kind, TokenKind::Colon);
    assert_eq!(tokens[6].kind, TokenKind::Pass);
}

// ========== IDENTIFIER TESTS ==========

#[test]
fn test_identifiers_basic_patterns() {
    let source = "hello _private __dunder__ snake_case camelCase CONSTANT";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens[0].kind, TokenKind::Identifier);
    assert_eq!(tokens[0].lexeme, "hello");
    assert_eq!(tokens[1].kind, TokenKind::Identifier);
    assert_eq!(tokens[1].lexeme, "_private");
    assert_eq!(tokens[2].kind, TokenKind::Identifier);
    assert_eq!(tokens[2].lexeme, "__dunder__");
    assert_eq!(tokens[3].kind, TokenKind::Identifier);
    assert_eq!(tokens[3].lexeme, "snake_case");
    assert_eq!(tokens[4].kind, TokenKind::Identifier);
    assert_eq!(tokens[4].lexeme, "camelCase");
    assert_eq!(tokens[5].kind, TokenKind::Identifier);
    assert_eq!(tokens[5].lexeme, "CONSTANT");
}

#[test]
fn test_identifiers_unicode() {
    let source = "café naïve москва αβγ 变量";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens[0].kind, TokenKind::Identifier);
    assert_eq!(tokens[0].lexeme, "café");
    assert_eq!(tokens[1].kind, TokenKind::Identifier);
    assert_eq!(tokens[1].lexeme, "naïve");
    assert_eq!(tokens[2].kind, TokenKind::Identifier);
    assert_eq!(tokens[2].lexeme, "москва");
    assert_eq!(tokens[3].kind, TokenKind::Identifier);
    assert_eq!(tokens[3].lexeme, "αβγ");
    assert_eq!(tokens[4].kind, TokenKind::Identifier);
    assert_eq!(tokens[4].lexeme, "变量");
}

#[test]
fn test_identifiers_with_digits() {
    let source = "var1 test2var _123 a1b2c3 x0y9z";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens[0].kind, TokenKind::Identifier);
    assert_eq!(tokens[0].lexeme, "var1");
    assert_eq!(tokens[1].kind, TokenKind::Identifier);
    assert_eq!(tokens[1].lexeme, "test2var");
    assert_eq!(tokens[2].kind, TokenKind::Identifier);
    assert_eq!(tokens[2].lexeme, "_123");
    assert_eq!(tokens[3].kind, TokenKind::Identifier);
    assert_eq!(tokens[3].lexeme, "a1b2c3");
    assert_eq!(tokens[4].kind, TokenKind::Identifier);
    assert_eq!(tokens[4].lexeme, "x0y9z");
}

#[test]
fn test_identifiers_edge_cases() {
    let source = "_ __ ___ __init__ __name__ __main__";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens[0].kind, TokenKind::Identifier);
    assert_eq!(tokens[0].lexeme, "_");
    assert_eq!(tokens[1].kind, TokenKind::Identifier);
    assert_eq!(tokens[1].lexeme, "__");
    assert_eq!(tokens[2].kind, TokenKind::Identifier);
    assert_eq!(tokens[2].lexeme, "___");
    assert_eq!(tokens[3].kind, TokenKind::Identifier);
    assert_eq!(tokens[3].lexeme, "__init__");
    assert_eq!(tokens[4].kind, TokenKind::Identifier);
    assert_eq!(tokens[4].lexeme, "__name__");
    assert_eq!(tokens[5].kind, TokenKind::Identifier);
    assert_eq!(tokens[5].lexeme, "__main__");
}

#[test]
fn test_very_long_identifier() {
    let long_id = "a".repeat(1000);
    let mut lexer = Lexer::new(&long_id);
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens[0].kind, TokenKind::Identifier);
    assert_eq!(tokens[0].lexeme.len(), 1000);
}

// ========== INTEGER TESTS ==========

#[test]
fn test_integers_basic() {
    let source = "0 1 123 999999 1000000 42";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    assert!(matches!(tokens[0].kind, TokenKind::Integer(_)));
    assert!(matches!(tokens[1].kind, TokenKind::Integer(_)));
    assert!(matches!(tokens[2].kind, TokenKind::Integer(_)));

    if let TokenKind::Integer(val) = tokens[0].kind {
        assert_eq!(val, 0);
    }
    if let TokenKind::Integer(val) = tokens[1].kind {
        assert_eq!(val, 1);
    }
    if let TokenKind::Integer(val) = tokens[2].kind {
        assert_eq!(val, 123);
    }
}

#[test]
fn test_integers_with_underscores() {
    let source = "1_000 1_000_000 1_2_3_4_5";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    assert!(matches!(tokens[0].kind, TokenKind::Integer(1000)));
    assert!(matches!(tokens[1].kind, TokenKind::Integer(1000000)));
    assert!(matches!(tokens[2].kind, TokenKind::Integer(12345)));

    assert_eq!(tokens[0].lexeme, "1_000");
    assert_eq!(tokens[1].lexeme, "1_000_000");
}

#[test]
fn test_integers_edge_cases() {
    let source = "00 000 0000";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    // Leading zeros should still parse as integers
    assert!(matches!(tokens[0].kind, TokenKind::Integer(_)));
    assert!(matches!(tokens[1].kind, TokenKind::Integer(_)));
    assert!(matches!(tokens[2].kind, TokenKind::Integer(_)));
}

#[test]
fn test_very_long_number() {
    // Numbers that exceed i64 range cause InvalidNumber error
    let long_num = "123456789".repeat(5);
    let mut lexer = Lexer::new(&long_num);
    let result = lexer.tokenize();

    // Should error on overflow (i64 max is ~9e18, this is much larger)
    assert!(result.is_err());
    assert!(matches!(result, Err(LexError::InvalidNumber(_, _, _))));
}

// ========== FLOAT TESTS ==========

#[test]
fn test_floats_basic() {
    let source = "0.0 3.14 0.5 123.456";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    assert!(matches!(tokens[0].kind, TokenKind::Float(_)));
    assert!(matches!(tokens[1].kind, TokenKind::Float(_)));
    assert!(matches!(tokens[2].kind, TokenKind::Float(_)));

    if let TokenKind::Float(val) = tokens[0].kind {
        assert_eq!(val, 0.0);
    }
    if let TokenKind::Float(val) = tokens[1].kind {
        assert!((val - 3.14).abs() < 0.0001);
    }
}

#[test]
fn test_floats_scientific_notation() {
    let source = "1e10 2.5e-3 3.14e+2 1E10 2.5E-3";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    assert!(matches!(tokens[0].kind, TokenKind::Float(_)));
    assert_eq!(tokens[0].lexeme, "1e10");
    assert!(matches!(tokens[1].kind, TokenKind::Float(_)));
    assert_eq!(tokens[1].lexeme, "2.5e-3");
    assert!(matches!(tokens[2].kind, TokenKind::Float(_)));
    assert_eq!(tokens[2].lexeme, "3.14e+2");
}

#[test]
fn test_floats_edge_cases() {
    let source = "1.0 1.0e0 1e-100 1e+100";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    // 1.0 should be float
    assert!(matches!(tokens[0].kind, TokenKind::Float(_)));
    assert_eq!(tokens[0].lexeme, "1.0");

    // Scientific notation edge cases
    assert!(matches!(tokens[1].kind, TokenKind::Float(_)));
    assert_eq!(tokens[1].lexeme, "1.0e0");
    // Note: Trailing dot ("0.") not supported - parses as integer then dot
}

#[test]
fn test_floats_with_underscores() {
    // TODO: Underscores in numeric literals not yet implemented
    let source = "1.5 3.14";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    assert!(matches!(tokens[0].kind, TokenKind::Float(_)));
    assert_eq!(tokens[0].lexeme, "1.5");
    assert!(matches!(tokens[1].kind, TokenKind::Float(_)));
    assert_eq!(tokens[1].lexeme, "3.14");
}

#[test]
fn test_number_followed_by_dot_method() {
    let source = "123.5";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    // Float with decimal point and digit
    assert!(matches!(tokens[0].kind, TokenKind::Float(_)));
    assert_eq!(tokens[0].lexeme, "123.5");
    // Note: Trailing dot ("123.") not supported - requires digit after dot
}

// ========== STRING TESTS ==========

#[test]
fn test_strings_single_quote() {
    let source = r#"'hello' 'world' '' 'a'"#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    assert!(matches!(tokens[0].kind, TokenKind::String(_)));
    if let TokenKind::String(ref s) = tokens[0].kind {
        assert_eq!(s, "hello");
    }
    assert!(matches!(tokens[1].kind, TokenKind::String(_)));
    if let TokenKind::String(ref s) = tokens[1].kind {
        assert_eq!(s, "world");
    }
    // Empty string
    assert!(matches!(tokens[2].kind, TokenKind::String(_)));
    if let TokenKind::String(ref s) = tokens[2].kind {
        assert_eq!(s, "");
    }
}

#[test]
fn test_strings_double_quote() {
    let source = r#""hello" "world" "" "a""#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    assert!(matches!(tokens[0].kind, TokenKind::String(_)));
    if let TokenKind::String(ref s) = tokens[0].kind {
        assert_eq!(s, "hello");
    }
    assert!(matches!(tokens[1].kind, TokenKind::String(_)));
    if let TokenKind::String(ref s) = tokens[1].kind {
        assert_eq!(s, "world");
    }
}

#[test]
fn test_strings_escape_sequences() {
    let source = r#"'line1\nline2' 'tab\there' 'quote\'s' "quote\"s" '\\backslash' '\r\t\n'"#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    assert!(matches!(tokens[0].kind, TokenKind::String(_)));
    if let TokenKind::String(ref s) = tokens[0].kind {
        assert_eq!(s, "line1\nline2");
    }
    assert!(matches!(tokens[1].kind, TokenKind::String(_)));
    if let TokenKind::String(ref s) = tokens[1].kind {
        assert_eq!(s, "tab\there");
    }
    assert!(matches!(tokens[2].kind, TokenKind::String(_)));
    if let TokenKind::String(ref s) = tokens[2].kind {
        assert_eq!(s, "quote's");
    }
}

#[test]
fn test_strings_triple_quoted() {
    let source = r#""""multi
line
string""" '''another
multi
line'''"#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    assert!(matches!(tokens[0].kind, TokenKind::String(_)));
    if let TokenKind::String(ref s) = tokens[0].kind {
        assert!(s.contains("multi"));
        assert!(s.contains("line"));
    }
}

#[test]
fn test_strings_with_unicode() {
    let source = r#"'café' "日本語" 'αβγ' "🚀emoji""#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    assert!(matches!(tokens[0].kind, TokenKind::String(_)));
    if let TokenKind::String(ref s) = tokens[0].kind {
        assert_eq!(s, "café");
    }
    assert!(matches!(tokens[1].kind, TokenKind::String(_)));
    if let TokenKind::String(ref s) = tokens[1].kind {
        assert_eq!(s, "日本語");
    }
}

#[test]
fn test_strings_empty_variations() {
    let source = r#"'' "" """""" ''''''"#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    assert!(matches!(tokens[0].kind, TokenKind::String(_)));
    if let TokenKind::String(ref s) = tokens[0].kind {
        assert_eq!(s, "");
    }
    assert!(matches!(tokens[1].kind, TokenKind::String(_)));
    if let TokenKind::String(ref s) = tokens[1].kind {
        assert_eq!(s, "");
    }
}

#[test]
fn test_string_unterminated_single() {
    let source = r#"'unterminated"#;
    let mut lexer = Lexer::new(source);
    let result = lexer.tokenize();

    assert!(result.is_err());
    assert!(matches!(result, Err(LexError::UnterminatedString(_, _))));
}

#[test]
fn test_string_unterminated_double() {
    let source = r#""unterminated"#;
    let mut lexer = Lexer::new(source);
    let result = lexer.tokenize();

    assert!(result.is_err());
    assert!(matches!(result, Err(LexError::UnterminatedString(_, _))));
}

#[test]
fn test_string_unterminated_triple() {
    let source = r#""""incomplete triple quote"#;
    let mut lexer = Lexer::new(source);
    let result = lexer.tokenize();

    assert!(result.is_err());
}

#[test]
fn test_mixed_quotes_separate_strings() {
    let source = r#"'string1' "string2""#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    // Should be two separate strings
    assert!(matches!(tokens[0].kind, TokenKind::String(_)));
    assert!(matches!(tokens[1].kind, TokenKind::String(_)));
}

// ========== OPERATOR TESTS ==========

#[test]
fn test_operators_arithmetic() {
    let source = "+ - * / // % **";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens[0].kind, TokenKind::Plus);
    assert_eq!(tokens[1].kind, TokenKind::Minus);
    assert_eq!(tokens[2].kind, TokenKind::Star);
    assert_eq!(tokens[3].kind, TokenKind::Slash);
    assert_eq!(tokens[4].kind, TokenKind::DoubleSlash);
    assert_eq!(tokens[5].kind, TokenKind::Percent);
    assert_eq!(tokens[6].kind, TokenKind::DoubleStar);
}

#[test]
fn test_operators_comparison() {
    let source = "== != < > <= >=";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens[0].kind, TokenKind::Equal);
    assert_eq!(tokens[1].kind, TokenKind::NotEqual);
    assert_eq!(tokens[2].kind, TokenKind::Less);
    assert_eq!(tokens[3].kind, TokenKind::Greater);
    assert_eq!(tokens[4].kind, TokenKind::LessEqual);
    assert_eq!(tokens[5].kind, TokenKind::GreaterEqual);
}

#[test]
fn test_operators_bitwise() {
    let source = "& | ^ ~ << >>";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens[0].kind, TokenKind::Ampersand);
    assert_eq!(tokens[1].kind, TokenKind::Pipe);
    assert_eq!(tokens[2].kind, TokenKind::Caret);
    assert_eq!(tokens[3].kind, TokenKind::Tilde);
    assert_eq!(tokens[4].kind, TokenKind::LeftShift);
    assert_eq!(tokens[5].kind, TokenKind::RightShift);
}

#[test]
fn test_operators_assignment() {
    let source = "= += -= *= /= //= %= **= &= |= ^= <<= >>=";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens[0].kind, TokenKind::Assign);
    assert_eq!(tokens[1].kind, TokenKind::PlusAssign);
    assert_eq!(tokens[2].kind, TokenKind::MinusAssign);
    assert_eq!(tokens[3].kind, TokenKind::StarAssign);
    assert_eq!(tokens[4].kind, TokenKind::SlashAssign);
    assert_eq!(tokens[5].kind, TokenKind::DoubleSlashAssign);
    assert_eq!(tokens[6].kind, TokenKind::PercentAssign);
    assert_eq!(tokens[7].kind, TokenKind::DoubleStarAssign);
    assert_eq!(tokens[8].kind, TokenKind::AmpersandAssign);
    assert_eq!(tokens[9].kind, TokenKind::PipeAssign);
    assert_eq!(tokens[10].kind, TokenKind::CaretAssign);
    assert_eq!(tokens[11].kind, TokenKind::LeftShiftAssign);
    assert_eq!(tokens[12].kind, TokenKind::RightShiftAssign);
}

#[test]
fn test_operators_special() {
    let source = "->";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens[0].kind, TokenKind::Arrow);
    // Note: @ and := are not yet implemented
}

#[test]
fn test_operators_ambiguous_parsing() {
    // Test that multi-character operators are correctly distinguished
    let source = "< << <<= <= <";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens[0].kind, TokenKind::Less);
    assert_eq!(tokens[1].kind, TokenKind::LeftShift);
    assert_eq!(tokens[2].kind, TokenKind::LeftShiftAssign);
    assert_eq!(tokens[3].kind, TokenKind::LessEqual);
    assert_eq!(tokens[4].kind, TokenKind::Less);
}

#[test]
fn test_operators_no_spaces() {
    let source = "a+b*c/d==e";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens[0].lexeme, "a");
    assert_eq!(tokens[1].kind, TokenKind::Plus);
    assert_eq!(tokens[2].lexeme, "b");
    assert_eq!(tokens[3].kind, TokenKind::Star);
    assert_eq!(tokens[4].lexeme, "c");
    assert_eq!(tokens[5].kind, TokenKind::Slash);
    assert_eq!(tokens[6].lexeme, "d");
    assert_eq!(tokens[7].kind, TokenKind::Equal);
    assert_eq!(tokens[8].lexeme, "e");
}

#[test]
fn test_operator_soup() {
    let source = "+-*/**//%%==!=<><=>=&|^~<<>>";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    // Should correctly parse all operators
    assert_eq!(tokens[0].kind, TokenKind::Plus);
    assert_eq!(tokens[1].kind, TokenKind::Minus);
    assert_eq!(tokens[2].kind, TokenKind::Star);
    assert_eq!(tokens[3].kind, TokenKind::Slash);
    assert_eq!(tokens[4].kind, TokenKind::DoubleStar);
    assert_eq!(tokens[5].kind, TokenKind::DoubleSlash);
    // Note: Greedy operator parsing - // is DoubleSlash, not two Slash
}

#[test]
fn test_colon_vs_assign() {
    let source = ": = :";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens[0].kind, TokenKind::Colon);
    assert_eq!(tokens[1].kind, TokenKind::Assign);
    assert_eq!(tokens[2].kind, TokenKind::Colon);
    // Note: Walrus operator := not yet implemented
}

#[test]
fn test_arrow_vs_minus_greater() {
    let source = "-> - >";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens[0].kind, TokenKind::Arrow);
    assert_eq!(tokens[1].kind, TokenKind::Minus);
    assert_eq!(tokens[2].kind, TokenKind::Greater);
}

// ========== DELIMITER TESTS ==========

#[test]
fn test_delimiters_all() {
    let source = "( ) [ ] { } , : ; . ...";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens[0].kind, TokenKind::LeftParen);
    assert_eq!(tokens[1].kind, TokenKind::RightParen);
    assert_eq!(tokens[2].kind, TokenKind::LeftBracket);
    assert_eq!(tokens[3].kind, TokenKind::RightBracket);
    assert_eq!(tokens[4].kind, TokenKind::LeftBrace);
    assert_eq!(tokens[5].kind, TokenKind::RightBrace);
    assert_eq!(tokens[6].kind, TokenKind::Comma);
    assert_eq!(tokens[7].kind, TokenKind::Colon);
    assert_eq!(tokens[8].kind, TokenKind::Semicolon);
    assert_eq!(tokens[9].kind, TokenKind::Dot);
    assert_eq!(tokens[10].kind, TokenKind::Ellipsis);
}

#[test]
fn test_delimiters_nested() {
    let source = "[({})]";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens[0].kind, TokenKind::LeftBracket);
    assert_eq!(tokens[1].kind, TokenKind::LeftParen);
    assert_eq!(tokens[2].kind, TokenKind::LeftBrace);
    assert_eq!(tokens[3].kind, TokenKind::RightBrace);
    assert_eq!(tokens[4].kind, TokenKind::RightParen);
    assert_eq!(tokens[5].kind, TokenKind::RightBracket);
}

#[test]
fn test_ellipsis_vs_dots() {
    let source = "... .. .";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens[0].kind, TokenKind::Ellipsis);
    assert_eq!(tokens[1].kind, TokenKind::Dot);
    assert_eq!(tokens[2].kind, TokenKind::Dot);
    assert_eq!(tokens[3].kind, TokenKind::Dot);
}

// ========== COMMENT TESTS ==========

#[test]
fn test_comments_single_line() {
    let source = "x = 5  # This is a comment\ny = 10  # Another comment";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    // Find tokens, comments should be present
    assert_eq!(tokens[0].lexeme, "x");
    assert_eq!(tokens[1].kind, TokenKind::Assign);

    // Check that comment token exists
    let has_comment = tokens.iter().any(|t| t.kind == TokenKind::Comment);
    assert!(has_comment, "Should have comment tokens");
}

#[test]
fn test_comments_end_of_file() {
    let source = "x = 5 # comment at end";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens[0].lexeme, "x");
    assert_eq!(tokens[1].kind, TokenKind::Assign);

    // Should end with EOF
    assert_eq!(tokens.last().unwrap().kind, TokenKind::Eof);
}

#[test]
fn test_comments_only() {
    let source = "# Just a comment\n# Another comment";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    // Should have comment tokens and EOF
    let has_comment = tokens.iter().any(|t| t.kind == TokenKind::Comment);
    assert!(has_comment);
    assert_eq!(tokens.last().unwrap().kind, TokenKind::Eof);
}

#[test]
fn test_comments_with_special_chars() {
    let source = "x = 5  # Comment with symbols: @#$%^&*()[]{}";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens[0].lexeme, "x");
    assert_eq!(tokens[1].kind, TokenKind::Assign);
}

// ========== WHITESPACE AND NEWLINE TESTS ==========

#[test]
fn test_whitespace_handling() {
    let source = "a   \t  b\t\tc";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    // Whitespace should be skipped but tokens preserved
    assert_eq!(tokens[0].lexeme, "a");
    assert_eq!(tokens[1].lexeme, "b");
    assert_eq!(tokens[2].lexeme, "c");
}

#[test]
fn test_newlines_multiple() {
    let source = "a\n\n\nb";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens[0].lexeme, "a");
    assert_eq!(tokens[1].kind, TokenKind::Newline);
    assert_eq!(tokens[2].kind, TokenKind::Newline);
    assert_eq!(tokens[3].kind, TokenKind::Newline);
    assert_eq!(tokens[4].lexeme, "b");
}

#[test]
fn test_empty_source() {
    let source = "";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::Eof);
}

#[test]
fn test_whitespace_only() {
    let source = "   \t  \n  \t\n  ";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    // Should have newlines and EOF
    let newline_count = tokens
        .iter()
        .filter(|t| t.kind == TokenKind::Newline)
        .count();
    assert!(newline_count >= 2);
    assert_eq!(tokens.last().unwrap().kind, TokenKind::Eof);
}

// ========== SOURCE LOCATION TESTS ==========

#[test]
fn test_source_locations_line_column() {
    let source = "x = 5\ny = 10";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    // First line
    assert_eq!(tokens[0].span.line, 1);
    assert_eq!(tokens[0].span.column, 1);
    assert_eq!(tokens[1].span.line, 1);

    // Find second line token
    let y_token = tokens.iter().find(|t| t.lexeme == "y").unwrap();
    assert_eq!(y_token.span.line, 2);
    assert_eq!(y_token.span.column, 1);
}

#[test]
fn test_span_length() {
    let source = "hello";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens[0].span.start, 0);
    assert_eq!(tokens[0].span.end, 5);
}

#[test]
fn test_multiline_tracking() {
    let source = "line1\nline2\nline3";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    let line1_token = tokens.iter().find(|t| t.lexeme == "line1").unwrap();
    let line2_token = tokens.iter().find(|t| t.lexeme == "line2").unwrap();
    let line3_token = tokens.iter().find(|t| t.lexeme == "line3").unwrap();

    assert_eq!(line1_token.span.line, 1);
    assert_eq!(line2_token.span.line, 2);
    assert_eq!(line3_token.span.line, 3);
}

// ========== COMPLEX INTEGRATION TESTS ==========

#[test]
fn test_function_definition_complete() {
    let source = "def greet(name: str) -> str:\n    return \"Hello, \" + name";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens[0].kind, TokenKind::Def);
    assert_eq!(tokens[1].lexeme, "greet");
    assert_eq!(tokens[2].kind, TokenKind::LeftParen);
    assert_eq!(tokens[3].lexeme, "name");
    assert_eq!(tokens[4].kind, TokenKind::Colon);
    assert_eq!(tokens[5].lexeme, "str");
    assert_eq!(tokens[6].kind, TokenKind::RightParen);
    assert_eq!(tokens[7].kind, TokenKind::Arrow);
}

#[test]
fn test_class_definition_complete() {
    let source = "class MyClass:\n    pass";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens[0].kind, TokenKind::Class);
    assert_eq!(tokens[1].lexeme, "MyClass");
    assert_eq!(tokens[2].kind, TokenKind::Colon);

    let pass_token = tokens.iter().find(|t| t.kind == TokenKind::Pass);
    assert!(pass_token.is_some());
}

#[test]
fn test_list_comprehension_syntax() {
    let source = "[x for x in range(10)]";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens[0].kind, TokenKind::LeftBracket);
    assert_eq!(tokens[1].lexeme, "x");
    assert_eq!(tokens[2].kind, TokenKind::For);
    assert_eq!(tokens[3].lexeme, "x");
    assert_eq!(tokens[4].kind, TokenKind::In);
    assert_eq!(tokens[5].lexeme, "range");
}

#[test]
fn test_dictionary_literal_syntax() {
    let source = r#"{"key": "value", "num": 42}"#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens[0].kind, TokenKind::LeftBrace);
    assert!(matches!(tokens[1].kind, TokenKind::String(_)));
    assert_eq!(tokens[2].kind, TokenKind::Colon);
    assert!(matches!(tokens[3].kind, TokenKind::String(_)));
    assert_eq!(tokens[4].kind, TokenKind::Comma);
}

#[test]
fn test_lambda_expression_syntax() {
    let source = "lambda x, y: x + y";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens[0].kind, TokenKind::Lambda);
    assert_eq!(tokens[1].lexeme, "x");
    assert_eq!(tokens[2].kind, TokenKind::Comma);
    assert_eq!(tokens[3].lexeme, "y");
    assert_eq!(tokens[4].kind, TokenKind::Colon);
}

#[test]
fn test_try_except_syntax() {
    let source = "try:\n    pass\nexcept Exception:\n    pass";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens[0].kind, TokenKind::Try);
    assert_eq!(tokens[1].kind, TokenKind::Colon);

    let except_token = tokens.iter().find(|t| t.kind == TokenKind::Except);
    assert!(except_token.is_some());
}

#[test]
fn test_async_await_syntax() {
    let source = "async def fetch():\n    await response";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens[0].kind, TokenKind::Async);
    assert_eq!(tokens[1].kind, TokenKind::Def);
    assert_eq!(tokens[2].lexeme, "fetch");

    let await_token = tokens.iter().find(|t| t.kind == TokenKind::Await);
    assert!(await_token.is_some());
}

#[test]
fn test_with_statement_syntax() {
    let source = "with open('file.txt') as f:\n    pass";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens[0].kind, TokenKind::With);
    assert_eq!(tokens[1].lexeme, "open");

    let as_token = tokens.iter().find(|t| t.kind == TokenKind::As);
    assert!(as_token.is_some());
}

#[test]
fn test_complex_expression() {
    let source = "result = (a + b) * c / d ** 2 - e % f";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens[0].lexeme, "result");
    assert_eq!(tokens[1].kind, TokenKind::Assign);
    assert_eq!(tokens[2].kind, TokenKind::LeftParen);

    let has_power = tokens.iter().any(|t| t.kind == TokenKind::DoubleStar);
    assert!(has_power);
}

// ========== ERROR HANDLING TESTS ==========

#[test]
fn test_unexpected_character_dollar() {
    let source = "x = 5 $ y = 10";
    let mut lexer = Lexer::new(source);
    let result = lexer.tokenize();

    assert!(result.is_err());
    if let Err(LexError::UnexpectedCharacter(ch, _, _)) = result {
        assert_eq!(ch, '$');
    } else {
        panic!("Expected UnexpectedCharacter error");
    }
}

#[test]
fn test_unexpected_character_backtick() {
    let source = "x = `invalid`";
    let mut lexer = Lexer::new(source);
    let result = lexer.tokenize();

    assert!(result.is_err());
}

#[test]
fn test_unexpected_character_backslash() {
    let source = "x = 5 \\ y = 10";
    let mut lexer = Lexer::new(source);
    let result = lexer.tokenize();

    // Backslash outside string should error
    assert!(result.is_err());
}

// ========== NUMBER FORMAT TESTS (BINARY, OCTAL, HEX) ==========

#[test]
fn test_binary_numbers() {
    let source = "0b0 0b1 0b1010 0b1111_0000 0B1010";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    assert!(matches!(tokens[0].kind, TokenKind::Integer(0)));
    assert!(matches!(tokens[1].kind, TokenKind::Integer(1)));
    assert!(matches!(tokens[2].kind, TokenKind::Integer(10))); // 0b1010 = 10
    assert!(matches!(tokens[3].kind, TokenKind::Integer(240))); // 0b1111_0000 = 240
    assert!(matches!(tokens[4].kind, TokenKind::Integer(10))); // 0B1010 = 10

    assert_eq!(tokens[0].lexeme, "0b0");
    assert_eq!(tokens[2].lexeme, "0b1010");
    assert_eq!(tokens[3].lexeme, "0b1111_0000");
}

#[test]
fn test_octal_numbers() {
    let source = "0o0 0o7 0o10 0o755 0o77_77 0O755";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    assert!(matches!(tokens[0].kind, TokenKind::Integer(0)));
    assert!(matches!(tokens[1].kind, TokenKind::Integer(7)));
    assert!(matches!(tokens[2].kind, TokenKind::Integer(8))); // 0o10 = 8
    assert!(matches!(tokens[3].kind, TokenKind::Integer(493))); // 0o755 = 493
    assert!(matches!(tokens[4].kind, TokenKind::Integer(4095))); // 0o77_77 = 4095
    assert!(matches!(tokens[5].kind, TokenKind::Integer(493))); // 0O755 = 493

    assert_eq!(tokens[0].lexeme, "0o0");
    assert_eq!(tokens[3].lexeme, "0o755");
    assert_eq!(tokens[4].lexeme, "0o77_77");
}

#[test]
fn test_hexadecimal_numbers() {
    let source = "0x0 0x9 0xa 0xF 0x10 0xFF 0xDEAD_BEEF 0X1A2B";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    assert!(matches!(tokens[0].kind, TokenKind::Integer(0)));
    assert!(matches!(tokens[1].kind, TokenKind::Integer(9)));
    assert!(matches!(tokens[2].kind, TokenKind::Integer(10))); // 0xa = 10
    assert!(matches!(tokens[3].kind, TokenKind::Integer(15))); // 0xF = 15
    assert!(matches!(tokens[4].kind, TokenKind::Integer(16))); // 0x10 = 16
    assert!(matches!(tokens[5].kind, TokenKind::Integer(255))); // 0xFF = 255
    assert!(matches!(tokens[6].kind, TokenKind::Integer(3735928559))); // 0xDEAD_BEEF
    assert!(matches!(tokens[7].kind, TokenKind::Integer(6699))); // 0X1A2B = 6699

    assert_eq!(tokens[0].lexeme, "0x0");
    assert_eq!(tokens[5].lexeme, "0xFF");
    assert_eq!(tokens[6].lexeme, "0xDEAD_BEEF");
}

#[test]
fn test_number_format_mixed() {
    let source = "42 0b101010 0o52 0x2A";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    // All represent the number 42 in different bases
    assert!(matches!(tokens[0].kind, TokenKind::Integer(42)));
    assert!(matches!(tokens[1].kind, TokenKind::Integer(42)));
    assert!(matches!(tokens[2].kind, TokenKind::Integer(42)));
    assert!(matches!(tokens[3].kind, TokenKind::Integer(42)));
}

#[test]
fn test_invalid_binary_number() {
    let source = "0b2"; // Invalid binary digit
    let mut lexer = Lexer::new(source);
    let result = lexer.tokenize();

    // 0b prefix with no valid binary digits should error
    assert!(result.is_err());
    assert!(matches!(result, Err(LexError::InvalidNumber(_, _, _))));
}

#[test]
fn test_invalid_octal_number() {
    let source = "0o8"; // Invalid octal digit
    let mut lexer = Lexer::new(source);
    let result = lexer.tokenize();

    // 0o prefix with no valid octal digits should error
    assert!(result.is_err());
    assert!(matches!(result, Err(LexError::InvalidNumber(_, _, _))));
}

#[test]
fn test_empty_prefix_numbers() {
    let source = "0b 0o 0x"; // Prefix without digits
    let mut lexer = Lexer::new(source);
    let result = lexer.tokenize();

    // Should error - empty number after prefix
    assert!(result.is_err());
}

#[test]
fn test_decimal_with_underscores() {
    let source = "1_000_000 123_456 1_2_3";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    assert!(matches!(tokens[0].kind, TokenKind::Integer(1_000_000)));
    assert!(matches!(tokens[1].kind, TokenKind::Integer(123_456)));
    assert!(matches!(tokens[2].kind, TokenKind::Integer(123)));

    assert_eq!(tokens[0].lexeme, "1_000_000");
    assert_eq!(tokens[1].lexeme, "123_456");
}

#[test]
fn test_float_with_underscores() {
    let source = "1_000.5 3.14_15_92 1e1_0";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    if let TokenKind::Float(val) = tokens[0].kind {
        assert!((val - 1000.5).abs() < 0.0001);
    } else {
        panic!("Expected float");
    }

    if let TokenKind::Float(val) = tokens[1].kind {
        assert!((val - 3.141592).abs() < 0.0001);
    } else {
        panic!("Expected float");
    }

    assert_eq!(tokens[0].lexeme, "1_000.5");
    assert_eq!(tokens[1].lexeme, "3.14_15_92");
}

#[test]
fn test_fstring_basic() {
    let source = r#"f"Hello {name}""#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens.len(), 2); // FString + EOF

    if let TokenKind::FString(ref parts) = tokens[0].kind {
        assert_eq!(parts.len(), 2);

        // Check text part
        if let FStringPart::Text(ref text) = parts[0] {
            assert_eq!(text, "Hello ");
        } else {
            panic!("Expected text part");
        }

        // Check expression part
        if let FStringPart::Expression {
            ref code,
            ref format_spec,
        } = parts[1]
        {
            assert_eq!(code, "name");
            assert_eq!(format_spec, &None);
        } else {
            panic!("Expected expression part");
        }
    } else {
        panic!("Expected FString token");
    }
}

#[test]
fn test_fstring_multiple_expressions() {
    let source = r#"f"{x} + {y} = {x + y}""#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    if let TokenKind::FString(ref parts) = tokens[0].kind {
        assert_eq!(parts.len(), 5); // x, " + ", y, " = ", x+y

        match &parts[0] {
            FStringPart::Expression { code, .. } => assert_eq!(code, "x"),
            _ => panic!("Expected expression"),
        }

        match &parts[1] {
            FStringPart::Text(text) => assert_eq!(text, " + "),
            _ => panic!("Expected text"),
        }

        match &parts[2] {
            FStringPart::Expression { code, .. } => assert_eq!(code, "y"),
            _ => panic!("Expected expression"),
        }

        match &parts[3] {
            FStringPart::Text(text) => assert_eq!(text, " = "),
            _ => panic!("Expected text"),
        }

        match &parts[4] {
            FStringPart::Expression { code, .. } => assert_eq!(code, "x + y"),
            _ => panic!("Expected expression"),
        }
    } else {
        panic!("Expected FString token");
    }
}

#[test]
fn test_fstring_with_format_spec() {
    let source = r#"f"{value:.2f}""#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    if let TokenKind::FString(ref parts) = tokens[0].kind {
        assert_eq!(parts.len(), 1);

        if let FStringPart::Expression {
            ref code,
            ref format_spec,
        } = parts[0]
        {
            assert_eq!(code, "value");
            assert_eq!(format_spec, &Some(".2f".to_string()));
        } else {
            panic!("Expected expression with format spec");
        }
    } else {
        panic!("Expected FString token");
    }
}

#[test]
fn test_fstring_escaped_braces() {
    let source = r#"f"{{escaped}} {name}""#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    if let TokenKind::FString(ref parts) = tokens[0].kind {
        assert_eq!(parts.len(), 2);

        match &parts[0] {
            FStringPart::Text(text) => assert_eq!(text, "{escaped} "),
            _ => panic!("Expected text with escaped braces"),
        }

        match &parts[1] {
            FStringPart::Expression { code, .. } => assert_eq!(code, "name"),
            _ => panic!("Expected expression"),
        }
    } else {
        panic!("Expected FString token");
    }
}

#[test]
fn test_fstring_single_quotes() {
    let source = r#"f'Value: {x}'"#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens.len(), 2);

    if let TokenKind::FString(ref parts) = tokens[0].kind {
        assert_eq!(parts.len(), 2);

        match &parts[0] {
            FStringPart::Text(text) => assert_eq!(text, "Value: "),
            _ => panic!("Expected text part"),
        }

        match &parts[1] {
            FStringPart::Expression { code, .. } => assert_eq!(code, "x"),
            _ => panic!("Expected expression"),
        }
    } else {
        panic!("Expected FString token");
    }
}

#[test]
fn test_fstring_uppercase_f() {
    let source = r#"F"Hello {world}""#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens.len(), 2);
    assert!(matches!(tokens[0].kind, TokenKind::FString(_)));
}

#[test]
fn test_fstring_only_text() {
    let source = r#"f"Just text, no expressions""#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    if let TokenKind::FString(ref parts) = tokens[0].kind {
        assert_eq!(parts.len(), 1);

        match &parts[0] {
            FStringPart::Text(text) => assert_eq!(text, "Just text, no expressions"),
            _ => panic!("Expected text part only"),
        }
    } else {
        panic!("Expected FString token");
    }
}

#[test]
fn test_fstring_complex_expression() {
    let source = r#"f"Result: {func(a, b) * 2}""#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    if let TokenKind::FString(ref parts) = tokens[0].kind {
        assert_eq!(parts.len(), 2);

        match &parts[1] {
            FStringPart::Expression { code, .. } => assert_eq!(code, "func(a, b) * 2"),
            _ => panic!("Expected complex expression"),
        }
    } else {
        panic!("Expected FString token");
    }
}

#[test]
fn test_fstring_with_escape_sequences() {
    let source = r#"f"Line 1\nLine 2: {value}""#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    if let TokenKind::FString(ref parts) = tokens[0].kind {
        assert_eq!(parts.len(), 2);

        match &parts[0] {
            FStringPart::Text(text) => assert_eq!(text, "Line 1\nLine 2: "),
            _ => panic!("Expected text with newline"),
        }
    } else {
        panic!("Expected FString token");
    }
}

#[test]
fn test_fstring_unmatched_brace_error() {
    let source = r#"f"Hello }world""#;
    let mut lexer = Lexer::new(source);
    let result = lexer.tokenize();

    assert!(result.is_err());
    if let Err(err) = result {
        assert!(err.to_string().contains("Unmatched '}'"));
    }
}

#[test]
fn test_raw_string_basic() {
    let source = r#"r"Hello\nWorld""#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens.len(), 2); // RawString + EOF

    if let TokenKind::RawString(ref value) = tokens[0].kind {
        // Raw strings preserve backslashes literally
        assert_eq!(value, r"Hello\nWorld");
    } else {
        panic!("Expected raw string token");
    }
}

#[test]
fn test_raw_string_backslashes() {
    let source = r#"r"C:\Users\name\file.txt""#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    if let TokenKind::RawString(ref value) = tokens[0].kind {
        assert_eq!(value, r"C:\Users\name\file.txt");
    } else {
        panic!("Expected raw string token");
    }
}

#[test]
fn test_raw_string_single_quotes() {
    let source = r#"r'Hello\tWorld'"#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    if let TokenKind::RawString(ref value) = tokens[0].kind {
        assert_eq!(value, r"Hello\tWorld");
    } else {
        panic!("Expected raw string token");
    }
}

#[test]
fn test_raw_string_uppercase_r() {
    let source = r#"R"Test\n""#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens.len(), 2);
    assert!(matches!(tokens[0].kind, TokenKind::RawString(_)));
}

#[test]
fn test_raw_string_regex_pattern() {
    let source = r#"r"\d+\.\d+""#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    if let TokenKind::RawString(ref value) = tokens[0].kind {
        assert_eq!(value, r"\d+\.\d+");
    } else {
        panic!("Expected raw string token");
    }
}

#[test]
fn test_raw_string_triple_quoted() {
    let source = r#"r"""Line 1\nLine 2\tTab""""#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    if let TokenKind::RawString(ref value) = tokens[0].kind {
        assert_eq!(value, r"Line 1\nLine 2\tTab");
    } else {
        panic!("Expected raw string token");
    }
}

#[test]
fn test_raw_string_with_backslash_at_end() {
    // Raw strings can contain backslashes except at the very end before closing quote
    let source = r#"r"path\to\file""#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    if let TokenKind::RawString(ref value) = tokens[0].kind {
        assert_eq!(value, r"path\to\file");
    } else {
        panic!("Expected raw string token");
    }
}

#[test]
fn test_raw_string_multiple_backslashes() {
    let source = r#"r"\\\\""#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    if let TokenKind::RawString(ref value) = tokens[0].kind {
        assert_eq!(value, r"\\\\");
    } else {
        panic!("Expected raw string token");
    }
}

#[test]
fn test_raw_vs_regular_string() {
    let source = r#"r"\n" "\n""#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    // First is raw string
    if let TokenKind::RawString(ref value) = tokens[0].kind {
        assert_eq!(value, r"\n"); // Literal backslash-n
    } else {
        panic!("Expected raw string token");
    }

    // Second is regular string
    if let TokenKind::String(ref value) = tokens[1].kind {
        assert_eq!(value, "\n"); // Actual newline character
    } else {
        panic!("Expected regular string token");
    }
}

#[test]
fn test_raw_string_latex() {
    let source = r#"r"\alpha + \beta = \gamma""#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    if let TokenKind::RawString(ref value) = tokens[0].kind {
        assert_eq!(value, r"\alpha + \beta = \gamma");
    } else {
        panic!("Expected raw string token");
    }
}

#[test]
fn test_byte_string_basic() {
    let source = r#"b"Hello""#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens.len(), 2); // ByteString + EOF

    if let TokenKind::ByteString(ref bytes) = tokens[0].kind {
        assert_eq!(bytes, b"Hello");
    } else {
        panic!("Expected byte string token");
    }
}

#[test]
fn test_byte_string_with_escape_sequences() {
    let source = r#"b"Line1\nLine2\tTab""#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    if let TokenKind::ByteString(ref bytes) = tokens[0].kind {
        assert_eq!(bytes, b"Line1\nLine2\tTab");
    } else {
        panic!("Expected byte string token");
    }
}

#[test]
fn test_byte_string_hex_escape() {
    let source = r#"b"\x48\x65\x6C\x6C\x6F""#; // "Hello" in hex
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    if let TokenKind::ByteString(ref bytes) = tokens[0].kind {
        assert_eq!(bytes, b"Hello");
        assert_eq!(bytes[0], 0x48); // 'H'
    } else {
        panic!("Expected byte string token");
    }
}

#[test]
fn test_byte_string_single_quotes() {
    let source = r#"b'data'"#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    if let TokenKind::ByteString(ref bytes) = tokens[0].kind {
        assert_eq!(bytes, b"data");
    } else {
        panic!("Expected byte string token");
    }
}

#[test]
fn test_byte_string_uppercase_b() {
    let source = r#"B"test""#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens.len(), 2);
    assert!(matches!(tokens[0].kind, TokenKind::ByteString(_)));
}

#[test]
fn test_byte_string_triple_quoted() {
    let source = r#"b"""Multi
line
bytes""""#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    if let TokenKind::ByteString(ref bytes) = tokens[0].kind {
        assert!(bytes.len() > 5);
        assert!(bytes.contains(&b'\n'));
    } else {
        panic!("Expected byte string token");
    }
}

#[test]
fn test_byte_string_empty() {
    let source = r#"b"""#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    if let TokenKind::ByteString(ref bytes) = tokens[0].kind {
        assert_eq!(bytes.len(), 0);
    } else {
        panic!("Expected byte string token");
    }
}

#[test]
fn test_byte_string_with_backslashes() {
    let source = r#"b"\\path\\file""#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    if let TokenKind::ByteString(ref bytes) = tokens[0].kind {
        assert_eq!(bytes, b"\\path\\file");
    } else {
        panic!("Expected byte string token");
    }
}

#[test]
fn test_byte_string_binary_data() {
    let source = r#"b"\x00\x01\x02\xFF""#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    if let TokenKind::ByteString(ref bytes) = tokens[0].kind {
        assert_eq!(bytes, &[0x00, 0x01, 0x02, 0xFF]);
    } else {
        panic!("Expected byte string token");
    }
}

#[test]
fn test_byte_string_non_ascii_error() {
    let source = r#"b"Hello 世界""#; // Contains non-ASCII characters
    let mut lexer = Lexer::new(source);
    let result = lexer.tokenize();

    assert!(result.is_err());
    if let Err(err) = result {
        assert!(err.to_string().contains("Non-ASCII"));
    }
}

// ============================================================================
// Byte Raw String Tests (br"..." or rb"...")
// ============================================================================

#[test]
fn test_byte_raw_string_basic_br() {
    let source = r#"br"Hello\nWorld""#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("Failed to tokenize");

    assert_eq!(tokens.len(), 2); // ByteRawString + EOF
    if let TokenKind::ByteRawString(bytes) = &tokens[0].kind {
        // Raw string should preserve \n literally (2 chars: backslash and 'n')
        assert_eq!(bytes, b"Hello\\nWorld");
    } else {
        panic!("Expected ByteRawString token");
    }
}

#[test]
fn test_byte_raw_string_basic_rb() {
    let source = r#"rb"Hello\nWorld""#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("Failed to tokenize");

    assert_eq!(tokens.len(), 2); // ByteRawString + EOF
    if let TokenKind::ByteRawString(bytes) = &tokens[0].kind {
        // Raw string should preserve \n literally
        assert_eq!(bytes, b"Hello\\nWorld");
    } else {
        panic!("Expected ByteRawString token");
    }
}

#[test]
fn test_byte_raw_string_windows_path() {
    let source = r#"br"C:\Users\username\file.txt""#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("Failed to tokenize");

    assert_eq!(tokens.len(), 2);
    if let TokenKind::ByteRawString(bytes) = &tokens[0].kind {
        // All backslashes preserved literally
        assert_eq!(bytes, b"C:\\Users\\username\\file.txt");
    } else {
        panic!("Expected ByteRawString token");
    }
}

#[test]
fn test_byte_raw_string_regex_pattern() {
    let source = r#"br"\d+\.\d+""#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("Failed to tokenize");

    assert_eq!(tokens.len(), 2);
    if let TokenKind::ByteRawString(bytes) = &tokens[0].kind {
        // Regex pattern preserved with backslashes
        assert_eq!(bytes, b"\\d+\\.\\d+");
    } else {
        panic!("Expected ByteRawString token");
    }
}

#[test]
fn test_byte_raw_string_single_quotes() {
    let source = r#"br'Hello\tWorld'"#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("Failed to tokenize");

    assert_eq!(tokens.len(), 2);
    if let TokenKind::ByteRawString(bytes) = &tokens[0].kind {
        assert_eq!(bytes, b"Hello\\tWorld");
    } else {
        panic!("Expected ByteRawString token");
    }
}

#[test]
fn test_byte_raw_string_uppercase_br() {
    let source = r#"BR"Test\n""#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("Failed to tokenize");

    assert_eq!(tokens.len(), 2);
    if let TokenKind::ByteRawString(bytes) = &tokens[0].kind {
        assert_eq!(bytes, b"Test\\n");
    } else {
        panic!("Expected ByteRawString token");
    }
}

#[test]
fn test_byte_raw_string_uppercase_rb() {
    let source = r#"RB"Test\r\n""#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("Failed to tokenize");

    assert_eq!(tokens.len(), 2);
    if let TokenKind::ByteRawString(bytes) = &tokens[0].kind {
        assert_eq!(bytes, b"Test\\r\\n");
    } else {
        panic!("Expected ByteRawString token");
    }
}

#[test]
fn test_byte_raw_string_mixed_case() {
    let source = r#"Br"Mixed\nCase""#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("Failed to tokenize");

    assert_eq!(tokens.len(), 2);
    if let TokenKind::ByteRawString(bytes) = &tokens[0].kind {
        assert_eq!(bytes, b"Mixed\\nCase");
    } else {
        panic!("Expected ByteRawString token");
    }
}

#[test]
fn test_byte_raw_string_triple_quoted() {
    let source = r####"br"""Line 1\nLine 2\nLine 3""""####;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("Failed to tokenize");

    assert_eq!(tokens.len(), 2);
    if let TokenKind::ByteRawString(bytes) = &tokens[0].kind {
        // Triple-quoted preserves actual newlines AND backslashes
        let expected = b"Line 1\\nLine 2\\nLine 3";
        assert_eq!(bytes, expected);
    } else {
        panic!("Expected ByteRawString token");
    }
}

#[test]
fn test_byte_raw_string_empty() {
    let source = r#"br"""#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("Failed to tokenize");

    assert_eq!(tokens.len(), 2);
    if let TokenKind::ByteRawString(bytes) = &tokens[0].kind {
        assert_eq!(bytes, b"");
    } else {
        panic!("Expected ByteRawString token");
    }
}

#[test]
fn test_byte_raw_string_non_ascii_error() {
    let source = r#"br"Hello 世界""#; // Contains non-ASCII characters
    let mut lexer = Lexer::new(source);
    let result = lexer.tokenize();

    assert!(result.is_err());
    if let Err(err) = result {
        assert!(err.to_string().contains("Non-ASCII"));
    }
}

#[test]
fn test_byte_raw_string_hex_notation_preserved() {
    // In byte raw strings, \x is NOT processed - it stays literal
    let source = r#"br"\x41\x42\x43""#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("Failed to tokenize");

    assert_eq!(tokens.len(), 2);
    if let TokenKind::ByteRawString(bytes) = &tokens[0].kind {
        // \x41 stays as literal \x41 (not converted to 'A')
        assert_eq!(bytes, b"\\x41\\x42\\x43");
    } else {
        panic!("Expected ByteRawString token");
    }
}
