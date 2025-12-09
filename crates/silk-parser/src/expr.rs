/// Expression parsing with operator precedence

use silk_ast::{Expression, ExpressionKind, BinaryOperator, UnaryOperator, CompareOperator, LogicalOperator};
use silk_lexer::TokenKind;
use crate::{Parser, ParseResult, ParseError};

/// Operator precedence levels (higher = tighter binding)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Precedence {
    None = 0,
    Walrus = 1,       // := (named expression)
    Or = 2,           // or
    And = 3,          // and
    Not = 4,          // not (prefix)
    Comparison = 5,   // ==, !=, <, >, <=, >=, in, not in, is, is not
    BitwiseOr = 6,    // |
    BitwiseXor = 7,   // ^
    BitwiseAnd = 8,   // &
    Shift = 9,        // <<, >>
    Addition = 10,    // +, -
    Multiplication = 11, // *, /, //, %, @
    Unary = 12,       // +, -, ~
    Power = 13,       // **
    Primary = 14,     // ., [], ()
}

impl Parser {
    /// Parse an expression
    pub(crate) fn parse_expression(&mut self) -> ParseResult<Expression> {
        self.parse_precedence(Precedence::None)
    }
    
    /// Parse expression with operator precedence climbing
    fn parse_precedence(&mut self, min_precedence: Precedence) -> ParseResult<Expression> {
        let mut left = self.parse_primary()?;
        
        while !self.is_at_end() {
            let precedence = self.get_precedence();
            
            // Stop if precedence is None (not an operator) or less than minimum
            if precedence == Precedence::None || precedence < min_precedence {
                break;
            }
            
            left = self.parse_infix(left, precedence)?;
        }
        
        Ok(left)
    }
    
    /// Parse a primary expression (literals, identifiers, prefix operators, grouping)
    fn parse_primary(&mut self) -> ParseResult<Expression> {
        let start = self.current_token().span.clone();
        
        let kind = match &self.current_token().kind {
            // Literals
            TokenKind::Integer(value) => {
                let value = *value;
                self.advance();
                ExpressionKind::Integer(value)
            }
            TokenKind::Float(value) => {
                let value = *value;
                self.advance();
                ExpressionKind::Float(value)
            }
            TokenKind::String(value) => {
                let value = value.clone();
                self.advance();
                ExpressionKind::String(value)
            }
            TokenKind::RawString(value) => {
                let value = value.clone();
                self.advance();
                ExpressionKind::RawString(value)
            }
            TokenKind::ByteString(bytes) => {
                let bytes = bytes.clone();
                self.advance();
                ExpressionKind::ByteString(bytes)
            }
            TokenKind::ByteRawString(bytes) => {
                let bytes = bytes.clone();
                self.advance();
                ExpressionKind::ByteRawString(bytes)
            }
            TokenKind::FString(parts) => {
                let parts = parts.clone();
                self.advance();
                ExpressionKind::FString { parts }
            }
            TokenKind::True => {
                self.advance();
                ExpressionKind::Boolean(true)
            }
            TokenKind::False => {
                self.advance();
                ExpressionKind::Boolean(false)
            }
            TokenKind::None => {
                self.advance();
                ExpressionKind::None
            }
            TokenKind::NotImplemented => {
                self.advance();
                ExpressionKind::NotImplemented
            }
            TokenKind::Ellipsis => {
                self.advance();
                ExpressionKind::Ellipsis
            }
            
            // Identifier
            TokenKind::Identifier => {
                let name = self.current_token().lexeme.clone();
                self.advance();
                ExpressionKind::Identifier(name)
            }
            
            // Tuple literal or grouping
            TokenKind::LeftParen => {
                self.advance(); // consume '('
                
                // Empty tuple: ()
                if self.check(TokenKind::RightParen) {
                    self.advance();
                    ExpressionKind::Tuple { elements: Vec::new() }
                } else {
                    // Parse first expression
                    let first_expr = self.parse_expression()?;
                    
                    // Check for comma (makes it a tuple)
                    if self.check(TokenKind::Comma) {
                        self.advance(); // consume comma
                        
                        let mut elements = vec![first_expr];
                        
                        // Parse remaining elements
                        while !self.check(TokenKind::RightParen) && !self.is_at_end() {
                            elements.push(self.parse_expression()?);
                            
                            if !self.check(TokenKind::RightParen) {
                                self.expect(TokenKind::Comma, "Expected ',' or ')' in tuple")?;
                            }
                        }
                        
                        self.expect(TokenKind::RightParen, "Expected ')' after tuple elements")?;
                        ExpressionKind::Tuple { elements }
                    } else {
                        // No comma, just a parenthesized expression
                        self.expect(TokenKind::RightParen, "Expected ')' after expression")?;
                        return Ok(first_expr);
                    }
                }
            }
            
            // Unary operators
            TokenKind::Plus | TokenKind::Minus | TokenKind::Tilde => {
                let op = match self.current_token().kind {
                    TokenKind::Plus => UnaryOperator::UAdd,
                    TokenKind::Minus => UnaryOperator::USub,
                    TokenKind::Tilde => UnaryOperator::Invert,
                    _ => unreachable!(),
                };
                self.advance();
                let operand = self.parse_precedence(Precedence::Unary)?;
                ExpressionKind::UnaryOp {
                    op,
                    operand: Box::new(operand),
                }
            }
            
            // Not operator
            TokenKind::Not => {
                self.advance();
                let operand = self.parse_precedence(Precedence::Not)?;
                ExpressionKind::UnaryOp {
                    op: UnaryOperator::Not,
                    operand: Box::new(operand),
                }
            }
            
            // List literal (TODO: comprehensions)
            TokenKind::LeftBracket => {
                self.advance(); // consume '['
                
                // Check for empty list
                if self.check(TokenKind::RightBracket) {
                    self.advance();
                    return Ok(Expression::new(
                        ExpressionKind::List { elements: vec![] },
                        silk_lexer::Span::new(start.start, self.current_token().span.end, start.line, start.column)
                    ));
                }
                
                // Parse first element
                let first_element = self.parse_expression()?;
                
                // DETECTION POINT: Check if this is a list comprehension
                if self.check(TokenKind::For) {
                    return self.parse_list_comprehension(first_element, start);
                }
                
                // Regular list: continue parsing elements
                let mut elements = vec![first_element];
                
                while self.check(TokenKind::Comma) {
                    self.advance(); // consume ','
                    
                    // Check for trailing comma
                    if self.check(TokenKind::RightBracket) {
                        break;
                    }
                    
                    elements.push(self.parse_expression()?);
                }
                
                self.expect(TokenKind::RightBracket, "Expected ']' after list elements")?;
                ExpressionKind::List { elements }
            }
            
            // Dict/set literal
            TokenKind::LeftBrace => {
                self.advance(); // consume '{'
                
                // Empty dict (by default, {} is a dict, not a set)
                if self.check(TokenKind::RightBrace) {
                    self.advance();
                    return Ok(Expression::new(
                        ExpressionKind::Dict { keys: vec![], values: vec![] },
                        silk_lexer::Span::new(start.start, self.current_token().span.end, start.line, start.column)
                    ));
                }
                
                // Parse first element
                let first_expr = self.parse_expression()?;
                
                // Check if this is a dict or set
                if self.check(TokenKind::Colon) {
                    // Dict: {key: value, ...}
                    self.advance(); // consume ':'
                    let first_value = self.parse_expression()?;
                    
                    let mut keys = vec![first_expr];
                    let mut values = vec![first_value];
                    
                    // Parse remaining key-value pairs
                    while self.check(TokenKind::Comma) {
                        self.advance(); // consume ','
                        
                        // Allow trailing comma
                        if self.check(TokenKind::RightBrace) {
                            break;
                        }
                        
                        let key = self.parse_expression()?;
                        self.expect(TokenKind::Colon, "Expected ':' in dict literal")?;
                        let value = self.parse_expression()?;
                        
                        keys.push(key);
                        values.push(value);
                    }
                    
                    self.expect(TokenKind::RightBrace, "Expected '}' after dict elements")?;
                    ExpressionKind::Dict { keys, values }
                } else {
                    // Set: {element, ...}
                    let mut elements = vec![first_expr];
                    
                    // Parse remaining elements
                    while self.check(TokenKind::Comma) {
                        self.advance(); // consume ','
                        
                        // Allow trailing comma
                        if self.check(TokenKind::RightBrace) {
                            break;
                        }
                        
                        elements.push(self.parse_expression()?);
                    }
                    
                    self.expect(TokenKind::RightBrace, "Expected '}' after set elements")?;
                    ExpressionKind::Set { elements }
                }
            }
            
            // Lambda expression
            TokenKind::Lambda => {
                self.advance(); // consume 'lambda'
                
                let mut params = Vec::new();
                
                // Parse parameters (if any) - simple form without type annotations or defaults
                if !self.check(TokenKind::Colon) {
                    loop {
                        let param_start = self.current_token().span.clone();
                        let name = self.expect(TokenKind::Identifier, "Expected parameter name in lambda")?.lexeme;
                        
                        params.push(silk_ast::Parameter {
                            name,
                            annotation: None,  // Lambdas don't have type annotations
                            default: None,     // Lambdas don't have default values in simple form
                            span: param_start,
                        });
                        
                        if self.check(TokenKind::Comma) {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                }
                
                self.expect(TokenKind::Colon, "Expected ':' after lambda parameters")?;
                
                // Parse body (single expression)
                let body = Box::new(self.parse_expression()?);
                
                ExpressionKind::Lambda { params, body }
            }
            
            _ => {
                return Err(ParseError::InvalidExpression(
                    self.current_token().span.line,
                    self.current_token().span.column,
                ));
            }
        };
        
        // Get end position from the previous token (we've advanced past it)
        let end_pos = if self.position > 0 {
            self.tokens[self.position - 1].span.end
        } else {
            start.end
        };
        
        let span = silk_lexer::Span::new(start.start, end_pos, start.line, start.column);
        
        Ok(Expression::new(kind, span))
    }
    
    /// Parse infix operators (binary, comparison, logical, postfix)
    fn parse_infix(&mut self, left: Expression, _precedence: Precedence) -> ParseResult<Expression> {
        let start = left.span.clone();
        let op_start = self.current_token().span.clone();
        
        let kind = match self.current_token().kind {
            // Binary operators
            TokenKind::Plus => {
                self.advance();
                let right = self.parse_precedence(Precedence::Addition.succ())?;
                ExpressionKind::BinaryOp {
                    left: Box::new(left),
                    op: BinaryOperator::Add,
                    right: Box::new(right),
                }
            }
            TokenKind::Minus => {
                self.advance();
                let right = self.parse_precedence(Precedence::Addition.succ())?;
                ExpressionKind::BinaryOp {
                    left: Box::new(left),
                    op: BinaryOperator::Sub,
                    right: Box::new(right),
                }
            }
            TokenKind::Star => {
                self.advance();
                let right = self.parse_precedence(Precedence::Multiplication.succ())?;
                ExpressionKind::BinaryOp {
                    left: Box::new(left),
                    op: BinaryOperator::Mult,
                    right: Box::new(right),
                }
            }
            TokenKind::Slash => {
                self.advance();
                let right = self.parse_precedence(Precedence::Multiplication.succ())?;
                ExpressionKind::BinaryOp {
                    left: Box::new(left),
                    op: BinaryOperator::Div,
                    right: Box::new(right),
                }
            }
            TokenKind::DoubleStar => {
                self.advance();
                // Power is right-associative
                let right = self.parse_precedence(Precedence::Power)?;
                ExpressionKind::BinaryOp {
                    left: Box::new(left),
                    op: BinaryOperator::Pow,
                    right: Box::new(right),
                }
            }
            
            // Comparison operators
            TokenKind::Equal => {
                self.advance();
                let right = self.parse_precedence(Precedence::Comparison.succ())?;
                ExpressionKind::Compare {
                    left: Box::new(left),
                    ops: vec![CompareOperator::Eq],
                    comparators: vec![right],
                }
            }
            TokenKind::NotEqual => {
                self.advance();
                let right = self.parse_precedence(Precedence::Comparison.succ())?;
                ExpressionKind::Compare {
                    left: Box::new(left),
                    ops: vec![CompareOperator::NotEq],
                    comparators: vec![right],
                }
            }
            TokenKind::Less => {
                self.advance();
                let right = self.parse_precedence(Precedence::Comparison.succ())?;
                ExpressionKind::Compare {
                    left: Box::new(left),
                    ops: vec![CompareOperator::Lt],
                    comparators: vec![right],
                }
            }
            TokenKind::Greater => {
                self.advance();
                let right = self.parse_precedence(Precedence::Comparison.succ())?;
                ExpressionKind::Compare {
                    left: Box::new(left),
                    ops: vec![CompareOperator::Gt],
                    comparators: vec![right],
                }
            }
            TokenKind::LessEqual => {
                self.advance();
                let right = self.parse_precedence(Precedence::Comparison.succ())?;
                ExpressionKind::Compare {
                    left: Box::new(left),
                    ops: vec![CompareOperator::LtE],
                    comparators: vec![right],
                }
            }
            TokenKind::GreaterEqual => {
                self.advance();
                let right = self.parse_precedence(Precedence::Comparison.succ())?;
                ExpressionKind::Compare {
                    left: Box::new(left),
                    ops: vec![CompareOperator::GtE],
                    comparators: vec![right],
                }
            }
            
            // Logical operators
            TokenKind::And => {
                self.advance();
                let right = self.parse_precedence(Precedence::And.succ())?;
                ExpressionKind::LogicalOp {
                    left: Box::new(left),
                    op: LogicalOperator::And,
                    right: Box::new(right),
                }
            }
            TokenKind::Or => {
                self.advance();
                let right = self.parse_precedence(Precedence::Or.succ())?;
                ExpressionKind::LogicalOp {
                    left: Box::new(left),
                    op: LogicalOperator::Or,
                    right: Box::new(right),
                }
            }
            
            // Named expression (walrus operator :=)
            TokenKind::ColonEqual => {
                self.advance(); // consume ':='
                
                // The left side must be an identifier
                if !matches!(left.kind, ExpressionKind::Identifier(_)) {
                    return Err(ParseError::InvalidSyntax(
                        "Left side of := must be a simple identifier".to_string(),
                        left.span.line,
                        left.span.column,
                    ));
                }
                
                // Parse the value (right-associative, parse at Walrus precedence)
                let value = Box::new(self.parse_precedence(Precedence::Walrus)?);
                
                ExpressionKind::NamedExpr {
                    target: Box::new(left),
                    value,
                }
            }
            
            // Ternary/conditional expression: body if test else orelse
            TokenKind::If => {
                self.advance(); // consume 'if'
                
                // Parse the test condition
                let test = Box::new(self.parse_precedence(Precedence::Or.succ())?);
                
                // Expect 'else'
                self.expect(TokenKind::Else, "Expected 'else' in conditional expression")?;
                
                // Parse the else value (at same precedence level to allow chaining)
                let orelse = Box::new(self.parse_precedence(Precedence::None)?);
                
                ExpressionKind::IfExp {
                    test,
                    body: Box::new(left),  // The left side is the 'if true' value
                    orelse,
                }
            }
            
            // Postfix operators
            TokenKind::LeftParen => {
                // Function call
                self.parse_call(left)?
            }
            TokenKind::LeftBracket => {
                // Subscript
                self.parse_subscript(left)?
            }
            TokenKind::Dot => {
                // Attribute access
                self.parse_attribute(left)?
            }
            
            _ => return Ok(left),
        };
        
        // Get end position from the previous token (we've moved past it)
        let end_pos = if self.position > 0 {
            self.tokens[self.position - 1].span.end
        } else {
            op_start.end
        };
        
        let span = silk_lexer::Span::new(start.start, end_pos, start.line, start.column);
        
        Ok(Expression::new(kind, span))
    }
    
    /// Get precedence of current token
    fn get_precedence(&self) -> Precedence {
        match self.current_token().kind {
            TokenKind::ColonEqual => Precedence::Walrus,
            TokenKind::If => Precedence::Or,  // Ternary is at Or precedence level
            TokenKind::Or => Precedence::Or,
            TokenKind::And => Precedence::And,
            TokenKind::Equal | TokenKind::NotEqual | TokenKind::Less | TokenKind::Greater |
            TokenKind::LessEqual | TokenKind::GreaterEqual | TokenKind::In | TokenKind::Is => {
                Precedence::Comparison
            }
            TokenKind::Pipe => Precedence::BitwiseOr,
            TokenKind::Caret => Precedence::BitwiseXor,
            TokenKind::Ampersand => Precedence::BitwiseAnd,
            TokenKind::LeftShift | TokenKind::RightShift => Precedence::Shift,
            TokenKind::Plus | TokenKind::Minus => Precedence::Addition,
            TokenKind::Star | TokenKind::Slash | TokenKind::DoubleSlash |
            TokenKind::Percent => Precedence::Multiplication,
            TokenKind::DoubleStar => Precedence::Power,
            TokenKind::LeftParen | TokenKind::LeftBracket | TokenKind::Dot => Precedence::Primary,
            _ => Precedence::None,
        }
    }
    
    fn parse_call(&mut self, func: Expression) -> ParseResult<ExpressionKind> {
        self.advance(); // consume '('
        
        let mut args = Vec::new();
        let mut keywords = Vec::new();
        let mut seen_keyword = false;
        
        while !self.check(TokenKind::RightParen) && !self.is_at_end() {
            let arg_start = self.current_token().span.clone();
            
            // Check for **kwargs
            if self.check(TokenKind::DoubleStar) {
                self.advance(); // consume '**'
                let value = self.parse_expression()?;
                keywords.push(silk_ast::CallKeyword {
                    arg: None, // None means **kwargs
                    value,
                    span: arg_start,
                });
                seen_keyword = true;
            }
            // Check if this is a keyword argument (identifier followed by '=', but not '==')
            else if self.check(TokenKind::Identifier) {
                // Look ahead to see if there's an '=' after the identifier
                if let Some(next_tok) = self.peek_token(1) {
                    if matches!(next_tok.kind, TokenKind::Assign) {
                        // This is a keyword argument: name=value
                        let name = self.current_token().lexeme.clone();
                        self.advance(); // consume identifier
                        self.advance(); // consume '='
                        let value = self.parse_expression()?;
                        keywords.push(silk_ast::CallKeyword {
                            arg: Some(name),
                            value,
                            span: arg_start,
                        });
                        seen_keyword = true;
                    } else {
                        // Regular positional argument that starts with identifier
                        if seen_keyword {
                            return Err(ParseError::InvalidSyntax(
                                "Positional argument cannot follow keyword argument".to_string(),
                                self.current_token().span.line,
                                self.current_token().span.column,
                            ));
                        }
                        args.push(self.parse_expression()?);
                    }
                } else {
                    // No next token, treat as positional
                    if seen_keyword {
                        return Err(ParseError::InvalidSyntax(
                            "Positional argument cannot follow keyword argument".to_string(),
                            self.current_token().span.line,
                            self.current_token().span.column,
                        ));
                    }
                    args.push(self.parse_expression()?);
                }
            }
            // Regular positional argument
            else {
                if seen_keyword {
                    return Err(ParseError::InvalidSyntax(
                        "Positional argument cannot follow keyword argument".to_string(),
                        self.current_token().span.line,
                        self.current_token().span.column,
                    ));
                }
                args.push(self.parse_expression()?);
            }
            
            if !self.check(TokenKind::RightParen) {
                self.expect(TokenKind::Comma, "Expected ',' or ')' in function call")?;
            }
        }
        
        self.expect(TokenKind::RightParen, "Expected ')' after function arguments")?;
        
        Ok(ExpressionKind::Call {
            func: Box::new(func),
            args,
            keywords,
        })
    }
    
    fn parse_subscript(&mut self, value: Expression) -> ParseResult<ExpressionKind> {
        self.advance(); // consume '['
        
        // Check if this is a slice by looking for colons
        // Slices can be: [start:stop:step], [start:stop], [:stop], [start:], [::step], etc.
        
        // Parse first component (could be start of slice or just an index)
        let first = if self.check(TokenKind::Colon) {
            None // Empty start: [:stop]
        } else {
            Some(Box::new(self.parse_expression()?))
        };
        
        // Check for colon to determine if it's a slice
        if self.check(TokenKind::Colon) {
            self.advance(); // consume first ':'
            
            // Parse stop (optional)
            let stop = if self.check(TokenKind::Colon) || self.check(TokenKind::RightBracket) {
                None // Empty stop: [start:] or [start::step]
            } else {
                Some(Box::new(self.parse_expression()?))
            };
            
            // Parse step (optional, requires second colon)
            let step = if self.check(TokenKind::Colon) {
                self.advance(); // consume second ':'
                if self.check(TokenKind::RightBracket) {
                    None // Empty step: [start:stop:]
                } else {
                    Some(Box::new(self.parse_expression()?))
                }
            } else {
                None
            };
            
            self.expect(TokenKind::RightBracket, "Expected ']' after slice")?;
            
            // Create a Slice expression as the index
            let start = self.current_token().span.clone();
            let slice_expr = Expression::new(
                ExpressionKind::Slice {
                    lower: first,
                    upper: stop,
                    step,
                },
                start,
            );
            
            Ok(ExpressionKind::Subscript {
                value: Box::new(value),
                index: Box::new(slice_expr),
            })
        } else {
            // Not a slice, just a regular subscript
            self.expect(TokenKind::RightBracket, "Expected ']' after subscript")?;
            
            Ok(ExpressionKind::Subscript {
                value: Box::new(value),
                index: first.unwrap(), // Safe because we parsed it above
            })
        }
    }
    
    fn parse_attribute(&mut self, value: Expression) -> ParseResult<ExpressionKind> {
        self.advance(); // consume '.'
        
        let attr = self.expect(TokenKind::Identifier, "Expected attribute name after '.'")?;
        
        Ok(ExpressionKind::Attribute {
            value: Box::new(value),
            attr: attr.lexeme,
        })
    }
    
    /// Parse list comprehension: [element for target in iter]
    fn parse_list_comprehension(&mut self, element: Expression, start: silk_lexer::Span) -> ParseResult<Expression> {
        // Parse the generator clauses
        let generators = self.parse_comprehension_generators()?;
        
        // Expect closing bracket
        self.expect(TokenKind::RightBracket, "Expected ']' after list comprehension")?;
        
        let end = self.current_token().span.clone();
        Ok(Expression::new(
            ExpressionKind::ListComp {
                element: Box::new(element),
                generators,
            },
            silk_lexer::Span::new(start.start, end.end, start.line, start.column)
        ))
    }
    
    /// Parse comprehension generators: for target in iter [if cond]*
    fn parse_comprehension_generators(&mut self) -> ParseResult<Vec<silk_ast::Comprehension>> {
        let mut generators = Vec::new();
        
        // For now: only parse ONE 'for' clause
        // Step 4: support filters (if clauses)
        // Multiple 'for' loops will be added in step 6
        
        // Expect 'for'
        self.expect(TokenKind::For, "Expected 'for' in comprehension")?;
        
        // Parse target (loop variable) - use Primary to get just the identifier/tuple
        let target_expr = self.parse_precedence(Precedence::Primary)?;
        let target = self.expr_to_pattern(target_expr)?;
        
        // Expect 'in'
        self.expect(TokenKind::In, "Expected 'in' after comprehension target")?;
        
        // Parse iterator - use Comparison precedence to stop before 'if' or ']'
        let iter = self.parse_precedence(Precedence::Comparison)?;
        
        // Step 4: Parse optional 'if' filters
        let mut ifs = Vec::new();
        while self.check(TokenKind::If) {
            self.advance(); // consume 'if'
            
            // Parse the filter condition
            // Use And precedence (one level below Or) to stop BEFORE ternary 'if'
            // This allows: x > 0, x and y, etc. but stops at ternary and next filter 'if'
            let filter = self.parse_precedence(Precedence::And)?;
            ifs.push(filter);
        }
        
        generators.push(silk_ast::Comprehension {
            target,
            iter,
            ifs,
            is_async: false,
        });
        
        Ok(generators)
    }
}

impl Precedence {
    /// Get next higher precedence level (for left-associative operators)
    fn succ(self) -> Self {
        match self as u8 {
            x if x < Precedence::Primary as u8 => unsafe { std::mem::transmute(x + 1) },
            _ => self,
        }
    }
}
