use silk_parser::Parser;
let source = "result = 10 % 3";
let program = Parser::parse(source).unwrap();
println!("Parsed OK: {:?}", program);
