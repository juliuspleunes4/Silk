//! Semantic analyzer for Silk programs
//! 
//! Performs single-pass semantic analysis with a lightweight pre-pass:
//! 1. Pre-pass: Collect function and class names for forward references
//! 2. Main pass: Define symbols and validate references in one traversal

use silk_ast::{Program, Statement, StatementKind, Expression, ExpressionKind, PatternKind};
use crate::{SymbolTable, Symbol, SymbolKind, SemanticError, ScopeKind};

/// Semantic analyzer for single-pass analysis
pub struct SemanticAnalyzer {
    /// Symbol table for tracking declarations
    symbol_table: SymbolTable,
    /// Errors collected during analysis
    errors: Vec<SemanticError>,
}

impl SemanticAnalyzer {
    /// Create a new semantic analyzer
    pub fn new() -> Self {
        Self {
            symbol_table: SymbolTable::new(),
            errors: Vec::new(),
        }
    }

    /// Analyze a program and return errors if any
    pub fn analyze(&mut self, program: &Program) -> Result<(), Vec<SemanticError>> {
        // Pre-pass: Collect function and class names for forward references
        self.collect_forward_declarations(program);

        // Main pass: Analyze statements (define variables, validate references)
        for statement in &program.statements {
            self.analyze_statement(statement);
        }

        // Return errors if any
        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(std::mem::take(&mut self.errors))
        }
    }

    /// Get a reference to the symbol table (for testing)
    pub fn symbol_table(&self) -> &SymbolTable {
        &self.symbol_table
    }

    // ========== PRE-PASS: FORWARD DECLARATIONS ==========

    /// Collect function and class names for forward references
    fn collect_forward_declarations(&mut self, program: &Program) {
        for statement in &program.statements {
            match &statement.kind {
                StatementKind::FunctionDef { name, .. } => {
                    let func_symbol = Symbol::new(
                        name.clone(),
                        SymbolKind::Function,
                        statement.span.clone(),
                    );
                    if let Err(err) = self.symbol_table.define_symbol(func_symbol) {
                        self.errors.push(err);
                    }
                }
                StatementKind::ClassDef { name, .. } => {
                    let class_symbol = Symbol::new(
                        name.clone(),
                        SymbolKind::Class,
                        statement.span.clone(),
                    );
                    if let Err(err) = self.symbol_table.define_symbol(class_symbol) {
                        self.errors.push(err);
                    }
                }
                _ => {}
            }
        }
    }

    // ========== MAIN PASS: SINGLE-PASS ANALYSIS ==========

    /// Analyze a statement: define symbols and validate references
    fn analyze_statement(&mut self, stmt: &Statement) {
        match &stmt.kind {
            // Expression statement: validate the expression
            StatementKind::Expr(expr) => {
                self.analyze_expression(expr);
            }

            // Simple assignment: validate RHS first, then define LHS
            StatementKind::Assign { targets, value, .. } => {
                // Validate the value expression first
                self.analyze_expression(value);
                
                // Define the target variables
                for target in targets {
                    if let ExpressionKind::Identifier(name) = &target.kind {
                        let symbol = Symbol::new(
                            name.clone(),
                            SymbolKind::Variable,
                            target.span.clone(),
                        );
                        if let Err(err) = self.symbol_table.define_symbol(symbol) {
                            self.errors.push(err);
                        }
                    }
                    // TODO: Handle tuple unpacking, attribute assignment, etc.
                }
            }

            // Augmented assignment: validate both LHS and RHS
            StatementKind::AugAssign { target, value, .. } => {
                // Check if variable exists (must be defined before use)
                if let ExpressionKind::Identifier(name) = &target.kind {
                    if self.symbol_table.resolve_symbol(name).is_none() {
                        self.errors.push(SemanticError::UndefinedVariable {
                            name: name.clone(),
                            line: target.span.line,
                            column: target.span.column,
                            span: target.span.clone(),
                        });
                    }
                }
                self.analyze_expression(value);
                // TODO: Handle attribute/subscript augmented assignment
            }

            // Function definition: already declared in pre-pass, now analyze body
            StatementKind::FunctionDef { params, body, decorator_list, .. } => {
                // Analyze decorators BEFORE entering scope (evaluated in outer scope)
                for decorator in decorator_list {
                    self.analyze_expression(decorator);
                }

                // Analyze parameter defaults BEFORE entering scope (evaluated in outer scope)
                for param in &params.args {
                    if let Some(default_expr) = &param.default {
                        self.analyze_expression(default_expr);
                    }
                }
                for param in &params.kwonlyargs {
                    if let Some(default_expr) = &param.default {
                        self.analyze_expression(default_expr);
                    }
                }
                
                // Enter function scope
                self.symbol_table.enter_scope(ScopeKind::Function);

                // Define parameters
                for param in &params.args {
                    self.define_parameter(param);
                }
                if let Some(vararg) = &params.vararg {
                    self.define_parameter(vararg);
                }
                for param in &params.kwonlyargs {
                    self.define_parameter(param);
                }
                if let Some(kwarg) = &params.kwarg {
                    self.define_parameter(kwarg);
                }

                // Analyze function body
                for stmt in body {
                    self.analyze_statement(stmt);
                }

                // Exit function scope
                if let Err(err) = self.symbol_table.exit_scope() {
                    self.errors.push(err);
                }
            }

            // Class definition: already declared in pre-pass, now analyze body
            StatementKind::ClassDef { bases, decorator_list, body, .. } => {
                // Analyze decorators BEFORE entering scope (evaluated in outer scope)
                for decorator in decorator_list {
                    self.analyze_expression(decorator);
                }

                // Analyze base classes BEFORE entering scope (evaluated in outer scope)
                for base in bases {
                    self.analyze_expression(base);
                }

                // TODO: Analyze keywords (e.g., metaclass=...) once keyword validation is implemented

                // Enter class scope
                self.symbol_table.enter_scope(ScopeKind::Class);

                // Analyze class body
                for stmt in body {
                    self.analyze_statement(stmt);
                }

                // Exit class scope
                if let Err(err) = self.symbol_table.exit_scope() {
                    self.errors.push(err);
                }
            }

            // Import statement: define imported names
            StatementKind::Import { names } => {
                for alias in names {
                    let import_name = alias.asname.as_ref().unwrap_or(&alias.name);
                    let symbol = Symbol::new(
                        import_name.clone(),
                        SymbolKind::Module,
                        stmt.span.clone(),
                    );
                    if let Err(err) = self.symbol_table.define_symbol(symbol) {
                        self.errors.push(err);
                    }
                }
            }

            // From...import statement: define imported names
            StatementKind::ImportFrom { names, .. } => {
                for alias in names {
                    let import_name = alias.asname.as_ref().unwrap_or(&alias.name);
                    let symbol = Symbol::new(
                        import_name.clone(),
                        SymbolKind::Module,
                        stmt.span.clone(),
                    );
                    if let Err(err) = self.symbol_table.define_symbol(symbol) {
                        self.errors.push(err);
                    }
                }
            }

            // Control flow: if statement
            StatementKind::If { test, body, orelse } => {
                self.analyze_expression(test);
                for stmt in body {
                    self.analyze_statement(stmt);
                }
                for stmt in orelse {
                    self.analyze_statement(stmt);
                }
            }

            // Control flow: while loop
            StatementKind::While { test, body, orelse } => {
                self.analyze_expression(test);
                
                self.symbol_table.enter_loop();
                for stmt in body {
                    self.analyze_statement(stmt);
                }
                self.symbol_table.exit_loop();
                
                for stmt in orelse {
                    self.analyze_statement(stmt);
                }
            }

            // Control flow: for loop
            StatementKind::For { target, iter, body, orelse, .. } => {
                // Validate iterator
                self.analyze_expression(iter);
                
                // Define loop variable
                if let PatternKind::Name(name) = &target.kind {
                    let symbol = Symbol::new(
                        name.clone(),
                        SymbolKind::Variable,
                        target.span.clone(),
                    );
                    if let Err(err) = self.symbol_table.define_symbol(symbol) {
                        self.errors.push(err);
                    }
                }
                
                self.symbol_table.enter_loop();
                for stmt in body {
                    self.analyze_statement(stmt);
                }
                self.symbol_table.exit_loop();
                
                for stmt in orelse {
                    self.analyze_statement(stmt);
                }
            }

            // Context manager: with statement
            StatementKind::With { items, body, .. } => {
                for item in items {
                    self.analyze_expression(&item.context_expr);
                    
                    // Define context manager variable if present
                    if let Some(var_expr) = &item.optional_vars {
                        if let ExpressionKind::Identifier(name) = &var_expr.kind {
                            let symbol = Symbol::new(
                                name.clone(),
                                SymbolKind::Variable,
                                var_expr.span.clone(),
                            );
                            if let Err(err) = self.symbol_table.define_symbol(symbol) {
                                self.errors.push(err);
                            }
                        }
                    }
                }
                
                for stmt in body {
                    self.analyze_statement(stmt);
                }
            }

            // Exception handling: try statement
            StatementKind::Try { body, handlers, orelse, finalbody } => {
                for stmt in body {
                    self.analyze_statement(stmt);
                }
                
                for handler in handlers {
                    if let Some(type_expr) = &handler.typ {
                        self.analyze_expression(type_expr);
                    }
                    
                    // Define exception variable if present
                    if let Some(name) = &handler.name {
                        let symbol = Symbol::new(
                            name.clone(),
                            SymbolKind::Variable,
                            stmt.span.clone(),
                        );
                        if let Err(err) = self.symbol_table.define_symbol(symbol) {
                            self.errors.push(err);
                        }
                    }
                    
                    for stmt in &handler.body {
                        self.analyze_statement(stmt);
                    }
                }
                
                for stmt in orelse {
                    self.analyze_statement(stmt);
                }
                for stmt in finalbody {
                    self.analyze_statement(stmt);
                }
            }

            // Return statement: validate context and expression
            StatementKind::Return { value } => {
                if !self.symbol_table.in_function() {
                    self.errors.push(SemanticError::ReturnOutsideFunction {
                        line: stmt.span.line,
                        column: stmt.span.column,
                        span: stmt.span.clone(),
                    });
                }
                if let Some(expr) = value {
                    self.analyze_expression(expr);
                }
            }

            // Break statement: validate context
            StatementKind::Break => {
                if !self.symbol_table.in_loop() {
                    self.errors.push(SemanticError::BreakOutsideLoop {
                        line: stmt.span.line,
                        column: stmt.span.column,
                        span: stmt.span.clone(),
                    });
                }
            }

            // Continue statement: validate context
            StatementKind::Continue => {
                if !self.symbol_table.in_loop() {
                    self.errors.push(SemanticError::ContinueOutsideLoop {
                        line: stmt.span.line,
                        column: stmt.span.column,
                        span: stmt.span.clone(),
                    });
                }
            }

            // Raise statement: validate expressions
            StatementKind::Raise { exc, cause } => {
                if let Some(expr) = exc {
                    self.analyze_expression(expr);
                }
                if let Some(expr) = cause {
                    self.analyze_expression(expr);
                }
            }

            // Assert statement: validate expression
            StatementKind::Assert { test, msg } => {
                self.analyze_expression(test);
                if let Some(expr) = msg {
                    self.analyze_expression(expr);
                }
            }

            // Delete statement: validate targets
            StatementKind::Delete { targets } => {
                for target in targets {
                    self.analyze_expression(target);
                }
            }

            // Match statement: validate subject, guards, and case bodies
            StatementKind::Match { subject, cases } => {
                self.analyze_expression(subject);
                for case in cases {
                    // TODO: Handle pattern variable binding once pattern analysis is implemented
                    if let Some(guard) = &case.guard {
                        self.analyze_expression(guard);
                    }
                    for stmt in &case.body {
                        self.analyze_statement(stmt);
                    }
                }
            }

            // Other statements (Pass, Global, Nonlocal, etc.)
            _ => {}
        }
    }

    /// Analyze an expression: validate all identifier references
    fn analyze_expression(&mut self, expr: &Expression) {
        match &expr.kind {
            // Identifier: check if defined
            ExpressionKind::Identifier(name) => {
                if self.symbol_table.resolve_symbol(name).is_none() {
                    self.errors.push(SemanticError::UndefinedVariable {
                        name: name.clone(),
                        line: expr.span.line,
                        column: expr.span.column,
                        span: expr.span.clone(),
                    });
                }
            }

            // Binary operation
            ExpressionKind::BinaryOp { left, right, .. } => {
                self.analyze_expression(left);
                self.analyze_expression(right);
            }

            // Unary operation
            ExpressionKind::UnaryOp { operand, .. } => {
                self.analyze_expression(operand);
            }

            // Comparison
            ExpressionKind::Compare { left, comparators, .. } => {
                self.analyze_expression(left);
                for comp in comparators {
                    self.analyze_expression(comp);
                }
            }

            // Function call
            ExpressionKind::Call { func, args, keywords } => {
                self.analyze_expression(func);
                for arg in args {
                    self.analyze_expression(arg);
                }
                for keyword in keywords {
                    self.analyze_expression(&keyword.value);
                }
            }

            // Attribute access
            ExpressionKind::Attribute { value, .. } => {
                self.analyze_expression(value);
            }

            // Subscript
            ExpressionKind::Subscript { value, index } => {
                self.analyze_expression(value);
                self.analyze_expression(index);
            }

            // Collections
            ExpressionKind::List { elements } |
            ExpressionKind::Tuple { elements } |
            ExpressionKind::Set { elements } => {
                for elem in elements {
                    self.analyze_expression(elem);
                }
            }

            // Dictionary
            ExpressionKind::Dict { keys, values } => {
                for key in keys {
                    self.analyze_expression(key);
                }
                for value in values {
                    self.analyze_expression(value);
                }
            }

            // Ternary/conditional expression
            ExpressionKind::IfExp { test, body, orelse } => {
                self.analyze_expression(test);
                self.analyze_expression(body);
                self.analyze_expression(orelse);
            }

            // Lambda expression
            ExpressionKind::Lambda { params, body } => {
                // Analyze parameter defaults BEFORE entering scope (evaluated in outer scope)
                for param in params {
                    if let Some(default_expr) = &param.default {
                        self.analyze_expression(default_expr);
                    }
                }
                
                self.symbol_table.enter_scope(ScopeKind::Function);
                
                // Define lambda parameters
                for param in params {
                    let param_symbol = Symbol::new(
                        param.name.clone(),
                        SymbolKind::Parameter,
                        param.span.clone(),
                    );
                    if let Err(err) = self.symbol_table.define_symbol(param_symbol) {
                        self.errors.push(err);
                    }
                }
                
                self.analyze_expression(body);
                let _ = self.symbol_table.exit_scope();
            }

            // List/set/generator comprehensions
            ExpressionKind::ListComp { element, generators } |
            ExpressionKind::SetComp { element, generators } |
            ExpressionKind::GeneratorExp { element, generators } => {
                self.symbol_table.enter_scope(ScopeKind::Local);
                
                // Process generators
                for gen in generators {
                    self.analyze_expression(&gen.iter);
                    
                    // Define generator variable
                    if let PatternKind::Name(name) = &gen.target.kind {
                        let symbol = Symbol::new(
                            name.clone(),
                            SymbolKind::Variable,
                            gen.target.span.clone(),
                        );
                        let _ = self.symbol_table.define_symbol(symbol);
                    }
                    
                    for filter in &gen.ifs {
                        self.analyze_expression(filter);
                    }
                }
                
                self.analyze_expression(element);
                let _ = self.symbol_table.exit_scope();
            }

            // Dictionary comprehension
            ExpressionKind::DictComp { key, value, generators } => {
                self.symbol_table.enter_scope(ScopeKind::Local);
                
                for gen in generators {
                    self.analyze_expression(&gen.iter);
                    
                    if let PatternKind::Name(name) = &gen.target.kind {
                        let symbol = Symbol::new(
                            name.clone(),
                            SymbolKind::Variable,
                            gen.target.span.clone(),
                        );
                        let _ = self.symbol_table.define_symbol(symbol);
                    }
                    
                    for filter in &gen.ifs {
                        self.analyze_expression(filter);
                    }
                }
                
                self.analyze_expression(key);
                self.analyze_expression(value);
                let _ = self.symbol_table.exit_scope();
            }

            // Walrus operator (named expression)
            ExpressionKind::NamedExpr { target, value } => {
                self.analyze_expression(value);
                
                // Define the target variable
                if let ExpressionKind::Identifier(name) = &target.kind {
                    let symbol = Symbol::new(
                        name.clone(),
                        SymbolKind::Variable,
                        target.span.clone(),
                    );
                    if let Err(err) = self.symbol_table.define_symbol(symbol) {
                        self.errors.push(err);
                    }
                }
            }

            // Literals don't need validation
            _ => {}
        }
    }

    /// Define a function parameter
    fn define_parameter(&mut self, arg: &silk_ast::FunctionArg) {
        let param_symbol = Symbol::new(
            arg.name.clone(),
            SymbolKind::Parameter,
            arg.span.clone(),
        );
        if let Err(err) = self.symbol_table.define_symbol(param_symbol) {
            self.errors.push(err);
        }
    }
}

impl Default for SemanticAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}
