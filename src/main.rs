mod lexer;
mod parser;
mod token;

use lexer::Lexer;
use parser::Parser;

fn main() {
    let source = "6 + (4 * 2) / 5 - 3";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan_tokens();

    for token in tokens {
        println!("{:?}", token);
    }

    let mut parser = Parser::new(tokens.to_vec());
    let ast = parser.parse().expect("Failed to parse");

    println!("\nAST:");
    println!("{ast:#?}");
}
