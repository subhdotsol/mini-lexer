use crate::token::{Token, TokenType};

pub struct Lexer {
    input: Vec<char>,
    current_position: usize,
    tokens: Vec<Token>,
}

impl Lexer {
    // Creates a new Lexer for the given input string.
    //
    // Example input which i am taking is  `"2*4+(5-3)"`
    // - Converts the string into a `Vec<char>` so we can index into it easily.
    // - `current_position` starts at 0 (the first character, `'2'`).
    // - `tokens` starts empty; it will be filled by `scan_tokens()`.
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            current_position: 0,
            tokens: Vec::new(),
        }
    }

    // Scans the entire input into a list of tokens and returns them.
    //
    // This is the main entry point: it keeps calling `scan_next_token()`
    // until we run out of input, then returns all collected tokens.
    //
    // For `"2*4+(5-3)"` it produces (in order):
    // `Number("2")`, `Star`, `Number("4")`, `Plus`, `OpenParenthesis`,
    // `Number("5")`, `Minus`, `Number("3")`, `CloseParenthesis`.
    pub fn scan_tokens(&mut self) -> &[Token] {
        // While we haven't consumed the whole input yet, scan one more token.
        while !self.is_at_end() {
            self.scan_next_token();
        }

        // Return a slice reference to all collected tokens.
        &self.tokens
    }

    // Returns `true` when there are no more characters left to read,
    // i.e. `current_position` has reached (or passed) the end of the input.
    fn is_at_end(&self) -> bool {
        self.current_position >= self.input.len()
    }

    // Consumes and returns the character at the current position, then
    // moves the position forward by one.
    //
    // Example: at position 0 of `"2*4..."` it returns `'2'` and
    // `current_position` becomes 1. `advance()` is how we "eat" characters.
    fn advance(&mut self) -> char {
        let character = self.input[self.current_position];
        self.current_position += 1;
        character
    }

    // Looks at the character at the current position WITHOUT consuming it.
    //
    // Returns `None` if we're out of bounds. Not used in this file yet, but
    // it's the standard "lookahead" helper needed for multi-char tokens
    // (e.g. detecting the next digit of a number).
    fn peek(&self) -> Option<char> {
        self.input.get(self.current_position).copied()
    }

    // Reads the next single token from the input and stores it in `tokens`.
    //
    // It does this by consuming one character with `advance()`, then deciding
    // what kind of token it is, based on `match`.
    //
    // For the first call on `"2*4+(5-3)"`: `advance()` returns `'2'`, which
    // is a digit, so we delegate to `scan_number()`.
    fn scan_next_token(&mut self) {
        let character = self.advance();

        match character {
            // Single-character operators/parens: push the matching token directly.
            '(' => self.add_token(TokenType::OpenParenthesis),
            ')' => self.add_token(TokenType::CloseParenthesis),
            '+' => self.add_token(TokenType::Plus),
            '-' => self.add_token(TokenType::Minus),
            '*' => self.add_token(TokenType::Star),
            '/' => self.add_token(TokenType::Slash),

            // Digits: numbers can span multiple characters (e.g. "12"), so
            // hand off to scan_number() to consume the whole run.
            c if c.is_ascii_digit() => self.scan_number(),

            // Whitespace is insignificant; just skip it (do nothing).
            ' ' | '\n' | '\t' | '\r' => {
                // Ignore whitespace
            }

            // Anything else is invalid input for this grammar; abort loudly.
            c => {
                panic!("Unexpected character: {c}");
            }
        }
    }

    // Reads a whole multi-digit number, starting from the digit that was
    // already consumed, and appends it as a `Number` token.
    //
    // Example: handling `"2*4..."` for the `'2'` (only one digit here), but
    // for something like `"123+"` this would swallow `'1'`, `'2'`, `'3'`.
    //
    // `start` is the index of the first digit. Because `advance()` was
    // already called once in `scan_next_token()`, the first digit is at
    // `current_position - 1`.
    fn scan_number(&mut self) {
        let start = self.current_position - 1;

        // Keep consuming characters as long as they're still digits.
        while let Some(c) = self.peek() {
            // Stop as soon as we hit a non-digit (operator, paren, space...).
            if !c.is_ascii_digit() {
                break;
            }

            self.advance();
        }

        // Collect the slice from the first digit up to the last one into a String.
        let literal: String = self.input[start..self.current_position].iter().collect();

        // Store it as a Number token with that literal value.
        self.tokens.push(Token::new(TokenType::Number, literal));
    }

    // Convenience helper: builds a token with no literal value (for operators
    // and parentheses) and pushes it onto the token list.
    fn add_token(&mut self, token_type: TokenType) {
        self.tokens.push(Token::new(token_type, ""));
    }
}
