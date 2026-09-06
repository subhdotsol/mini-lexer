use crate::token::{Token, TokenType};

#[derive(Debug)]
pub enum Expr {
    NumberLiteral(i64),

    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
}

pub struct Parser {
    tokens: Vec<Token>,
    current_position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current_position: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Expr, String> {
        let expression = self.parse_expression()?;

        self.consume(TokenType::Eof, "Expected end of input")?;

        Ok(expression)
    }

    fn parse_expression(&mut self) -> Result<Expr, String> {
        let mut expression = self.parse_term()?;

        while self.match_token(&[TokenType::Plus, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.parse_term()?;

            expression = Expr::Binary {
                left: Box::new(expression),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expression)
    }

    fn parse_term(&mut self) -> Result<Expr, String> {
        let mut expression = self.parse_factor()?;

        while self.match_token(&[TokenType::Star, TokenType::Slash]) {
            let operator = self.previous().clone();
            let right = self.parse_factor()?;

            expression = Expr::Binary {
                left: Box::new(expression),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expression)
    }

    fn parse_factor(&mut self) -> Result<Expr, String> {
        if self.match_token(&[TokenType::Number]) {
            let token = self.previous();

            let value = token
                .literal
                .parse::<i64>()
                .map_err(|_| "Invalid number".to_string())?;

            return Ok(Expr::NumberLiteral(value));
        }

        if self.match_token(&[TokenType::OpenParenthesis]) {
            let expression = self.parse_expression()?;

            self.consume(TokenType::CloseParenthesis, "Expected ')' after expression")?;

            return Ok(expression);
        }

        Err("Expected number or '('".to_string())
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current_position]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current_position - 1]
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current_position += 1;
        }

        self.previous()
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return token_type == TokenType::Eof;
        }

        self.peek().token_type == token_type
    }

    fn match_token(&mut self, types: &[TokenType]) -> bool {
        for token_type in types {
            if self.check(token_type.clone()) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn consume(&mut self, expected: TokenType, message: &str) -> Result<&Token, String> {
        if self.check(expected) {
            return Ok(self.advance());
        }

        Err(message.to_string())
    }
}
