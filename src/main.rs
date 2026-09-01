mod lexer;
mod token;

use lexer::Lexer;

fn main() {
    let source = "6 + (4 * 2) / 5 - 3";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan_tokens();

    for token in tokens {
        println!("{:?}", token);
    }
}
