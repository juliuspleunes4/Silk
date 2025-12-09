//! Semantic analyzer for Silk programs
//! 
//! Performs two-pass semantic analysis:
//! 1. Symbol collection: Walk AST and collect all definitions
//! 2. Name resolution: Resolve all identifier references and check for errors

use silk_ast::{Program, Statement, StatementKind, Expression, ExpressionKind, Parameter, PatternKind, FunctionParams};
use silk_lexer::Span;
use crate::{SymbolTable, Symbol, SymbolKind, SemanticError, ScopeKind};

/// Semantic analyzer for performing multi-pass analysis
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
        // Pass 1: Collect all symbol definitions
        self.collect_symbols(program);

        // Pass 2: Resolve all references
        self.resolve_references(program);

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

    // ========== PASS 1: SYMBOL COLLECTION ==========

    /// First pass: collect all symbol definitions
    fn collect_symbols(&mut self, program: &Program) {
        for statement in &program.statements {
            self.collect_symbols_from_statement(statement);
        }
    }

    /// Collect symbols from a statement
    fn collect_symbols_from_statement(&mut self, stmt: &Statement) {
        match &stmt.kind {
            // Simple assignment: x = value
            StatementKind::Assign { targets, .. } => {
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

            // Augmented assignment: x += value
            // Augmented assignment requires the variable to already exist (x += 5 is x = x + 5)
            StatementKind::AugAssign { target, .. } => {
                if let ExpressionKind::Identifier(name) = &target.kind {
                    // Check if variable exists; error if not
                    if self.symbol_table.resolve_symbol(name).is_none() {
                        self.errors.push(SemanticError::UndefinedVariable {
                            name: name.clone(),
                            line: target.span.line,
                            column: target.span.column,
                            span: target.span.clone(),
                        });
                    }
                }
                // TODO: Handle attribute/subscript augmented assignment
            }

            // Function definition
            StatementKind::FunctionDef { name, params, body, is_async, .. } => {
                self.collect_function_def(name, params, body, *is_async, stmt.span.clone());
            }

            // Class definition
            StatementKind::ClassDef { name, body, .. } => {
                self.collect_class_def(name, body, stmt.span.clone());
            }

            // Import statement
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

            // From...import statement
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

            // Control flow statements with nested blocks
            StatementKind::If { body, orelse, .. } => {
                for stmt in body {
                    self.collect_symbols_from_statement(stmt);
                }
                for stmt in orelse {
                    self.collect_symbols_from_statement(stmt);
                }
            }

            StatementKind::While { body, orelse, .. } => {
                for stmt in body {
                    self.collect_symbols_from_statement(stmt);
                }
                for stmt in orelse {
                    self.collect_symbols_from_statement(stmt);
                }
            }

            StatementKind::For { target, body, orelse, .. } => {
                // Loop variable is defined
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
                
                for stmt in body {
                    self.collect_symbols_from_statement(stmt);
                }
                for stmt in orelse {
                    self.collect_symbols_from_statement(stmt);
                }
            }

            StatementKind::With { items, body, .. } => {
                // Context manager variables
                for item in items {
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
                    self.collect_symbols_from_statement(stmt);
                }
            }

            StatementKind::Try { body, handlers, orelse, finalbody } => {
                for stmt in body {
                    self.collect_symbols_from_statement(stmt);
                }
                for handler in handlers {
                    // Exception variable
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
                        self.collect_symbols_from_statement(stmt);
                    }
                }
                for stmt in orelse {
                    self.collect_symbols_from_statement(stmt);
                }
                for stmt in finalbody {
                    self.collect_symbols_from_statement(stmt);
                }
            }

            // Other statements don't define symbols
            _ => {}
        }
    }

    /// Collect symbols from function definition
    fn collect_function_def(&mut self, name: &str, params: &FunctionParams, body: &[Statement], _is_async: bool, span: Span) {
        // Define function in current scope
        let func_symbol = Symbol::new(
            name.to_string(),
            SymbolKind::Function,
            span.clone(),
        );
        if let Err(err) = self.symbol_table.define_symbol(func_symbol) {
            self.errors.push(err);
        }

        // Enter function scope
        self.symbol_table.enter_scope(ScopeKind::Function);

        // Define parameters from all parameter types
        for param in &params.args {
            self.collect_function_arg(param);
        }
        if let Some(vararg) = &params.vararg {
            self.collect_function_arg(vararg);
        }
        for param in &params.kwonlyargs {
            self.collect_function_arg(param);
        }
        if let Some(kwarg) = &params.kwarg {
            self.collect_function_arg(kwarg);
        }

        // Collect symbols from function body
        for stmt in body {
            self.collect_symbols_from_statement(stmt);
        }

        // Exit function scope
        if let Err(err) = self.symbol_table.exit_scope() {
            self.errors.push(err);
        }
    }

    /// Collect function argument symbol
    fn collect_function_arg(&mut self, arg: &silk_ast::FunctionArg) {
        let param_symbol = Symbol::new(
            arg.name.clone(),
            SymbolKind::Parameter,
            arg.span.clone(),
        );
        if let Err(err) = self.symbol_table.define_symbol(param_symbol) {
            self.errors.push(err);
        }
    }

    /// Collect lambda parameter symbol
    fn collect_lambda_parameter(&mut self, param: &Parameter) {
        let param_symbol = Symbol::new(
            param.name.clone(),
            SymbolKind::Parameter,
            param.span.clone(),
        );
        if let Err(err) = self.symbol_table.define_symbol(param_symbol) {
            self.errors.push(err);
        }
    }

    /// Collect symbols from class definition
    fn collect_class_def(&mut self, name: &str, body: &[Statement], span: Span) {
        // Define class in current scope
        let class_symbol = Symbol::new(
            name.to_string(),
            SymbolKind::Class,
            span.clone(),
        );
        if let Err(err) = self.symbol_table.define_symbol(class_symbol) {
            self.errors.push(err);
        }

        // Enter class scope
        self.symbol_table.enter_scope(ScopeKind::Class);

        // Collect symbols from class body
        for stmt in body {
            self.collect_symbols_from_statement(stmt);
        }

        // Exit class scope
        if let Err(err) = self.symbol_table.exit_scope() {
            self.errors.push(err);
        }
    }

    // ========== PASS 2: NAME RESOLUTION ==========

    /// Second pass: resolve all identifier references
    fn resolve_references(&mut self, program: &Program) {
        for statement in &program.statements {
            self.resolve_statement(statement);
        }
    }

    /// Resolve references in a statement
    fn resolve_statement(&mut self, stmt: &Statement) {
        match &stmt.kind {
            StatementKind::Expr(expr) => {
                self.resolve_expression(expr);
            }

            StatementKind::Assign { value, .. } => {
                self.resolve_expression(value);
            }

            StatementKind::AugAssign { target, value, .. } => {
                self.resolve_expression(target);
                self.resolve_expression(value);
            }

            StatementKind::Return { value } => {
                // Check if return is in function
                if !self.symbol_table.in_function() {
                    self.errors.push(SemanticError::ReturnOutsideFunction {
                        line: stmt.span.line,
                        column: stmt.span.column,
                        span: stmt.span.clone(),
                    });
                }
                if let Some(expr) = value {
                    self.resolve_expression(expr);
                }
            }

            StatementKind::Break => {
                if !self.symbol_table.in_loop() {
                    self.errors.push(SemanticError::BreakOutsideLoop {
                        line: stmt.span.line,
                        column: stmt.span.column,
                        span: stmt.span.clone(),
                    });
                }
            }

            StatementKind::Continue => {
                if !self.symbol_table.in_loop() {
                    self.errors.push(SemanticError::ContinueOutsideLoop {
                        line: stmt.span.line,
                        column: stmt.span.column,
                        span: stmt.span.clone(),
                    });
                }
            }

            StatementKind::FunctionDef { params, body, .. } => {
                self.symbol_table.enter_scope(ScopeKind::Function);
                
                // NOTE: Re-defining parameters here because each pass creates new scopes.
                // This is redundant but necessary with current two-pass architecture.
                // Parameters allow redefinition, so this doesn't cause errors.
                // TODO: Refactor to single-pass or persist scopes between passes.
                for param in &params.args {
                    self.collect_function_arg(param);
                }
                if let Some(vararg) = &params.vararg {
                    self.collect_function_arg(vararg);
                }
                for param in &params.kwonlyargs {
                    self.collect_function_arg(param);
                }
                if let Some(kwarg) = &params.kwarg {
                    self.collect_function_arg(kwarg);
                }
                
                for stmt in body {
                    self.resolve_statement(stmt);
                }
                let _ = self.symbol_table.exit_scope();
            }

            StatementKind::ClassDef { body, .. } => {
                self.symbol_table.enter_scope(ScopeKind::Class);
                for stmt in body {
                    self.resolve_statement(stmt);
                }
                let _ = self.symbol_table.exit_scope();
            }

            StatementKind::If { test, body, orelse } => {
                self.resolve_expression(test);
                for stmt in body {
                    self.resolve_statement(stmt);
                }
                for stmt in orelse {
                    self.resolve_statement(stmt);
                }
            }

            StatementKind::While { test, body, orelse } => {
                self.resolve_expression(test);
                
                self.symbol_table.enter_loop();
                for stmt in body {
                    self.resolve_statement(stmt);
                }
                self.symbol_table.exit_loop();
                
                for stmt in orelse {
                    self.resolve_statement(stmt);
                }
            }

            StatementKind::For { target: _, iter, body, orelse, .. } => {
                self.resolve_expression(iter);
                
                self.symbol_table.enter_loop();
                for stmt in body {
                    self.resolve_statement(stmt);
                }
                self.symbol_table.exit_loop();
                
                for stmt in orelse {
                    self.resolve_statement(stmt);
                }
            }

            StatementKind::With { items, body, .. } => {
                for item in items {
                    self.resolve_expression(&item.context_expr);
                }
                for stmt in body {
                    self.resolve_statement(stmt);
                }
            }

            StatementKind::Raise { exc, cause } => {
                if let Some(expr) = exc {
                    self.resolve_expression(expr);
                }
                if let Some(expr) = cause {
                    self.resolve_expression(expr);
                }
            }

            StatementKind::Assert { test, msg } => {
                self.resolve_expression(test);
                if let Some(expr) = msg {
                    self.resolve_expression(expr);
                }
            }

            StatementKind::Try { body, handlers, orelse, finalbody } => {
                for stmt in body {
                    self.resolve_statement(stmt);
                }
                for handler in handlers {
                    if let Some(type_expr) = &handler.typ {
                        self.resolve_expression(type_expr);
                    }
                    for stmt in &handler.body {
                        self.resolve_statement(stmt);
                    }
                }
                for stmt in orelse {
                    self.resolve_statement(stmt);
                }
                for stmt in finalbody {
                    self.resolve_statement(stmt);
                }
            }

            StatementKind::Delete { targets } => {
                for target in targets {
                    self.resolve_expression(target);
                }
            }

            // Other statements
            _ => {}
        }
    }

    /// Resolve references in an expression
    fn resolve_expression(&mut self, expr: &Expression) {
        match &expr.kind {
            ExpressionKind::Identifier(name) => {
                // Check if identifier is defined
                if self.symbol_table.resolve_symbol(name).is_none() {
                    self.errors.push(SemanticError::UndefinedVariable {
                        name: name.clone(),
                        line: expr.span.line,
                        column: expr.span.column,
                        span: expr.span.clone(),
                    });
                }
            }

            ExpressionKind::BinaryOp { left, right, .. } => {
                self.resolve_expression(left);
                self.resolve_expression(right);
            }

            ExpressionKind::UnaryOp { operand, .. } => {
                self.resolve_expression(operand);
            }

            ExpressionKind::Compare { left, comparators, .. } => {
                self.resolve_expression(left);
                for comp in comparators {
                    self.resolve_expression(comp);
                }
            }

            ExpressionKind::Call { func, args, keywords } => {
                self.resolve_expression(func);
                for arg in args {
                    self.resolve_expression(arg);
                }
                for keyword in keywords {
                    self.resolve_expression(&keyword.value);
                }
            }

            ExpressionKind::Attribute { value, .. } => {
                self.resolve_expression(value);
            }

            ExpressionKind::Subscript { value, index } => {
                self.resolve_expression(value);
                self.resolve_expression(index);
            }

            ExpressionKind::List { elements } |
            ExpressionKind::Tuple { elements } |
            ExpressionKind::Set { elements } => {
                for elem in elements {
                    self.resolve_expression(elem);
                }
            }

            ExpressionKind::Dict { keys, values } => {
                for key in keys {
                    self.resolve_expression(key);
                }
                for value in values {
                    self.resolve_expression(value);
                }
            }

            ExpressionKind::IfExp { test, body, orelse } => {
                self.resolve_expression(test);
                self.resolve_expression(body);
                self.resolve_expression(orelse);
            }

            ExpressionKind::Lambda { params, body } => {
                self.symbol_table.enter_scope(ScopeKind::Function);
                
                // Define lambda parameters
                for param in params {
                    self.collect_lambda_parameter(param);
                }
                
                self.resolve_expression(body);
                let _ = self.symbol_table.exit_scope();
            }

            ExpressionKind::ListComp { element, generators } |
            ExpressionKind::SetComp { element, generators } |
            ExpressionKind::GeneratorExp { element, generators } => {
                self.symbol_table.enter_scope(ScopeKind::Local);
                
                // Process generators
                for gen in generators {
                    self.resolve_expression(&gen.iter);
                    // Target becomes a variable in comp scope
                    if let PatternKind::Name(name) = &gen.target.kind {
                        let symbol = Symbol::new(
                            name.clone(),
                            SymbolKind::Variable,
                            gen.target.span.clone(),
                        );
                        let _ = self.symbol_table.define_symbol(symbol);
                    }
                    for filter in &gen.ifs {
                        self.resolve_expression(filter);
                    }
                }
                
                self.resolve_expression(element);
                let _ = self.symbol_table.exit_scope();
            }

            ExpressionKind::DictComp { key, value, generators } => {
                self.symbol_table.enter_scope(ScopeKind::Local);
                
                for gen in generators {
                    self.resolve_expression(&gen.iter);
                    if let PatternKind::Name(name) = &gen.target.kind {
                        let symbol = Symbol::new(
                            name.clone(),
                            SymbolKind::Variable,
                            gen.target.span.clone(),
                        );
                        let _ = self.symbol_table.define_symbol(symbol);
                    }
                    for filter in &gen.ifs {
                        self.resolve_expression(filter);
                    }
                }
                
                self.resolve_expression(key);
                self.resolve_expression(value);
                let _ = self.symbol_table.exit_scope();
            }

            ExpressionKind::NamedExpr { target, value } => {
                self.resolve_expression(value);
                // Define the target variable (walrus operator)
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

            // Literals don't need resolution
            _ => {}
        }
    }
}

impl Default for SemanticAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}
