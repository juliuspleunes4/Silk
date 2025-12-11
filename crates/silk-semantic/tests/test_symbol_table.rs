//! Tests for symbol table and scope management

use silk_lexer::Span;
use silk_semantic::{ScopeKind, SemanticError, Symbol, SymbolKind, SymbolTable};

// ========== HELPER FUNCTIONS ==========

fn make_span(line: usize, column: usize) -> Span {
    Span::new(0, 0, line, column)
}

// ========== BASIC SYMBOL DEFINITION TESTS ==========

#[test]
fn test_define_variable_in_global_scope() {
    let mut table = SymbolTable::new();

    let symbol = Symbol::new("x".to_string(), SymbolKind::Variable, make_span(1, 1));
    let result = table.define_symbol(symbol);

    assert!(result.is_ok());
}

#[test]
fn test_define_multiple_variables() {
    let mut table = SymbolTable::new();

    let x = Symbol::new("x".to_string(), SymbolKind::Variable, make_span(1, 1));
    let y = Symbol::new("y".to_string(), SymbolKind::Variable, make_span(2, 1));
    let z = Symbol::new("z".to_string(), SymbolKind::Variable, make_span(3, 1));

    assert!(table.define_symbol(x).is_ok());
    assert!(table.define_symbol(y).is_ok());
    assert!(table.define_symbol(z).is_ok());
}

#[test]
fn test_resolve_defined_variable() {
    let mut table = SymbolTable::new();

    let symbol = Symbol::new("x".to_string(), SymbolKind::Variable, make_span(1, 1));
    table.define_symbol(symbol).unwrap();

    let resolved = table.resolve_symbol("x");
    assert!(resolved.is_some());
    assert_eq!(resolved.unwrap().name, "x");
    assert_eq!(resolved.unwrap().kind, SymbolKind::Variable);
}

#[test]
fn test_resolve_undefined_variable() {
    let table = SymbolTable::new();

    let resolved = table.resolve_symbol("undefined");
    assert!(resolved.is_none());
}

#[test]
fn test_define_function_symbol() {
    let mut table = SymbolTable::new();

    let func = Symbol::new("foo".to_string(), SymbolKind::Function, make_span(1, 1));
    assert!(table.define_symbol(func).is_ok());

    let resolved = table.resolve_symbol("foo");
    assert!(resolved.is_some());
    assert_eq!(resolved.unwrap().kind, SymbolKind::Function);
}

#[test]
fn test_define_class_symbol() {
    let mut table = SymbolTable::new();

    let class = Symbol::new("MyClass".to_string(), SymbolKind::Class, make_span(1, 1));
    assert!(table.define_symbol(class).is_ok());

    let resolved = table.resolve_symbol("MyClass");
    assert!(resolved.is_some());
    assert_eq!(resolved.unwrap().kind, SymbolKind::Class);
}

#[test]
fn test_variable_reassignment_allowed() {
    let mut table = SymbolTable::new();

    let x1 = Symbol::new("x".to_string(), SymbolKind::Variable, make_span(1, 1));
    let x2 = Symbol::new("x".to_string(), SymbolKind::Variable, make_span(5, 1));

    assert!(table.define_symbol(x1).is_ok());
    // Variables can be reassigned in Python
    assert!(table.define_symbol(x2).is_ok());
}

#[test]
fn test_function_redefinition_error() {
    let mut table = SymbolTable::new();

    let func1 = Symbol::new("foo".to_string(), SymbolKind::Function, make_span(1, 1));
    let func2 = Symbol::new("foo".to_string(), SymbolKind::Function, make_span(5, 1));

    assert!(table.define_symbol(func1).is_ok());
    let result = table.define_symbol(func2);

    assert!(result.is_err());
    match result {
        Err(SemanticError::RedefinedVariable {
            name,
            line,
            first_line,
            ..
        }) => {
            assert_eq!(name, "foo");
            assert_eq!(line, 5);
            assert_eq!(first_line, 1);
        }
        _ => panic!("Expected RedefinedVariable error"),
    }
}

#[test]
fn test_symbol_with_type_info() {
    use silk_semantic::Type;

    let mut table = SymbolTable::new();

    let symbol = Symbol::with_type(
        "x".to_string(),
        SymbolKind::Variable,
        make_span(1, 1),
        Type::Int,
    );

    table.define_symbol(symbol).unwrap();

    let resolved = table.resolve_symbol("x").unwrap();
    assert_eq!(resolved.ty, Type::Int);
}

#[test]
fn test_parameter_symbol() {
    let mut table = SymbolTable::new();

    let param = Symbol::new("arg".to_string(), SymbolKind::Parameter, make_span(1, 10));
    table.define_symbol(param).unwrap();

    let resolved = table.resolve_symbol("arg").unwrap();
    assert_eq!(resolved.kind, SymbolKind::Parameter);
}

// ========== SCOPE MANAGEMENT TESTS ==========

#[test]
fn test_enter_and_exit_scope() {
    let mut table = SymbolTable::new();

    assert_eq!(table.current_scope_kind(), ScopeKind::Global);

    table.enter_scope(ScopeKind::Function);
    assert_eq!(table.current_scope_kind(), ScopeKind::Function);

    table.exit_scope().unwrap();
    assert_eq!(table.current_scope_kind(), ScopeKind::Global);
}

#[test]
fn test_cannot_exit_global_scope() {
    let mut table = SymbolTable::new();

    let result = table.exit_scope();
    assert!(result.is_err());

    match result {
        Err(SemanticError::InvalidScope { message }) => {
            assert!(message.contains("Cannot exit global scope"));
        }
        _ => panic!("Expected InvalidScope error"),
    }
}

#[test]
fn test_nested_scopes() {
    let mut table = SymbolTable::new();

    // Global scope
    let global_var = Symbol::new("x".to_string(), SymbolKind::Variable, make_span(1, 1));
    table.define_symbol(global_var).unwrap();

    // Enter function scope
    table.enter_scope(ScopeKind::Function);
    let local_var = Symbol::new("y".to_string(), SymbolKind::Variable, make_span(3, 5));
    table.define_symbol(local_var).unwrap();

    // Can resolve both x (from parent) and y (local)
    assert!(table.resolve_symbol("x").is_some());
    assert!(table.resolve_symbol("y").is_some());

    // Exit function scope
    table.exit_scope().unwrap();

    // Can still resolve x, but not y
    assert!(table.resolve_symbol("x").is_some());
    assert!(table.resolve_symbol("y").is_none());
}

#[test]
fn test_variable_shadowing() {
    let mut table = SymbolTable::new();

    // Define x in global scope
    let global_x = Symbol::new("x".to_string(), SymbolKind::Variable, make_span(1, 1));
    table.define_symbol(global_x).unwrap();

    // Enter function scope and define x again
    table.enter_scope(ScopeKind::Function);
    let local_x = Symbol::new("x".to_string(), SymbolKind::Variable, make_span(3, 5));
    table.define_symbol(local_x).unwrap();

    // Resolve x - should get local version
    let resolved = table.resolve_symbol("x").unwrap();
    assert_eq!(resolved.span.line, 3); // Local version (line 3)

    // Exit scope
    table.exit_scope().unwrap();

    // Resolve x - should get global version again
    let resolved = table.resolve_symbol("x").unwrap();
    assert_eq!(resolved.span.line, 1); // Global version (line 1)
}

#[test]
fn test_deeply_nested_scopes() {
    let mut table = SymbolTable::new();

    // Global: define a
    let a = Symbol::new("a".to_string(), SymbolKind::Variable, make_span(1, 1));
    table.define_symbol(a).unwrap();

    // Function: define b
    table.enter_scope(ScopeKind::Function);
    let b = Symbol::new("b".to_string(), SymbolKind::Variable, make_span(2, 1));
    table.define_symbol(b).unwrap();

    // Local: define c
    table.enter_scope(ScopeKind::Local);
    let c = Symbol::new("c".to_string(), SymbolKind::Variable, make_span(3, 1));
    table.define_symbol(c).unwrap();

    // Can resolve a, b, c
    assert!(table.resolve_symbol("a").is_some());
    assert!(table.resolve_symbol("b").is_some());
    assert!(table.resolve_symbol("c").is_some());

    // Exit local scope
    table.exit_scope().unwrap();
    assert!(table.resolve_symbol("a").is_some());
    assert!(table.resolve_symbol("b").is_some());
    assert!(table.resolve_symbol("c").is_none());

    // Exit function scope
    table.exit_scope().unwrap();
    assert!(table.resolve_symbol("a").is_some());
    assert!(table.resolve_symbol("b").is_none());
    assert!(table.resolve_symbol("c").is_none());
}

#[test]
fn test_in_function_detection() {
    let mut table = SymbolTable::new();

    assert!(!table.in_function());

    table.enter_scope(ScopeKind::Function);
    assert!(table.in_function());

    table.enter_scope(ScopeKind::Local);
    assert!(table.in_function()); // Still in function (nested)

    table.exit_scope().unwrap();
    assert!(table.in_function());

    table.exit_scope().unwrap();
    assert!(!table.in_function());
}

#[test]
fn test_class_scope() {
    let mut table = SymbolTable::new();

    table.enter_scope(ScopeKind::Class);
    assert_eq!(table.current_scope_kind(), ScopeKind::Class);

    let method = Symbol::new("method".to_string(), SymbolKind::Function, make_span(2, 5));
    table.define_symbol(method).unwrap();

    assert!(table.resolve_symbol("method").is_some());

    table.exit_scope().unwrap();
    assert!(table.resolve_symbol("method").is_none());
}
