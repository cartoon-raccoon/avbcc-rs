use std::iter::Iterator;

pub mod tokens;
pub use tokens::*;

#[derive(Debug, Clone)]
pub struct Lexer {
    text: String,
    position: Coordinate,
}

impl Lexer {
    pub fn new(text: String) -> Self {
        Self {
            text,
            position: Coordinate {line: 0, col: 0}
        }
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }

    pub fn next_token(&mut self) -> Option<Token> {
        None
    }

}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}