/// Statement parsing

use silk_ast::{Statement, StatementKind};
use silk_lexer::TokenKind;
use crate::{Parser, ParseResult};

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
        todo!("Implement if statement parsing")
    }
    
    fn parse_while_statement(&mut self) -> ParseResult<StatementKind> {
        todo!("Implement while statement parsing")
    }
    
    fn parse_for_statement(&mut self) -> ParseResult<StatementKind> {
        todo!("Implement for statement parsing")
    }
    
    fn parse_function_def(&mut self) -> ParseResult<StatementKind> {
        todo!("Implement function definition parsing")
    }
    
    fn parse_class_def(&mut self) -> ParseResult<StatementKind> {
        todo!("Implement class definition parsing")
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
        todo!("Implement import statement parsing")
    }
    
    fn parse_from_import_statement(&mut self) -> ParseResult<StatementKind> {
        todo!("Implement from import statement parsing")
    }
    
    fn parse_global_statement(&mut self) -> ParseResult<StatementKind> {
        todo!("Implement global statement parsing")
    }
    
    fn parse_nonlocal_statement(&mut self) -> ParseResult<StatementKind> {
        todo!("Implement nonlocal statement parsing")
    }
    
    fn parse_assert_statement(&mut self) -> ParseResult<StatementKind> {
        todo!("Implement assert statement parsing")
    }
    
    fn parse_raise_statement(&mut self) -> ParseResult<StatementKind> {
        todo!("Implement raise statement parsing")
    }
    
    fn parse_del_statement(&mut self) -> ParseResult<StatementKind> {
        todo!("Implement del statement parsing")
    }
    
    fn parse_with_statement(&mut self) -> ParseResult<StatementKind> {
        todo!("Implement with statement parsing")
    }
    
    fn parse_try_statement(&mut self) -> ParseResult<StatementKind> {
        todo!("Implement try statement parsing")
    }
    
    fn parse_match_statement(&mut self) -> ParseResult<StatementKind> {
        todo!("Implement match statement parsing")
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
}
