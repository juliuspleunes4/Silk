use silk_parser::Parser;
use silk_semantic::SemanticAnalyzer;

fn main() {
    let source = r#"
def f() -> int:
    return "hello"
"#;
    let program = Parser::parse(source).expect("Failed to parse");
    
    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&program);
    
    if let Err(errors) = result {
        for error in &errors {
            println!("Error: {}", error);
        }
    } else {
        println!("No errors found");
    }
}
