/// Tests for annotated assignment (AnnAssign) semantic analysis
use silk_parser::Parser;
use silk_semantic::types::Type;
use silk_semantic::SemanticAnalyzer;

#[test]
fn test_ann_assign_creates_symbol() {
    let source = "x: int = 10";
    let program = Parser::parse(source).unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();

    // Verify symbol exists
    let symbol = analyzer.symbol_table().resolve_symbol("x");
    assert!(symbol.is_some(), "Symbol 'x' should exist");
}

#[test]
fn test_ann_assign_symbol_has_correct_type() {
    let source = "x: int = 10";
    let program = Parser::parse(source).unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();

    // Verify symbol has correct type from annotation
    let symbol = analyzer.symbol_table().resolve_symbol("x").unwrap();
    assert_eq!(symbol.ty, Type::Int);
}

#[test]
fn test_ann_assign_without_value_creates_symbol() {
    let source = "x: int";
    let program = Parser::parse(source).unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();

    // Verify symbol exists even without value
    let symbol = analyzer.symbol_table().resolve_symbol("x");
    assert!(symbol.is_some(), "Symbol 'x' should exist");
}

#[test]
fn test_ann_assign_without_value_has_correct_type() {
    let source = "y: str";
    let program = Parser::parse(source).unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();

    // Verify type from annotation
    let symbol = analyzer.symbol_table().resolve_symbol("y").unwrap();
    assert_eq!(symbol.ty, Type::Str);
}

#[test]
fn test_ann_assign_with_value_symbol_type() {
    let source = "pi: float = 3.14";
    let program = Parser::parse(source).unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();

    // Verify type comes from annotation, not value inference
    let symbol = analyzer.symbol_table().resolve_symbol("pi").unwrap();
    assert_eq!(symbol.ty, Type::Float);
}

#[test]
fn test_ann_assign_in_function_scope() {
    let source = r#"
def my_func():
    x: int = 42
    return x
"#;
    let program = Parser::parse(source).unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    // Symbol should exist in function scope, not global
    // We can verify no errors occurred by unwrap()
    analyzer.analyze(&program).unwrap();
}

#[test]
fn test_multiple_ann_assigns() {
    let source = r#"
x: int = 10
y: str = "hello"
z: float
"#;
    let program = Parser::parse(source).unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();

    // Verify all symbols exist with correct types
    let x_symbol = analyzer.symbol_table().resolve_symbol("x").unwrap();
    assert_eq!(x_symbol.ty, Type::Int);

    let y_symbol = analyzer.symbol_table().resolve_symbol("y").unwrap();
    assert_eq!(y_symbol.ty, Type::Str);

    let z_symbol = analyzer.symbol_table().resolve_symbol("z").unwrap();
    assert_eq!(z_symbol.ty, Type::Float);
}

#[test]
fn test_ann_assign_bool_type() {
    let source = "flag: bool = True";
    let program = Parser::parse(source).unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("flag").unwrap();
    assert_eq!(symbol.ty, Type::Bool);
}

#[test]
fn test_ann_assign_generic_type() {
    // Generic types like list[int] are now properly resolved
    // Test that generic types are now properly resolved
    let source = "items: list[int] = []";
    let program = Parser::parse(source).unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();

    let symbol = analyzer.symbol_table().resolve_symbol("items").unwrap();
    // Generic types are now properly resolved
    assert_eq!(symbol.ty, Type::List(Box::new(Type::Int)));
}
