use crate::parser::Expr;
use crate::token::TokenType;

pub struct PrettyPrinter;

impl PrettyPrinter {
    pub fn print(expr: &Expr) -> String {
        match expr {
            Expr::NumberLiteral(value) => value.to_string(),

            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left = Self::print(left);
                let right = Self::print(right);

                let operator = match operator.token_type {
                    TokenType::Plus => "+",
                    TokenType::Minus => "-",
                    TokenType::Star => "*",
                    TokenType::Slash => "/",

                    _ => panic!("Invalid binary operator"),
                };

                format!("({left} {operator} {right})")
            }
        }
    }
}
