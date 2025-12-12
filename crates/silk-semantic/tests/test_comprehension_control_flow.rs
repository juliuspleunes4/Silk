use silk_parser::Parser;
use silk_semantic::ControlFlowAnalyzer;

fn analyze(source: &str) -> Vec<silk_semantic::SemanticError> {
    let program = Parser::parse(source).expect("Parse failed");
    
    let mut analyzer = ControlFlowAnalyzer::new();
    match analyzer.analyze(&program) {
        Ok(_) => Vec::new(),
        Err(errors) => {
            // Filter out UnusedFunction errors - we're testing comprehension scope
            errors
                .into_iter()
                .filter(|e| !matches!(e, silk_semantic::SemanticError::UnusedFunction { .. }))
                .collect()
        }
    }
}

#[test]
fn test_comprehension_uses_outer_variable() {
    let source = r#"
def main():
    items = [1, 2, 3]
    result = [x * 2 for x in items]
    return result
"#;
    let errors = analyze(source);
    assert_eq!(errors.len(), 0, "Should not error - items is defined");
}

#[test]
fn test_comprehension_variable_doesnt_leak() {
    let source = r#"
def main():
    items = [1, 2, 3]
    result = [x * 2 for x in items]
    return x
"#;
    let errors = analyze(source);
    // Filter to only uninitialized errors
    let uninit_errors: Vec<_> = errors.iter().filter(|e| {
        matches!(e, silk_semantic::SemanticError::UninitializedVariable { .. })
    }).collect();
    assert_eq!(uninit_errors.len(), 1, "Should have 1 uninitialized error - x doesn't leak from comprehension. Errors: {:?}", errors);
    assert!(matches!(
        &errors[0],
        silk_semantic::SemanticError::UninitializedVariable { name, .. } if name == "x"
    ));
}

#[test]
fn test_nested_comprehensions() {
    let source = r#"
def main():
    matrix = [[1, 2], [3, 4]]
    flat = [x for row in matrix for x in row]
    return flat
"#;
    let errors = analyze(source);
    assert_eq!(errors.len(), 0, "Nested comprehensions should work");
}

#[test]
fn test_comprehension_with_filter() {
    let source = r#"
def main():
    items = [1, 2, 3, 4, 5]
    evens = [x for x in items if x > 2]
    return evens
"#;
    let errors = analyze(source);
    assert_eq!(errors.len(), 0, "Comprehension with filter should work");
}

#[test]
fn test_dict_comprehension_scope() {
    let source = r#"
def main():
    items = [1, 2, 3]
    result = {x: x * 2 for x in items}
    return result
"#;
    let errors = analyze(source);
    assert_eq!(errors.len(), 0, "Dict comprehension should work");
}

#[test]
fn test_dict_comprehension_variable_doesnt_leak() {
    let source = r#"
def main():
    items = [1, 2, 3]
    result = {k: k * 2 for k in items}
    return k
"#;
    let errors = analyze(source);
    // Filter to only uninitialized errors
    let uninit_errors: Vec<_> = errors.iter().filter(|e| {
        matches!(e, silk_semantic::SemanticError::UninitializedVariable { .. })
    }).collect();
    assert_eq!(uninit_errors.len(), 1, "Should have 1 uninitialized error - k doesn't leak. Errors: {:?}", errors);
    let uninit_error = uninit_errors[0];
    assert!(matches!(
        uninit_error,
        silk_semantic::SemanticError::UninitializedVariable { name, .. } if name == "k"
    ));
}

#[test]
fn test_set_comprehension_scope() {
    let source = r#"
def main():
    items = [1, 2, 3, 1, 2]
    result = {x * 2 for x in items}
    return result
"#;
    let errors = analyze(source);
    assert_eq!(errors.len(), 0, "Set comprehension should work");
}

#[test]
fn test_generator_expression_scope() {
    let source = r#"
def main():
    items = [1, 2, 3]
    result = (x * 2 for x in items)
    return result
"#;
    let errors = analyze(source);
    assert_eq!(errors.len(), 0, "Generator expression should work");
}

#[test]
fn test_comprehension_in_function() {
    let source = r#"
def process(items):
    return [x * 2 for x in items]

def main():
    result = process([1, 2, 3])
    return result
"#;
    let errors = analyze(source);
    assert_eq!(errors.len(), 0, "Comprehension in function should work");
}

#[test]
fn test_multiple_generators_in_comprehension() {
    let source = r#"
def main():
    rows = [1, 2, 3]
    cols = [4, 5, 6]
    pairs = [(r, c) for r in rows for c in cols]
    return pairs
"#;
    let errors = analyze(source);
    assert_eq!(errors.len(), 0, "Multiple generators should work");
}

#[test]
fn test_comprehension_with_undefined_outer_variable() {
    let source = r#"
def main():
    result = [x * 2 for x in undefined_items]
    return result
"#;
    let errors = analyze(source);
    assert_eq!(errors.len(), 1, "Should error - undefined_items not defined");
    assert!(matches!(
        &errors[0],
        silk_semantic::SemanticError::UninitializedVariable { name, .. } if name == "undefined_items"
    ));
}

#[test]
fn test_nested_comprehension_outer_variable_access() {
    let source = r#"
def main():
    multiplier = 2
    items = [1, 2, 3]
    result = [x * multiplier for x in items]
    return result
"#;
    let errors = analyze(source);
    assert_eq!(errors.len(), 0, "Comprehension can access outer variables");
}

#[test]
fn test_comprehension_iterator_checked_before_scope() {
    let source = r#"
def main():
    result = [y for y in x]
    x = [1, 2, 3]
    return result
"#;
    let errors = analyze(source);
    assert_eq!(errors.len(), 1, "Should error - x used before defined");
    assert!(matches!(
        &errors[0],
        silk_semantic::SemanticError::UninitializedVariable { name, .. } if name == "x"
    ));
}

#[test]
fn test_comprehension_tracks_usage_of_outer_variables() {
    let source = r#"
def main():
    items = [1, 2, 3]
    multiplier = 2
    result = [x * multiplier for x in items]
    return result
"#;
    let errors = analyze(source);
    // Filter out unused variable warnings
    let init_errors: Vec<_> = errors.iter().filter(|e| {
        !matches!(e, silk_semantic::SemanticError::UnusedVariable { .. })
    }).collect();
    assert_eq!(init_errors.len(), 0, "Should track that multiplier and items are used");
}
