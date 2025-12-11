//! Control flow analysis for Silk programs
//!
//! Performs control flow analysis to detect:
//! - Unreachable code after returns/breaks/continues
//! - Uninitialized variable usage
//! - Missing return statements in functions
//! - Infinite loops
//! - Dead code

use crate::SemanticError;
use silk_ast::{Expression, ExpressionKind, Pattern, Program, Statement, StatementKind};
use silk_lexer::Span;
use std::collections::HashSet;

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
    /// Whether the current loop contains a break statement
    loop_has_break: bool,
    /// Set of variables that have been initialized in the current scope
    initialized_variables: HashSet<String>,
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
            loop_has_break: false,
            initialized_variables: HashSet::new(),
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

    /// Mark a variable as initialized
    fn mark_initialized(&mut self, name: &str) {
        self.initialized_variables.insert(name.to_string());
    }

    /// Check if a variable is initialized, report error if not
    fn check_initialized(&mut self, name: &str, span: &Span) {
        if !self.initialized_variables.contains(name) {
            self.errors.push(SemanticError::UninitializedVariable {
                name: name.to_string(),
                line: span.line,
                column: span.column,
                span: span.clone(),
            });
        }
    }

    /// Extract variable name from an expression (for assignments)
    fn extract_variable_name(expr: &Expression) -> Option<String> {
        match &expr.kind {
            ExpressionKind::Identifier(name) => Some(name.clone()),
            _ => None,
        }
    }

    /// Extract variable name from a pattern (for loops, with statements, etc.)
    fn extract_pattern_variable(pattern: &Pattern) -> Option<String> {
        match &pattern.kind {
            silk_ast::PatternKind::Name(name) => Some(name.clone()),
            _ => None, // For now, ignore complex patterns (tuples, etc.)
        }
    }

    /// Check an expression for uninitialized variable usage
    fn check_expression(&mut self, expr: &Expression) {
        match &expr.kind {
            ExpressionKind::Identifier(name) => {
                self.check_initialized(name, &expr.span);
            }
            ExpressionKind::BinaryOp { left, right, .. } => {
                self.check_expression(left);
                self.check_expression(right);
            }
            ExpressionKind::UnaryOp { operand, .. } => {
                self.check_expression(operand);
            }
            ExpressionKind::Compare {
                left, comparators, ..
            } => {
                self.check_expression(left);
                for comp in comparators {
                    self.check_expression(comp);
                }
            }
            ExpressionKind::Call { func: _, args, keywords } => {
                // Don't check func - functions are resolved separately (not variable initialization)
                // This avoids false positives for built-in functions like print, len, etc.
                for arg in args {
                    self.check_expression(arg);
                }
                for keyword in keywords {
                    self.check_expression(&keyword.value);
                }
            }
            ExpressionKind::Attribute { value, .. } => {
                self.check_expression(value);
            }
            ExpressionKind::Subscript { value, index } => {
                self.check_expression(value);
                self.check_expression(index);
            }
            ExpressionKind::List { elements } | ExpressionKind::Tuple { elements } | ExpressionKind::Set { elements } => {
                for elem in elements {
                    self.check_expression(elem);
                }
            }
            ExpressionKind::Dict { keys, values } => {
                for key in keys {
                    self.check_expression(key);
                }
                for value in values {
                    self.check_expression(value);
                }
            }
            ExpressionKind::IfExp {
                test,
                body,
                orelse,
            } => {
                self.check_expression(test);
                self.check_expression(body);
                self.check_expression(orelse);
            }
            ExpressionKind::Lambda { params, body } => {
                // Lambda parameters are initialized within the lambda body scope
                // Save current initialized variables
                let previous_initialized = self.initialized_variables.clone();
                
                // Mark lambda parameters as initialized
                for param in params {
                    self.mark_initialized(&param.name);
                }
                
                // Check lambda body with parameters marked as initialized
                self.check_expression(body);
                
                // Restore previous initialization state (lambda is an expression)
                self.initialized_variables = previous_initialized;
            }
            ExpressionKind::LogicalOp { left, right, .. } => {
                self.check_expression(left);
                self.check_expression(right);
            }
            ExpressionKind::NamedExpr { target, value } => {
                // Walrus operator: check value, then mark target as initialized
                self.check_expression(value);
                if let Some(name) = Self::extract_variable_name(target) {
                    self.mark_initialized(&name);
                }
            }
            // Literals don't need checking
            ExpressionKind::Integer(_)
            | ExpressionKind::Float(_)
            | ExpressionKind::String(_)
            | ExpressionKind::RawString(_)
            | ExpressionKind::ByteString(_)
            | ExpressionKind::ByteRawString(_)
            | ExpressionKind::FString { .. }
            | ExpressionKind::Boolean(_)
            | ExpressionKind::None
            | ExpressionKind::NotImplemented
            | ExpressionKind::Ellipsis => {}
            
            // Slice, yield, await - skip for now
            ExpressionKind::Slice { .. }
            | ExpressionKind::Yield { .. }
            | ExpressionKind::YieldFrom { .. }
            | ExpressionKind::Await { .. } => {}
            
            // Comprehensions, generators - skip for now (they have their own scope)
            ExpressionKind::ListComp { .. }
            | ExpressionKind::SetComp { .. }
            | ExpressionKind::DictComp { .. }
            | ExpressionKind::GeneratorExp { .. } => {}
        }
    }

    /// Check if a while loop condition is always true (infinite loop)
    /// Detects patterns like `while True:` or `while 1:`
    fn is_infinite_loop_condition(test: &silk_ast::Expression) -> bool {
        use silk_ast::ExpressionKind;
        
        match &test.kind {
            // while True:
            ExpressionKind::Boolean(true) => true,
            // while 1: (or any non-zero integer)
            ExpressionKind::Integer(n) if *n != 0 => true,
            // while 1.0: (or any non-zero float)
            ExpressionKind::Float(f) if *f != 0.0 => true,
            _ => false,
        }
    }

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
            StatementKind::Assign {
                targets,
                value,
                type_annotation: _,
            } => {
                // Check value expression for uninitialized variables
                self.check_expression(value);
                
                // Mark all target variables as initialized
                for target in targets {
                    if let Some(name) = Self::extract_variable_name(target) {
                        self.mark_initialized(&name);
                    }
                }
            }

            StatementKind::AnnAssign {
                target,
                value,
                annotation: _,
                ..
            } => {
                // Check value if present
                if let Some(val) = value {
                    self.check_expression(val);
                }
                
                // Mark target as initialized
                if let Some(name) = Self::extract_variable_name(target) {
                    self.mark_initialized(&name);
                }
            }

            StatementKind::AugAssign { target, op: _, value } => {
                // Augmented assignment requires variable to already exist
                // Check both target (must be initialized) and value
                self.check_expression(target);
                self.check_expression(value);
            }

            // Function definition
            StatementKind::FunctionDef { body, params, .. } => {
                let previous_in_function = self.current_function_returns;
                let previous_in_loop = self.in_loop;
                let previous_reachable = self.is_reachable;
                let previous_unreachable_reported = self.unreachable_reported;
                let previous_initialized = self.initialized_variables.clone();
                
                // Check default parameter expressions BEFORE entering function scope
                // Default expressions are evaluated in the outer scope, not the function scope
                for param in &params.args {
                    if let Some(default_expr) = &param.default {
                        self.check_expression(default_expr);
                    }
                }
                for param in &params.kwonlyargs {
                    if let Some(default_expr) = &param.default {
                        self.check_expression(default_expr);
                    }
                }
                
                self.current_function_returns = false;
                self.in_loop = false;
                self.is_reachable = true; // Function body starts reachable
                self.unreachable_reported = false; // Reset for new scope
                self.initialized_variables.clear(); // New scope - start fresh
                
                // Mark all function parameters as initialized
                for param in &params.args {
                    self.mark_initialized(&param.name);
                }
                if let Some(vararg) = &params.vararg {
                    self.mark_initialized(&vararg.name);
                }
                for param in &params.kwonlyargs {
                    self.mark_initialized(&param.name);
                }
                if let Some(kwarg) = &params.kwarg {
                    self.mark_initialized(&kwarg.name);
                }

                // Analyze function body
                for stmt in body {
                    self.analyze_statement(stmt);
                }

                // TODO: Check if function returns on all paths

                self.current_function_returns = previous_in_function;
                self.in_loop = previous_in_loop;
                self.is_reachable = previous_reachable; // Restore reachability
                self.unreachable_reported = previous_unreachable_reported;
                self.initialized_variables = previous_initialized;
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
                test,
                body,
                orelse,
            } => {
                // Check test expression first (this may include walrus operator that initializes variables)
                self.check_expression(test);
                
                let previous_reachable = self.is_reachable;
                let previous_unreachable_reported = self.unreachable_reported;
                let previous_initialized = self.initialized_variables.clone();
                
                // Analyze if body
                self.is_reachable = previous_reachable;
                self.unreachable_reported = false; // Reset for if block
                for stmt in body {
                    self.analyze_statement(stmt);
                }
                let if_reachable = self.is_reachable;
                let if_initialized = self.initialized_variables.clone();

                // Analyze else body (orelse is Vec, not Option)
                // If orelse is empty, it's implicitly reachable (no code to execute)
                let (else_reachable, else_initialized) = if orelse.is_empty() {
                    // No else clause - restore previous state
                    (previous_reachable, previous_initialized.clone())
                } else {
                    self.is_reachable = previous_reachable;
                    self.initialized_variables = previous_initialized.clone();
                    self.unreachable_reported = false; // Reset for else block
                    for stmt in orelse {
                        self.analyze_statement(stmt);
                    }
                    (self.is_reachable, self.initialized_variables.clone())
                };
                
                // Code after if is reachable if either branch is reachable
                self.is_reachable = if_reachable || else_reachable;
                self.unreachable_reported = previous_unreachable_reported; // Restore
                
                // Variable is initialized after if/else based on which branches are reachable:
                // - If both branches reachable: must be in both (intersection)
                // - If only if branch reachable: use if branch state
                // - If only else branch reachable: use else branch state
                // - If neither reachable: doesn't matter (code is unreachable)
                self.initialized_variables = if if_reachable && else_reachable {
                    // Both reachable: intersection (must be in both)
                    if_initialized.intersection(&else_initialized).cloned().collect()
                } else if if_reachable {
                    // Only if reachable
                    if_initialized
                } else {
                    // Only else reachable (or neither, but we use else state)
                    else_initialized
                };
            }

            StatementKind::While {
                test,
                body,
                orelse,
            } => {
                // Check test expression
                self.check_expression(test);
                
                let previous_in_loop = self.in_loop;
                let previous_reachable = self.is_reachable;
                let previous_loop_has_break = self.loop_has_break;
                
                self.in_loop = true;
                self.loop_has_break = false; // Reset for this loop
                self.is_reachable = true; // Loop body starts reachable

                // Analyze while body
                for stmt in body {
                    self.analyze_statement(stmt);
                }

                // Check if this is an infinite loop (while True) without break
                let is_infinite_loop = Self::is_infinite_loop_condition(test);
                let loop_exits = !is_infinite_loop || self.loop_has_break;

                self.in_loop = previous_in_loop;
                self.loop_has_break = previous_loop_has_break;
                
                // Code after loop is reachable if loop can exit
                self.is_reachable = previous_reachable && loop_exits;

                // Analyze else clause (orelse is Vec, not Option)
                // Else clause is reachable if we're still reachable
                if !orelse.is_empty() {
                    let else_reachable = self.is_reachable;
                    for stmt in orelse {
                        self.analyze_statement(stmt);
                    }
                    // After else, we're reachable if either the loop exits or else completes
                    self.is_reachable = else_reachable;
                }
            }

            StatementKind::For {
                target,
                iter,
                body,
                orelse,
                is_async: _,
            } => {
                // Check iterator expression
                self.check_expression(iter);
                
                // Mark loop variable as initialized
                if let Some(name) = Self::extract_pattern_variable(target) {
                    self.mark_initialized(&name);
                }
                
                let previous_in_loop = self.in_loop;
                let previous_reachable = self.is_reachable;
                let previous_loop_has_break = self.loop_has_break;
                
                self.in_loop = true;
                self.loop_has_break = false; // Reset for this loop
                self.is_reachable = true; // Loop body starts reachable

                // TODO: Track loop variable initialization

                // Analyze for body
                for stmt in body {
                    self.analyze_statement(stmt);
                }

                self.in_loop = previous_in_loop;
                self.loop_has_break = previous_loop_has_break;
                // For loops are finite, so code after is always reachable
                self.is_reachable = previous_reachable;

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
                let previous_unreachable_reported = self.unreachable_reported;
                let previous_initialized = self.initialized_variables.clone();
                
                // Analyze try body
                self.is_reachable = previous_reachable;
                self.unreachable_reported = false;
                for stmt in body {
                    self.analyze_statement(stmt);
                }
                let try_reachable = self.is_reachable;
                let try_initialized = self.initialized_variables.clone();

                // Analyze exception handlers
                // Track initialization state from all handlers
                let mut handlers_reachable = false;
                let mut handlers_initialized_sets: Vec<HashSet<String>> = Vec::new();
                
                for handler in handlers {
                    self.is_reachable = previous_reachable; // Each handler starts fresh
                    self.initialized_variables = previous_initialized.clone();
                    self.unreachable_reported = false;
                    
                    // Mark exception variable as initialized
                    if let Some(name) = &handler.name {
                        self.mark_initialized(name);
                    }
                    
                    for stmt in &handler.body {
                        self.analyze_statement(stmt);
                    }
                    handlers_reachable = handlers_reachable || self.is_reachable;
                    handlers_initialized_sets.push(self.initialized_variables.clone());
                }

                // After try/except, code is reachable if:
                // - Try block exits normally (no return/raise), OR
                // - At least one handler exists and is reachable
                let after_except_reachable = if handlers.is_empty() {
                    try_reachable
                } else {
                    try_reachable || handlers_reachable
                };
                
                self.is_reachable = after_except_reachable;
                
                // Merge initialization state based on which paths are reachable:
                // - If try and all handlers are reachable: must be in all (intersection)
                // - If only some paths are reachable: only use those paths
                if handlers.is_empty() {
                    // No handlers: just use try state
                    self.initialized_variables = try_initialized;
                } else if try_reachable && handlers_reachable {
                    // Both try and at least one handler are reachable: intersection
                    let mut merged_initialized = try_initialized;
                    for handler_initialized in handlers_initialized_sets {
                        merged_initialized = merged_initialized.intersection(&handler_initialized).cloned().collect();
                    }
                    self.initialized_variables = merged_initialized;
                } else if try_reachable {
                    // Only try block is reachable (all handlers return/raise)
                    self.initialized_variables = try_initialized;
                } else {
                    // Only handlers are reachable (try returns/raises)
                    // Use intersection of all handlers
                    if let Some(first) = handlers_initialized_sets.first() {
                        let mut merged_initialized = first.clone();
                        for handler_initialized in handlers_initialized_sets.iter().skip(1) {
                            merged_initialized = merged_initialized.intersection(handler_initialized).cloned().collect();
                        }
                        self.initialized_variables = merged_initialized;
                    }
                }

                // Analyze else clause (executes if no exception occurred)
                if !orelse.is_empty() {
                    self.unreachable_reported = false;
                    for stmt in orelse {
                        self.analyze_statement(stmt);
                    }
                    // After else, we keep the reachability from else
                }

                // Analyze finally clause - ALWAYS reachable, even if try/except return
                if !finalbody.is_empty() {
                    self.is_reachable = previous_reachable; // Finally always executes
                    self.unreachable_reported = false;
                    for stmt in finalbody {
                        self.analyze_statement(stmt);
                    }
                    // After finally, use finally's reachability (it may return/raise)
                } else {
                    // No finally clause, restore from try/except analysis
                    self.unreachable_reported = previous_unreachable_reported;
                }
            }

            StatementKind::With {
                items,
                body,
                is_async: _,
            } => {
                // Mark with statement variables as initialized
                for item in items {
                    // Check context expression
                    self.check_expression(&item.context_expr);
                    
                    // Mark optional variable as initialized
                    if let Some(optional_vars) = &item.optional_vars {
                        if let Some(name) = Self::extract_variable_name(optional_vars) {
                            self.mark_initialized(&name);
                        }
                    }
                }
                
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
                if !self.in_loop {
                    self.errors.push(SemanticError::BreakOutsideLoop {
                        line: stmt.span.line,
                        column: stmt.span.column,
                        span: stmt.span,
                    });
                }
                self.loop_has_break = true; // Mark that this loop has a break
                self.is_reachable = false; // Code after break is unreachable
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

            StatementKind::Expr(expression) => {
                // Check expression for uninitialized variable usage
                self.check_expression(expression);
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
