/// Statement parsing

use silk_ast::{Statement, StatementKind, FunctionParams, FunctionArg, Keyword};
use silk_lexer::TokenKind;
use crate::{Parser, ParseResult, ParseError};

impl Parser {
    /// Parse a statement
    pub(crate) fn parse_statement(&mut self) -> ParseResult<Statement> {
        let start = self.current_token().span.clone();
        
        let kind = match self.current_token().kind {
            TokenKind::If => self.parse_if_statement()?,
            TokenKind::While => self.parse_while_statement()?,
            TokenKind::For => self.parse_for_statement()?,
            TokenKind::Def => self.parse_function_def()?,
            TokenKind::Class => self.parse_class_def()?,
            TokenKind::Return => self.parse_return_statement()?,
            TokenKind::Break => {
                self.advance();
                StatementKind::Break
            }
            TokenKind::Continue => {
                self.advance();
                StatementKind::Continue
            }
            TokenKind::Pass => {
                self.advance();
                StatementKind::Pass
            }
            TokenKind::Import => self.parse_import_statement()?,
            TokenKind::From => self.parse_from_import_statement()?,
            TokenKind::Global => self.parse_global_statement()?,
            TokenKind::Nonlocal => self.parse_nonlocal_statement()?,
            TokenKind::Assert => self.parse_assert_statement()?,
            TokenKind::Raise => self.parse_raise_statement()?,
            TokenKind::Del => self.parse_del_statement()?,
            TokenKind::With => self.parse_with_statement()?,
            TokenKind::Try => self.parse_try_statement()?,
            TokenKind::Match => self.parse_match_statement()?,
            _ => {
                // Try to parse as expression or assignment
                self.parse_expr_or_assign_statement()?
            }
        };
        
        let end = self.current_token().span.clone();
        let span = silk_lexer::Span::new(start.start, end.end, start.line, start.column);
        
        Ok(Statement::new(kind, span))
    }
    
    fn parse_if_statement(&mut self) -> ParseResult<StatementKind> {
        self.advance(); // consume 'if'
        
        let test = self.parse_expression()?;
        self.expect(TokenKind::Colon, "Expected ':' after if condition")?;
        
        let body = self.parse_block()?;
        
        // Parse elif and else branches
        let mut orelse = Vec::new();
        
        while self.check(TokenKind::Elif) {
            self.advance(); // consume 'elif'
            let elif_test = self.parse_expression()?;
            self.expect(TokenKind::Colon, "Expected ':' after elif condition")?;
            let elif_body = self.parse_block()?;
            
            // Create nested if for elif
            orelse = vec![Statement::new(
                StatementKind::If {
                    test: elif_test,
                    body: elif_body,
                    orelse: Vec::new(),
                },
                self.current_token().span.clone(),
            )];
        }
        
        if self.check(TokenKind::Else) {
            self.advance(); // consume 'else'
            self.expect(TokenKind::Colon, "Expected ':' after else")?;
            orelse = self.parse_block()?;
        }
        
        Ok(StatementKind::If { test, body, orelse })
    }
    
    fn parse_while_statement(&mut self) -> ParseResult<StatementKind> {
        self.advance(); // consume 'while'
        
        let test = self.parse_expression()?;
        self.expect(TokenKind::Colon, "Expected ':' after while condition")?;
        
        let body = self.parse_block()?;
        
        // Optional else clause
        let orelse = if self.check(TokenKind::Else) {
            self.advance();
            self.expect(TokenKind::Colon, "Expected ':' after else")?;
            self.parse_block()?
        } else {
            Vec::new()
        };
        
        Ok(StatementKind::While { test, body, orelse })
    }
    
    fn parse_for_statement(&mut self) -> ParseResult<StatementKind> {
        self.advance(); // consume 'for'
        
        // Parse target (variable(s))
        let target_expr = self.parse_expression()?;
        let target = self.expr_to_pattern(target_expr)?;
        
        self.expect(TokenKind::In, "Expected 'in' in for loop")?;
        
        let iter = self.parse_expression()?;
        self.expect(TokenKind::Colon, "Expected ':' after for clause")?;
        
        let body = self.parse_block()?;
        
        // Optional else clause
        let orelse = if self.check(TokenKind::Else) {
            self.advance();
            self.expect(TokenKind::Colon, "Expected ':' after else")?;
            self.parse_block()?
        } else {
            Vec::new()
        };
        
        Ok(StatementKind::For {
            target,
            iter,
            body,
            orelse,
            is_async: false,
        })
    }
    
    fn parse_function_def(&mut self) -> ParseResult<StatementKind> {
        self.advance(); // consume 'def'
        
        let name = self.expect(TokenKind::Identifier, "Expected function name")?.lexeme;
        
        self.expect(TokenKind::LeftParen, "Expected '(' after function name")?;
        
        // Parse parameters
        let params = self.parse_function_params()?;
        
        self.expect(TokenKind::RightParen, "Expected ')' after parameters")?;
        
        // Parse return type annotation
        let returns = if self.check(TokenKind::Arrow) {
            self.advance();
            Some(self.parse_type()?)
        } else {
            None
        };
        
        self.expect(TokenKind::Colon, "Expected ':' after function signature")?;
        
        let body = self.parse_block()?;
        
        Ok(StatementKind::FunctionDef {
            name,
            params,
            body,
            decorator_list: Vec::new(), // TODO: Parse decorators
            returns,
            is_async: false,
        })
    }
    
    fn parse_class_def(&mut self) -> ParseResult<StatementKind> {
        self.advance(); // consume 'class'
        
        let name = self.expect(TokenKind::Identifier, "Expected class name")?.lexeme;
        
        // Parse base classes
        let mut bases = Vec::new();
        let mut keywords = Vec::new();
        
        if self.check(TokenKind::LeftParen) {
            self.advance();
            
            // Parse bases and keyword arguments
            while !self.check(TokenKind::RightParen) && !self.is_at_end() {
                let expr = self.parse_expression()?;
                
                // Check if this is a keyword argument (name=value)
                if self.check(TokenKind::Assign) {
                    self.advance();
                    let value = self.parse_expression()?;
                    
                    // Extract keyword name from expression
                    if let silk_ast::ExpressionKind::Identifier(arg) = expr.kind {
                        keywords.push(Keyword {
                            arg: Some(arg),
                            value,
                            span: expr.span,
                        });
                    } else {
                        keywords.push(Keyword {
                            arg: None,
                            value,
                            span: expr.span,
                        });
                    }
                } else {
                    bases.push(expr);
                }
                
                if self.check(TokenKind::Comma) {
                    self.advance();
                } else {
                    break;
                }
            }
            
            self.expect(TokenKind::RightParen, "Expected ')' after bases")?;
        }
        
        self.expect(TokenKind::Colon, "Expected ':' after class header")?;
        
        let body = self.parse_block()?;
        
        Ok(StatementKind::ClassDef {
            name,
            bases,
            keywords,
            body,
            decorator_list: Vec::new(), // TODO: Parse decorators
        })
    }
    
    fn parse_return_statement(&mut self) -> ParseResult<StatementKind> {
        self.advance(); // consume 'return'
        
        if self.check(TokenKind::Newline) || self.is_at_end() {
            Ok(StatementKind::Return { value: None })
        } else {
            let value = self.parse_expression()?;
            Ok(StatementKind::Return { value: Some(value) })
        }
    }
    
    fn parse_import_statement(&mut self) -> ParseResult<StatementKind> {
        self.advance(); // consume 'import'
        
        let mut names = Vec::new();
        loop {
            let name = self.expect(TokenKind::Identifier, "Expected module name")?.lexeme;
            
            // Handle dotted names (e.g., os.path)
            let mut full_name = name;
            while self.check(TokenKind::Dot) {
                self.advance();
                let part = self.expect(TokenKind::Identifier, "Expected identifier after '.'")?.lexeme;
                full_name.push('.');
                full_name.push_str(&part);
            }
            
            let asname = if self.check(TokenKind::As) {
                self.advance();
                Some(self.expect(TokenKind::Identifier, "Expected alias name")?.lexeme)
            } else {
                None
            };
            
            names.push(silk_ast::Alias {
                name: full_name,
                asname,
                span: self.current_token().span.clone(),
            });
            
            if self.check(TokenKind::Comma) {
                self.advance();
            } else {
                break;
            }
        }
        
        Ok(StatementKind::Import { names })
    }
    
    fn parse_from_import_statement(&mut self) -> ParseResult<StatementKind> {
        self.advance(); // consume 'from'
        
        // Count leading dots for relative imports
        let mut level = 0;
        while self.check(TokenKind::Dot) {
            self.advance();
            level += 1;
        }
        
        // Parse module name (if not pure relative import)
        let module = if matches!(self.current_token().kind, TokenKind::Identifier) {
            let name = self.current_token().lexeme.clone();
            self.advance();
            
            // Handle dotted names
            let mut full_name = name;
            while self.check(TokenKind::Dot) {
                self.advance();
                let part = self.expect(TokenKind::Identifier, "Expected identifier")?.lexeme;
                full_name.push('.');
                full_name.push_str(&part);
            }
            Some(full_name)
        } else {
            None
        };
        
        self.expect(TokenKind::Import, "Expected 'import'")?;
        
        // Parse imported names
        let mut names = Vec::new();
        
        // Handle "from x import *"
        if self.check(TokenKind::Star) {
            self.advance();
            names.push(silk_ast::Alias {
                name: "*".to_string(),
                asname: None,
                span: self.current_token().span.clone(),
            });
        } else {
            // Handle parenthesized imports: from x import (a, b, c)
            let has_parens = self.check(TokenKind::LeftParen);
            if has_parens {
                self.advance();
            }
            
            loop {
                let name = self.expect(TokenKind::Identifier, "Expected import name")?.lexeme;
                
                let asname = if self.check(TokenKind::As) {
                    self.advance();
                    Some(self.expect(TokenKind::Identifier, "Expected alias")?.lexeme)
                } else {
                    None
                };
                
                names.push(silk_ast::Alias {
                    name,
                    asname,
                    span: self.current_token().span.clone(),
                });
                
                if self.check(TokenKind::Comma) {
                    self.advance();
                } else {
                    break;
                }
            }
            
            if has_parens {
                self.expect(TokenKind::RightParen, "Expected ')'")?;
            }
        }
        
        Ok(StatementKind::ImportFrom { module, names, level })
    }
    
    fn parse_global_statement(&mut self) -> ParseResult<StatementKind> {
        self.advance(); // consume 'global'
        
        let mut names = Vec::new();
        loop {
            if let TokenKind::Identifier = self.current_token().kind {
                names.push(self.current_token().lexeme.clone());
                self.advance();
                
                if self.check(TokenKind::Comma) {
                    self.advance();
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        
        Ok(StatementKind::Global { names })
    }
    
    fn parse_nonlocal_statement(&mut self) -> ParseResult<StatementKind> {
        self.advance(); // consume 'nonlocal'
        
        let mut names = Vec::new();
        loop {
            if let TokenKind::Identifier = self.current_token().kind {
                names.push(self.current_token().lexeme.clone());
                self.advance();
                
                if self.check(TokenKind::Comma) {
                    self.advance();
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        
        Ok(StatementKind::Nonlocal { names })
    }
    
    fn parse_assert_statement(&mut self) -> ParseResult<StatementKind> {
        self.advance(); // consume 'assert'
        
        let test = self.parse_expression()?;
        
        let msg = if self.check(TokenKind::Comma) {
            self.advance();
            Some(self.parse_expression()?)
        } else {
            None
        };
        
        Ok(StatementKind::Assert { test, msg })
    }
    
    fn parse_raise_statement(&mut self) -> ParseResult<StatementKind> {
        self.advance(); // consume 'raise'
        
        if self.check(TokenKind::Newline) || self.is_at_end() {
            return Ok(StatementKind::Raise { exc: None, cause: None });
        }
        
        let exc = Some(self.parse_expression()?);
        
        let cause = if self.check(TokenKind::From) {
            self.advance();
            Some(self.parse_expression()?)
        } else {
            None
        };
        
        Ok(StatementKind::Raise { exc, cause })
    }
    
    fn parse_del_statement(&mut self) -> ParseResult<StatementKind> {
        self.advance(); // consume 'del'
        
        let mut targets = Vec::new();
        loop {
            targets.push(self.parse_expression()?);
            
            if self.check(TokenKind::Comma) {
                self.advance();
            } else {
                break;
            }
        }
        
        Ok(StatementKind::Delete { targets })
    }
    
    fn parse_with_statement(&mut self) -> ParseResult<StatementKind> {
        self.advance(); // consume 'with'
        
        let mut items = Vec::new();
        
        loop {
            let context_expr = self.parse_expression()?;
            
            let optional_vars = if self.check(TokenKind::As) {
                self.advance();
                Some(self.parse_expression()?)
            } else {
                None
            };
            
            items.push(silk_ast::WithItem {
                context_expr,
                optional_vars,
                span: self.current_token().span.clone(),
            });
            
            if self.check(TokenKind::Comma) {
                self.advance();
            } else {
                break;
            }
        }
        
        self.expect(TokenKind::Colon, "Expected ':' after with clause")?;
        
        let body = self.parse_block()?;
        
        Ok(StatementKind::With {
            items,
            body,
            is_async: false,
        })
    }
    
    fn parse_try_statement(&mut self) -> ParseResult<StatementKind> {
        self.advance(); // consume 'try'
        
        self.expect(TokenKind::Colon, "Expected ':' after try")?;
        let body = self.parse_block()?;
        
        let mut handlers = Vec::new();
        
        // Parse except clauses
        while self.check(TokenKind::Except) {
            self.advance();
            
            let handler_start = self.current_token().span.clone();
            
            // Parse exception type
            let typ = if !self.check(TokenKind::Colon) {
                Some(self.parse_expression()?)
            } else {
                None
            };
            
            // Parse "as name"
            let name = if self.check(TokenKind::As) {
                self.advance();
                Some(self.expect(TokenKind::Identifier, "Expected exception name")?.lexeme)
            } else {
                None
            };
            
            self.expect(TokenKind::Colon, "Expected ':' after except clause")?;
            let handler_body = self.parse_block()?;
            
            handlers.push(silk_ast::ExceptHandler {
                typ,
                name,
                body: handler_body,
                span: handler_start,
            });
        }
        
        // Parse else clause
        let orelse = if self.check(TokenKind::Else) {
            self.advance();
            self.expect(TokenKind::Colon, "Expected ':' after else")?;
            self.parse_block()?
        } else {
            Vec::new()
        };
        
        // Parse finally clause
        let finalbody = if self.check(TokenKind::Finally) {
            self.advance();
            self.expect(TokenKind::Colon, "Expected ':' after finally")?;
            self.parse_block()?
        } else {
            Vec::new()
        };
        
        Ok(StatementKind::Try {
            body,
            handlers,
            orelse,
            finalbody,
        })
    }
    
    fn parse_match_statement(&mut self) -> ParseResult<StatementKind> {
        self.advance(); // consume 'match'
        
        let subject = self.parse_expression()?;
        self.expect(TokenKind::Colon, "Expected ':' after match subject")?;
        
        self.skip_newlines();
        self.expect(TokenKind::Indent, "Expected indentation after match")?;
        
        let mut cases = Vec::new();
        
        // Parse case clauses
        while self.check(TokenKind::Case) {
            self.advance();
            
            let case_start = self.current_token().span.clone();
            
            // Parse pattern
            let pattern_expr = self.parse_expression()?;
            let pattern = self.expr_to_pattern(pattern_expr)?;
            
            // Parse guard
            let guard = if self.check(TokenKind::If) {
                self.advance();
                Some(self.parse_expression()?)
            } else {
                None
            };
            
            self.expect(TokenKind::Colon, "Expected ':' after case pattern")?;
            
            // Parse case body
            self.skip_newlines();
            self.expect(TokenKind::Indent, "Expected indentation in case body")?;
            
            let mut case_body = Vec::new();
            while !self.check(TokenKind::Dedent) && !self.is_at_end() {
                if self.check(TokenKind::Newline) {
                    self.advance();
                    continue;
                }
                case_body.push(self.parse_statement()?);
            }
            
            self.expect(TokenKind::Dedent, "Expected dedentation")?;
            
            cases.push(silk_ast::MatchCase {
                pattern,
                guard,
                body: case_body,
                span: case_start,
            });
        }
        
        self.expect(TokenKind::Dedent, "Expected dedentation after match cases")?;
        
        Ok(StatementKind::Match { subject, cases })
    }
    
    fn parse_expr_or_assign_statement(&mut self) -> ParseResult<StatementKind> {
        let expr = self.parse_expression()?;
        
        // Check for assignment
        if self.check(TokenKind::Assign) {
            self.advance(); // consume '='
            let value = self.parse_expression()?;
            return Ok(StatementKind::Assign {
                targets: vec![expr],
                value,
                type_annotation: None,
            });
        }
        
        // Check for augmented assignment
        if let Some(op) = self.check_aug_assign() {
            self.advance();
            let value = self.parse_expression()?;
            return Ok(StatementKind::AugAssign {
                target: expr,
                op,
                value,
            });
        }
        
        // Just an expression statement
        Ok(StatementKind::Expr(expr))
    }
    
    fn check_aug_assign(&self) -> Option<silk_ast::AugAssignOperator> {
        use silk_ast::AugAssignOperator::*;
        
        Some(match self.current_token().kind {
            TokenKind::PlusAssign => Add,
            TokenKind::MinusAssign => Sub,
            TokenKind::StarAssign => Mult,
            TokenKind::SlashAssign => Div,
            TokenKind::DoubleSlashAssign => FloorDiv,
            TokenKind::PercentAssign => Mod,
            TokenKind::DoubleStarAssign => Pow,
            TokenKind::AmpersandAssign => BitAnd,
            TokenKind::PipeAssign => BitOr,
            TokenKind::CaretAssign => BitXor,
            TokenKind::LeftShiftAssign => LShift,
            TokenKind::RightShiftAssign => RShift,
            _ => return None,
        })
    }
    
    /// Parse a block of statements (after a colon and INDENT)
    fn parse_block(&mut self) -> ParseResult<Vec<Statement>> {
        self.skip_newlines();
        
        // Expect INDENT token
        self.expect(TokenKind::Indent, "Expected indentation")?;
        
        let mut statements = Vec::new();
        
        // Parse statements until DEDENT
        while !self.check(TokenKind::Dedent) && !self.is_at_end() {
            if self.check(TokenKind::Newline) {
                self.advance();
                continue;
            }
            
            statements.push(self.parse_statement()?);
        }
        
        // Expect DEDENT token
        self.expect(TokenKind::Dedent, "Expected dedentation")?;
        
        Ok(statements)
    }
    
    /// Convert an expression to a pattern (for use in for loops, assignments, comprehensions, etc.)
    pub(crate) fn expr_to_pattern(&self, expr: silk_ast::Expression) -> ParseResult<silk_ast::Pattern> {
        use silk_ast::{ExpressionKind, Pattern, PatternKind};
        
        Ok(match expr.kind {
            ExpressionKind::Identifier(name) => {
                Pattern::new(PatternKind::Name(name), expr.span)
            }
            ExpressionKind::Tuple { elements } | ExpressionKind::List { elements } => {
                let patterns: Result<Vec<_>, _> = elements
                    .into_iter()
                    .map(|e| self.expr_to_pattern(e))
                    .collect();
                Pattern::new(PatternKind::Sequence { patterns: patterns? }, expr.span)
            }
            _ => {
                // For now, treat other expressions as capture patterns
                return Err(ParseError::InvalidPattern(
                    expr.span.line,
                    expr.span.column,
                ));
            }
        })
    }
    
    /// Parse function parameters
    fn parse_function_params(&mut self) -> ParseResult<FunctionParams> {
        let mut args = Vec::new();
        let mut vararg = None;
        let kwonlyargs = Vec::new();  // TODO: Implement keyword-only args
        let mut kwarg = None;
        
        // Parse parameters
        while !self.check(TokenKind::RightParen) && !self.is_at_end() {
            let param_start = self.current_token().span.clone();
            
            // Check for **kwargs
            if self.check(TokenKind::DoubleStar) {
                self.advance(); // consume '**'
                let name = self.expect(TokenKind::Identifier, "Expected parameter name after '**'")?.lexeme;
                
                // Parse optional type annotation
                let annotation = if self.check(TokenKind::Colon) {
                    self.advance();
                    Some(self.parse_type()?)
                } else {
                    None
                };
                
                kwarg = Some(FunctionArg {
                    name,
                    annotation,
                    default: None, // **kwargs cannot have default
                    span: param_start,
                });
                
                // **kwargs must be last parameter
                if self.check(TokenKind::Comma) {
                    self.advance();
                    if !self.check(TokenKind::RightParen) {
                        return Err(ParseError::InvalidSyntax(
                            "**kwargs must be the last parameter".to_string(),
                            self.current_token().span.line,
                            self.current_token().span.column,
                        ));
                    }
                }
                break;
            }
            // Check for *args
            else if self.check(TokenKind::Star) {
                self.advance(); // consume '*'
                
                // Check if this is just a separator (bare *)
                if self.check(TokenKind::Comma) || self.check(TokenKind::RightParen) {
                    // Bare * means keyword-only args follow (not implemented yet)
                    if self.check(TokenKind::Comma) {
                        self.advance();
                    }
                    continue;
                }
                
                let name = self.expect(TokenKind::Identifier, "Expected parameter name after '*'")?.lexeme;
                
                // Parse optional type annotation
                let annotation = if self.check(TokenKind::Colon) {
                    self.advance();
                    Some(self.parse_type()?)
                } else {
                    None
                };
                
                vararg = Some(FunctionArg {
                    name,
                    annotation,
                    default: None, // *args cannot have default
                    span: param_start,
                });
                
                if self.check(TokenKind::Comma) {
                    self.advance();
                }
            }
            // Regular parameter
            else {
                let name = self.expect(TokenKind::Identifier, "Expected parameter name")?.lexeme;
                
                // Parse type annotation
                let annotation = if self.check(TokenKind::Colon) {
                    self.advance();
                    Some(self.parse_type()?)
                } else {
                    None
                };
                
                // Parse default value
                let default = if self.check(TokenKind::Assign) {
                    self.advance();
                    Some(self.parse_expression()?)
                } else {
                    None
                };
                
                args.push(FunctionArg {
                    name,
                    annotation,
                    default,
                    span: param_start,
                });
                
                if self.check(TokenKind::Comma) {
                    self.advance();
                } else {
                    break;
                }
            }
        }
        
        Ok(FunctionParams {
            args,
            vararg,
            kwonlyargs,
            kwarg,
        })
    }
    
    /// Parse a type annotation (simplified for now)
    fn parse_type(&mut self) -> ParseResult<silk_ast::Type> {
        use silk_ast::{Type, TypeKind};
        
        let start = self.current_token().span.clone();
        
        // For now, just parse simple type names
        if let TokenKind::Identifier = self.current_token().kind {
            let name = self.current_token().lexeme.clone();
            self.advance();
            
            // Handle generic types like List[int]
            if self.check(TokenKind::LeftBracket) {
                self.advance();
                let mut args = vec![self.parse_type()?];
                
                while self.check(TokenKind::Comma) {
                    self.advance();
                    args.push(self.parse_type()?);
                }
                
                self.expect(TokenKind::RightBracket, "Expected ']'")?;
                
                let base = Box::new(Type::new(TypeKind::Name(name), start.clone()));
                return Ok(Type::new(TypeKind::Generic { base, args }, start));
            }
            
            Ok(Type::new(TypeKind::Name(name), start))
        } else {
            Err(ParseError::InvalidSyntax(
                "Expected type".to_string(),
                start.line,
                start.column,
            ))
        }
    }
}
