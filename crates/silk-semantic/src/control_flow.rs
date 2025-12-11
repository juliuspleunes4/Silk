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
    /// Whether the current code is reachable
    is_reachable: bool,
    /// Whether we've already reported unreachable code in this block
    unreachable_reported: bool,
}

impl ControlFlowAnalyzer {
    /// Create a new control flow analyzer
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
            current_function_returns: false,
            in_loop: false,
            is_reachable: true,
            unreachable_reported: false,
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

    // ========== HELPER METHODS ==========

    /// Analyze a block of statements
    /// Returns true if the block always terminates (return/break/continue/raise on all paths)
    fn analyze_block(&mut self, statements: &[Statement]) -> bool {
        for statement in statements {
            self.analyze_statement(statement);
        }
        // Block terminates if we ended unreachable
        !self.is_reachable
    }

    // ========== STATEMENT ANALYSIS ==========

    /// Analyze a single statement
    fn analyze_statement(&mut self, stmt: &Statement) {
        // Check if this statement is unreachable
        if !self.is_reachable {
            // Only report the first unreachable statement in a block
            if !self.unreachable_reported {
                let statement_type = match &stmt.kind {
                    StatementKind::Return { .. } => "return",
                    StatementKind::Break { .. } => "break",
                    StatementKind::Continue { .. } => "continue",
                    StatementKind::Raise { .. } => "raise",
                    StatementKind::Pass => "pass",
                    StatementKind::Expr { .. } => "expression",
                    StatementKind::Assign { .. } => "assignment",
                    StatementKind::AnnAssign { .. } => "annotated assignment",
                    StatementKind::AugAssign { .. } => "augmented assignment",
                    StatementKind::If { .. } => "if statement",
                    StatementKind::While { .. } => "while loop",
                    StatementKind::For { .. } => "for loop",
                    StatementKind::FunctionDef { .. } => "function definition",
                    StatementKind::ClassDef { .. } => "class definition",
                    StatementKind::Try { .. } => "try statement",
                    StatementKind::With { .. } => "with statement",
                    StatementKind::Match { .. } => "match statement",
                    StatementKind::Assert { .. } => "assert",
                    StatementKind::Delete { .. } => "delete",
                    StatementKind::Import { .. } | StatementKind::ImportFrom { .. } => "import",
                    StatementKind::Global { .. } | StatementKind::Nonlocal { .. } => "declaration",
                };

                self.errors.push(SemanticError::UnreachableCode {
                    statement_type: statement_type.to_string(),
                    line: stmt.span.line,
                    column: stmt.span.column,
                    span: stmt.span,
                });

                self.unreachable_reported = true;
            }

            // Don't analyze unreachable code further - this prevents cascading errors
            return;
        }

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
                let previous_reachable = self.is_reachable;
                let previous_unreachable_reported = self.unreachable_reported;
                
                self.current_function_returns = false;
                self.in_loop = false;
                self.is_reachable = true; // Function body starts reachable
                self.unreachable_reported = false; // Reset for new scope

                // Analyze function body
                for stmt in body {
                    self.analyze_statement(stmt);
                }

                // TODO: Check if function returns on all paths

                self.current_function_returns = previous_in_function;
                self.in_loop = previous_in_loop;
                self.is_reachable = previous_reachable; // Restore reachability
                self.unreachable_reported = previous_unreachable_reported;
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
                let previous_reachable = self.is_reachable;
                let previous_unreachable_reported = self.unreachable_reported;
                
                // Analyze if body
                self.is_reachable = previous_reachable;
                self.unreachable_reported = false; // Reset for if block
                for stmt in body {
                    self.analyze_statement(stmt);
                }
                let if_reachable = self.is_reachable;

                // Analyze else body (orelse is Vec, not Option)
                // If orelse is empty, it's implicitly reachable (no code to execute)
                let else_reachable = if orelse.is_empty() {
                    previous_reachable
                } else {
                    self.is_reachable = previous_reachable;
                    self.unreachable_reported = false; // Reset for else block
                    for stmt in orelse {
                        self.analyze_statement(stmt);
                    }
                    self.is_reachable
                };
                
                // Code after if is reachable if either branch is reachable
                self.is_reachable = if_reachable || else_reachable;
                self.unreachable_reported = previous_unreachable_reported; // Restore
            }

            StatementKind::While {
                test: _,
                body,
                orelse,
            } => {
                let previous_in_loop = self.in_loop;
                let previous_reachable = self.is_reachable;
                
                self.in_loop = true;
                self.is_reachable = true; // Loop body starts reachable

                // Analyze while body
                for stmt in body {
                    self.analyze_statement(stmt);
                }

                self.in_loop = previous_in_loop;
                self.is_reachable = previous_reachable; // Code after loop is reachable

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
                let previous_reachable = self.is_reachable;
                
                self.in_loop = true;
                self.is_reachable = true; // Loop body starts reachable

                // TODO: Track loop variable initialization

                // Analyze for body
                for stmt in body {
                    self.analyze_statement(stmt);
                }

                self.in_loop = previous_in_loop;
                self.is_reachable = previous_reachable; // Code after loop is reachable

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
                let previous_reachable = self.is_reachable;
                
                // Analyze try body
                self.is_reachable = previous_reachable;
                for stmt in body {
                    self.analyze_statement(stmt);
                }
                let try_reachable = self.is_reachable;

                // Analyze exception handlers
                let mut handlers_reachable = false;
                for handler in handlers {
                    self.is_reachable = previous_reachable; // Each handler starts fresh
                    for stmt in &handler.body {
                        self.analyze_statement(stmt);
                    }
                    handlers_reachable = handlers_reachable || self.is_reachable;
                }

                // After try/except, code is reachable if try exits normally OR any handler is reachable
                // (Exceptions can always occur, so handlers are always potentially executed)
                self.is_reachable = try_reachable || handlers_reachable || !handlers.is_empty();

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
                // Mark code after return as unreachable
                self.is_reachable = false;
                self.current_function_returns = true;
            }

            StatementKind::Break { .. } => {
                // Mark code after break as unreachable
                self.is_reachable = false;
                // TODO: Detect break outside loop
            }

            StatementKind::Continue { .. } => {
                // Mark code after continue as unreachable
                self.is_reachable = false;
                // TODO: Detect continue outside loop
            }

            StatementKind::Raise { .. } => {
                // Mark code after raise as unreachable
                self.is_reachable = false;
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

assert True
del x

import os

global g
"#;
        let program = Parser::parse(source).expect("Failed to parse");
        let mut analyzer = ControlFlowAnalyzer::new();
        
        // Should not panic and should traverse all statements
        let result = analyzer.analyze(&program);
        
        // Should succeed - no control flow errors in this code
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
