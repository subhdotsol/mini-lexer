mod lexer;
mod parser;
mod pretty_printer;
mod token;

use lexer::Lexer;
use parser::Parser;
use pretty_printer::PrettyPrinter;

fn main() {
    let source = "6 + (4 * 2) / 5 - 3";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan_tokens();

    for token in tokens {
        println!("{:?}", token);
    }

    // turning the tokens into an AST

    let mut parser = Parser::new(tokens.to_vec());
    let ast = parser.parse().expect("Failed to parse");

    println!("\nAST:");
    println!("{ast:#?}");

    // pretty print the AST

    let output = PrettyPrinter::print(&ast);

    println!("\nPRETTY PRINT:");
    println!("{output}");
}
