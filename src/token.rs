#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Number,
    Plus,
    Minus,
    Star,
    Slash,
    OpenParenthesis,
    CloseParenthesis,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, literal: impl Into<String>) -> Self {
        Token {
            token_type,
            literal: literal.into(),
        }
    }
}
