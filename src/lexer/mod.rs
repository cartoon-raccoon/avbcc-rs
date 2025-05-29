use std::iter::Iterator;

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
            position: Coordinate {0, 0}
        }
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }

}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Result<Option<Self::Item>> {
        
    }
}