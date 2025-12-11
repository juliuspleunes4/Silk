//! Control flow analysis for Silk programs
//!
//! Performs control flow analysis to detect:
//! - Unreachable code after returns/breaks/continues
//! - Uninitialized variable usage
//! - Missing return statements in functions
//! - Infinite loops
//! - Dead code

use crate::SemanticError;
use silk_ast::{Program, Statement, StatementKind};

/// Control flow analyzer for detecting control flow errors
pub struct ControlFlowAnalyzer {
    /// Errors collected during analysis
    errors: Vec<SemanticError>,
    /// Whether the current function has a return on all paths
    current_function_returns: bool,
    /// Whether we're currently inside a loop
    in_loop: bool,
}

impl ControlFlowAnalyzer {
    /// Create a new control flow analyzer
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
            current_function_returns: false,
            in_loop: false,
        }
    }

    /// Analyze a program and return errors if any
    pub fn analyze(&mut self, program: &Program) -> Result<(), Vec<SemanticError>> {
        // Analyze all statements in the program
        for statement in &program.statements {
            self.analyze_statement(statement);
        }
        
        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(std::mem::take(&mut self.errors))
        }
    }

    /// Get a reference to the collected errors (for testing)
    pub fn errors(&self) -> &[SemanticError] {
        &self.errors
    }

    // ========== STATEMENT ANALYSIS ==========

    /// Analyze a single statement
    fn analyze_statement(&mut self, stmt: &Statement) {
        match &stmt.kind {
            // Assignment statements
            StatementKind::Assign { .. } | StatementKind::AugAssign { .. } => {
                // TODO: Track variable initialization
            }

            StatementKind::AnnAssign { .. } => {
                // TODO: Track variable initialization with type annotation
            }

            // Function definition
            StatementKind::FunctionDef { body, .. } => {
                let previous_in_function = self.current_function_returns;
                let previous_in_loop = self.in_loop;
                
                self.current_function_returns = false;
                self.in_loop = false;

                // Analyze function body
                for stmt in body {
                    self.analyze_statement(stmt);
                }

                // TODO: Check if function returns on all paths

                self.current_function_returns = previous_in_function;
                self.in_loop = previous_in_loop;
            }

            // Class definition
            StatementKind::ClassDef { body, .. } => {
                // Analyze class body
                for stmt in body {
                    self.analyze_statement(stmt);
                }
            }

            // Control flow statements
            StatementKind::If {
                test: _,
                body,
                orelse,
            } => {
                // Analyze if body
                for stmt in body {
                    self.analyze_statement(stmt);
                }

                // Analyze else body (orelse is Vec, not Option)
                for stmt in orelse {
                    self.analyze_statement(stmt);
                }
            }

            StatementKind::While {
                test: _,
                body,
                orelse,
            } => {
                let previous_in_loop = self.in_loop;
                self.in_loop = true;

                // Analyze while body
                for stmt in body {
                    self.analyze_statement(stmt);
                }

                self.in_loop = previous_in_loop;

                // Analyze else clause (orelse is Vec, not Option)
                for stmt in orelse {
                    self.analyze_statement(stmt);
                }
            }

            StatementKind::For {
                target: _,
                iter: _,
                body,
                orelse,
                is_async: _,
            } => {
                let previous_in_loop = self.in_loop;
                self.in_loop = true;

                // TODO: Track loop variable initialization

                // Analyze for body
                for stmt in body {
                    self.analyze_statement(stmt);
                }

                self.in_loop = previous_in_loop;

                // Analyze else clause (orelse is Vec, not Option)
                for stmt in orelse {
                    self.analyze_statement(stmt);
                }
            }

            // Exception handling
            StatementKind::Try {
                body,
                handlers,
                orelse,
                finalbody,
            } => {
                // Analyze try body
                for stmt in body {
                    self.analyze_statement(stmt);
                }

                // Analyze exception handlers
                for handler in handlers {
                    for stmt in &handler.body {
                        self.analyze_statement(stmt);
                    }
                }

                // Analyze else clause (orelse is Vec, not Option)
                for stmt in orelse {
                    self.analyze_statement(stmt);
                }

                // Analyze finally clause (finalbody is Vec, not Option)
                for stmt in finalbody {
                    self.analyze_statement(stmt);
                }
            }

            StatementKind::With {
                items: _,
                body,
                is_async: _,
            } => {
                // Analyze with body
                for stmt in body {
                    self.analyze_statement(stmt);
                }
            }

            // Pattern matching
            StatementKind::Match { subject: _, cases } => {
                // Analyze each case
                for case in cases {
                    for stmt in &case.body {
                        self.analyze_statement(stmt);
                    }
                }
            }

            // Simple statements - no nested structure
            StatementKind::Return { .. } => {
                // TODO: Track return statements
            }

            StatementKind::Break { .. } => {
                // TODO: Detect break outside loop
            }

            StatementKind::Continue { .. } => {
                // TODO: Detect continue outside loop
            }

            StatementKind::Raise { .. } => {
                // TODO: Track raise statements
            }

            StatementKind::Assert { .. } => {
                // No control flow impact
            }

            StatementKind::Delete { .. } => {
                // No control flow impact
            }

            StatementKind::Pass => {
                // No control flow impact
            }

            StatementKind::Import { .. } | StatementKind::ImportFrom { .. } => {
                // No control flow impact
            }

            StatementKind::Global { .. } | StatementKind::Nonlocal { .. } => {
                // No control flow impact
            }

            StatementKind::Expr { .. } => {
                // Expression statement - no control flow impact
            }
        }
    }
}

impl Default for ControlFlowAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use silk_parser::Parser;

    #[test]
    fn test_new_analyzer_has_no_errors() {
        let analyzer = ControlFlowAnalyzer::new();
        assert!(analyzer.errors().is_empty());
        assert!(!analyzer.current_function_returns);
        assert!(!analyzer.in_loop);
    }

    #[test]
    fn test_default_creates_new_analyzer() {
        let analyzer = ControlFlowAnalyzer::default();
        assert!(analyzer.errors().is_empty());
    }

    #[test]
    fn test_analyze_empty_program_succeeds() {
        let mut analyzer = ControlFlowAnalyzer::new();
        let program = Program {
            statements: Vec::new(),
            span: silk_lexer::Span::new(0, 0, 1, 1),
        };

        let result = analyzer.analyze(&program);
        assert!(result.is_ok());
        assert!(analyzer.errors().is_empty());
    }

    #[test]
    fn test_analyzer_collects_errors() {
        let mut analyzer = ControlFlowAnalyzer::new();
        
        // Manually add an error to test error collection
        analyzer.errors.push(SemanticError::UnreachableCode {
            statement_type: "return".to_string(),
            line: 1,
            column: 1,
            span: silk_lexer::Span::new(0, 1, 1, 1),
        });

        let result = analyzer.analyze(&Program {
            statements: Vec::new(),
            span: silk_lexer::Span::new(0, 0, 1, 1),
        });
        assert!(result.is_err());
        
        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 1);
        assert!(matches!(errors[0], SemanticError::UnreachableCode { .. }));
    }

    #[test]
    fn test_traverses_all_statement_types() {
        // This test verifies the analyzer can traverse a program with all statement types
        let source = r#"
x = 10
y = 20

def foo():
    return 42

class MyClass:
    pass

if True:
    pass
else:
    pass

while False:
    break
    
pass

raise Exception("error")
assert True
del x

import os

global g
"#;
        let program = Parser::parse(source).expect("Failed to parse");
        let mut analyzer = ControlFlowAnalyzer::new();
        
        // Should not panic and should traverse all statements
        let result = analyzer.analyze(&program);
        
        // At this stage, we're just testing traversal, so OK is expected
        // (we haven't implemented error detection yet)
        assert!(result.is_ok(), "Traversal should succeed: {:?}", result);
    }

    #[test]
    fn test_tracks_function_context() {
        let source = r#"
def outer():
    x = 1
    def inner():
        y = 2
    return x
"#;
        let program = Parser::parse(source).expect("Failed to parse");
        let mut analyzer = ControlFlowAnalyzer::new();
        
        let result = analyzer.analyze(&program);
        assert!(result.is_ok());
    }

    #[test]
    fn test_tracks_loop_context() {
        let source = r#"
x = 0
while x < 10:
    x = x + 1
"#;
        let program = Parser::parse(source).expect("Failed to parse");
        let mut analyzer = ControlFlowAnalyzer::new();
        
        let result = analyzer.analyze(&program);
        assert!(result.is_ok());
    }
}
