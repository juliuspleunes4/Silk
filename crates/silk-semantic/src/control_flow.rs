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
use std::collections::{HashMap, HashSet};

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
    /// Stack of scopes, each containing initialized variables
    /// Inner functions can see variables from outer scopes (closures)
    scope_stack: Vec<HashSet<String>>,
    /// Map of variable names to their assignment locations (for unused detection)
    assigned_variables: HashMap<String, Span>,
    /// Set of variables that have been used (read)
    used_variables: HashSet<String>,
    /// Map of function names to their definition locations (for unused detection)
    defined_functions: HashMap<String, Span>,
    /// Set of functions that have been called
    called_functions: HashSet<String>,
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
            scope_stack: vec![HashSet::new()], // Start with global scope
            assigned_variables: HashMap::new(),
            used_variables: HashSet::new(),
            defined_functions: HashMap::new(),
            called_functions: HashSet::new(),
        }
    }

    /// Analyze a program and return errors if any
    pub fn analyze(&mut self, program: &Program) -> Result<(), Vec<SemanticError>> {
        // Analyze all statements in the program
        for statement in &program.statements {
            self.analyze_statement(statement);
        }
        
        // Report unused variables (excluding those with _ prefix)
        self.report_unused_variables();
        
        // Report unused functions (excluding those with _ prefix)
        self.report_unused_functions();
        
        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(std::mem::take(&mut self.errors))
        }
    }

    /// Report variables that were assigned but never used
    fn report_unused_variables(&mut self) {
        for (name, span) in &self.assigned_variables {
            // Skip variables starting with underscore (Python convention for unused)
            if name.starts_with('_') {
                continue;
            }
            
            // Check if variable was ever used
            if !self.used_variables.contains(name) {
                self.errors.push(SemanticError::UnusedVariable {
                    name: name.clone(),
                    line: span.line,
                    column: span.column,
                    span: span.clone(),
                });
            }
        }
    }

    /// Report functions that were defined but never called
    fn report_unused_functions(&mut self) {
        for (name, span) in &self.defined_functions {
            // Skip functions starting with underscore (Python convention for unused)
            if name.starts_with('_') {
                continue;
            }
            
            // Skip special functions that are implicitly used
            if name == "main" {
                continue; // main is the entry point
            }
            
            // Check if function was ever called
            if !self.called_functions.contains(name) {
                self.errors.push(SemanticError::UnusedFunction {
                    name: name.clone(),
                    line: span.line,
                    column: span.column,
                    span: span.clone(),
                });
            }
        }
    }

    /// Track a function definition
    fn track_function_definition(&mut self, name: &str, span: &Span) {
        // Only track the first definition (ignore redefinitions)
        self.defined_functions.entry(name.to_string()).or_insert(span.clone());
    }

    /// Track a function call
    fn track_function_call(&mut self, name: &str) {
        self.called_functions.insert(name.to_string());
    }

    /// Get a reference to the collected errors (for testing)
    pub fn errors(&self) -> &[SemanticError] {
        &self.errors
    }

    // ========== SCOPE MANAGEMENT ==========

    /// Push a new scope onto the stack (for nested functions)
    fn push_scope(&mut self) {
        self.scope_stack.push(HashSet::new());
    }

    /// Pop the current scope from the stack
    fn pop_scope(&mut self) {
        if self.scope_stack.len() > 1 {
            self.scope_stack.pop();
        }
    }

    /// Get a reference to the current (innermost) scope
    fn current_scope(&self) -> &HashSet<String> {
        self.scope_stack.last().expect("Scope stack should never be empty")
    }

    /// Get a mutable reference to the current (innermost) scope
    fn current_scope_mut(&mut self) -> &mut HashSet<String> {
        self.scope_stack.last_mut().expect("Scope stack should never be empty")
    }

    /// Check if a variable is initialized in any scope (current or outer)
    fn is_initialized(&self, name: &str) -> bool {
        // Search from innermost to outermost scope
        self.scope_stack.iter().rev().any(|scope| scope.contains(name))
    }

    /// Clone the entire scope stack
    fn clone_scope_stack(&self) -> Vec<HashSet<String>> {
        self.scope_stack.clone()
    }

    /// Restore the scope stack
    fn restore_scope_stack(&mut self, stack: Vec<HashSet<String>>) {
        self.scope_stack = stack;
    }

    // ========== HELPER METHODS ==========

    /// Mark a variable as initialized in the current scope
    fn mark_initialized(&mut self, name: &str) {
        self.current_scope_mut().insert(name.to_string());
    }

    /// Track that a variable was assigned (for unused variable detection)
    fn track_assignment(&mut self, name: &str, span: &Span) {
        // Only track the first assignment location
        if !self.assigned_variables.contains_key(name) {
            self.assigned_variables.insert(name.to_string(), span.clone());
        }
    }

    /// Track that a variable was used (read)
    fn track_usage(&mut self, name: &str) {
        self.used_variables.insert(name.to_string());
    }

    /// Check if a variable is initialized, report error if not
    fn check_initialized(&mut self, name: &str, span: &Span) {
        if !self.is_initialized(name) {
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

    /// Track usage without checking initialization (for function calls to built-ins)
    fn track_expression_usage(&mut self, expr: &Expression) {
        match &expr.kind {
            ExpressionKind::Identifier(name) => {
                self.track_usage(name);
            }
            ExpressionKind::Attribute { value, .. } => {
                self.track_expression_usage(value);
            }
            ExpressionKind::Subscript { value, index } => {
                self.track_expression_usage(value);
                self.track_expression_usage(index);
            }
            // For other expressions, just track nested identifiers
            ExpressionKind::BinaryOp { left, right, .. } => {
                self.track_expression_usage(left);
                self.track_expression_usage(right);
            }
            ExpressionKind::UnaryOp { operand, .. } => {
                self.track_expression_usage(operand);
            }
            _ => {}
        }
    }

    /// Check an expression for uninitialized variable usage
    fn check_expression(&mut self, expr: &Expression) {
        match &expr.kind {
            ExpressionKind::Identifier(name) => {
                self.check_initialized(name, &expr.span);
                self.track_usage(name);
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
            ExpressionKind::Call { func, args, keywords } => {
                // Track function call if calling a named function
                if let ExpressionKind::Identifier(name) = &func.kind {
                    self.track_function_call(name);
                }
                
                // Track usage of function variable (for lambdas, variables holding functions)
                // but don't require initialization (to allow built-in functions)
                self.track_expression_usage(func);
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
                // Push new scope for lambda
                self.push_scope();
                
                // Mark lambda parameters as initialized
                for param in params {
                    self.mark_initialized(&param.name);
                }
                
                // Check lambda body with parameters marked as initialized
                self.check_expression(body);
                
                // Pop lambda scope
                self.pop_scope();
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
                    self.track_assignment(&name, &target.span);
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
                
                // Mark all target variables as initialized and track assignment
                for target in targets {
                    if let Some(name) = Self::extract_variable_name(target) {
                        self.mark_initialized(&name);
                        self.track_assignment(&name, &target.span);
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
                
                // Mark target as initialized and track assignment
                if let Some(name) = Self::extract_variable_name(target) {
                    self.mark_initialized(&name);
                    self.track_assignment(&name, &target.span);
                }
            }

            StatementKind::AugAssign { target, op: _, value } => {
                // Augmented assignment requires variable to already exist
                // Check both target (must be initialized) and value
                self.check_expression(target);
                self.check_expression(value);
            }

            // Function definition
            StatementKind::FunctionDef { name, body, params, returns, decorator_list, .. } => {
                // Track function definition (for unused function detection)
                self.track_function_definition(name, &stmt.span);
                
                // Decorated functions are considered "used" (called by the decorator)
                if !decorator_list.is_empty() {
                    self.track_function_call(name);
                }
                
                let previous_in_function = self.current_function_returns;
                let previous_in_loop = self.in_loop;
                let previous_reachable = self.is_reachable;
                let previous_unreachable_reported = self.unreachable_reported;
                
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
                
                // Push new scope for function (inherits outer scope visibility)
                self.push_scope();
                
                // Mark all function parameters as initialized and track as assigned
                for param in &params.args {
                    self.mark_initialized(&param.name);
                    self.track_assignment(&param.name, &param.span);
                }
                if let Some(vararg) = &params.vararg {
                    self.mark_initialized(&vararg.name);
                    self.track_assignment(&vararg.name, &vararg.span);
                }
                for param in &params.kwonlyargs {
                    self.mark_initialized(&param.name);
                    self.track_assignment(&param.name, &param.span);
                }
                if let Some(kwarg) = &params.kwarg {
                    self.mark_initialized(&kwarg.name);
                    self.track_assignment(&kwarg.name, &kwarg.span);
                }

                // Analyze function body
                for stmt in body {
                    self.analyze_statement(stmt);
                }

                // Check if function returns on all paths
                // A function must return if:
                // 1. It has a return type annotation
                // 2. The return type is not None
                // 3. The end of function is reachable (no return on current path)
                if let Some(return_type) = returns {
                    // Check if return type is None (functions returning None don't need explicit return)
                    let is_none_return = matches!(&return_type.kind, silk_ast::TypeKind::None) 
                        || matches!(&return_type.kind, silk_ast::TypeKind::Name(n) if n == "None");
                    
                    // If function has non-None return type and end is reachable, report error
                    if !is_none_return && self.is_reachable {
                        self.errors.push(SemanticError::MissingReturn {
                            function_name: name.clone(),
                            line: stmt.span.line,
                            column: stmt.span.column,
                            span: stmt.span,
                        });
                    }
                }

                self.current_function_returns = previous_in_function;
                self.in_loop = previous_in_loop;
                self.is_reachable = previous_reachable; // Restore reachability
                self.unreachable_reported = previous_unreachable_reported;
                
                // Pop function scope
                self.pop_scope();
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
                let previous_scope_stack = self.clone_scope_stack();
                
                // Analyze if body
                self.is_reachable = previous_reachable;
                self.unreachable_reported = false; // Reset for if block
                for stmt in body {
                    self.analyze_statement(stmt);
                }
                let if_reachable = self.is_reachable;
                let if_scope_stack = self.clone_scope_stack();

                // Analyze else body (orelse is Vec, not Option)
                // If orelse is empty, it's implicitly reachable (no code to execute)
                let (else_reachable, else_scope_stack) = if orelse.is_empty() {
                    // No else clause - restore previous state
                    (previous_reachable, previous_scope_stack.clone())
                } else {
                    self.is_reachable = previous_reachable;
                    self.restore_scope_stack(previous_scope_stack.clone());
                    self.unreachable_reported = false; // Reset for else block
                    for stmt in orelse {
                        self.analyze_statement(stmt);
                    }
                    (self.is_reachable, self.clone_scope_stack())
                };
                
                // Code after if is reachable if either branch is reachable
                self.is_reachable = if_reachable || else_reachable;
                self.unreachable_reported = previous_unreachable_reported; // Restore
                
                // Variable is initialized after if/else based on which branches are reachable:
                // - If both branches reachable: must be in both (intersection of current scope only)
                // - If only if branch reachable: use if branch state
                // - If only else branch reachable: use else branch state
                // - If neither reachable: doesn't matter (code is unreachable)
                if if_reachable && else_reachable {
                    // Both reachable - merge scopes (intersection of current scope only)
                    let if_current = if_scope_stack.last().unwrap();
                    let else_current = else_scope_stack.last().unwrap();
                    let merged: HashSet<String> = if_current.intersection(else_current).cloned().collect();
                    self.restore_scope_stack(previous_scope_stack);
                    *self.current_scope_mut() = merged;
                } else if if_reachable {
                    self.restore_scope_stack(if_scope_stack);
                } else {
                    self.restore_scope_stack(else_scope_stack);
                }
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
                
                // Mark loop variable as initialized and track assignment
                if let Some(name) = Self::extract_pattern_variable(target) {
                    self.mark_initialized(&name);
                    self.track_assignment(&name, &target.span);
                }
                
                let previous_in_loop = self.in_loop;
                let previous_reachable = self.is_reachable;
                let previous_loop_has_break = self.loop_has_break;
                
                self.in_loop = true;
                self.loop_has_break = false; // Reset for this loop
                self.is_reachable = true; // Loop body starts reachable

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
                let previous_scope_stack = self.clone_scope_stack();
                
                // Analyze try body
                self.is_reachable = previous_reachable;
                self.unreachable_reported = false;
                for stmt in body {
                    self.analyze_statement(stmt);
                }
                let try_reachable = self.is_reachable;
                let try_scope_stack = self.clone_scope_stack();

                // Analyze exception handlers
                // Track initialization state from all handlers
                let mut handlers_reachable = false;
                let mut handlers_scope_stacks: Vec<Vec<HashSet<String>>> = Vec::new();
                
                for handler in handlers {
                    self.is_reachable = previous_reachable; // Each handler starts fresh
                    self.restore_scope_stack(previous_scope_stack.clone());
                    self.unreachable_reported = false;
                    
                    // Mark exception variable as initialized and track assignment
                    if let Some(name) = &handler.name {
                        self.mark_initialized(name);
                        // Exception handler variable - get span from handler
                        self.track_assignment(name, &handler.span);
                    }
                    
                    for stmt in &handler.body {
                        self.analyze_statement(stmt);
                    }
                    handlers_reachable = handlers_reachable || self.is_reachable;
                    handlers_scope_stacks.push(self.clone_scope_stack());
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
                    self.restore_scope_stack(try_scope_stack);
                } else if try_reachable && handlers_reachable {
                    // Both try and at least one handler are reachable: intersection of current scope only
                    let try_current = try_scope_stack.last().unwrap();
                    let mut merged_current = try_current.clone();
                    for handler_scope_stack in &handlers_scope_stacks {
                        let handler_current = handler_scope_stack.last().unwrap();
                        merged_current = merged_current.intersection(handler_current).cloned().collect();
                    }
                    self.restore_scope_stack(previous_scope_stack);
                    *self.current_scope_mut() = merged_current;
                } else if try_reachable {
                    // Only try block is reachable (all handlers return/raise)
                    self.restore_scope_stack(try_scope_stack);
                } else {
                    // Only handlers are reachable (try returns/raises)
                    // Use intersection of all handlers' current scopes
                    if let Some(first) = handlers_scope_stacks.first() {
                        let mut merged_current = first.last().unwrap().clone();
                        for handler_scope_stack in handlers_scope_stacks.iter().skip(1) {
                            let handler_current = handler_scope_stack.last().unwrap();
                            merged_current = merged_current.intersection(handler_current).cloned().collect();
                        }
                        self.restore_scope_stack(previous_scope_stack);
                        *self.current_scope_mut() = merged_current;
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
                // Mark with statement variables as initialized and track assignment
                for item in items {
                    // Check context expression
                    self.check_expression(&item.context_expr);
                    
                    // Mark optional variable as initialized and track assignment
                    if let Some(optional_vars) = &item.optional_vars {
                        if let Some(name) = Self::extract_variable_name(optional_vars) {
                            self.mark_initialized(&name);
                            self.track_assignment(&name, &optional_vars.span);
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
            StatementKind::Return { value } => {
                // Check return value expression
                if let Some(expr) = value {
                    self.check_expression(expr);
                }
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
y = x + 20

def foo():
    return 42

result = foo()

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
print(y)
print(result)

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
        return 2
    result = inner()
    return x + result

value = outer()
print(value)
"#;
        let program = Parser::parse(source).expect("Failed to parse");
        let mut analyzer = ControlFlowAnalyzer::new();
        
        let result = analyzer.analyze(&program);
        assert!(result.is_ok(), "Should succeed: {:?}", result);
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
