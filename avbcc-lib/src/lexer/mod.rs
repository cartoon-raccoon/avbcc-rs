use std::iter::{FusedIterator, Iterator};
use std::collections::HashMap;

use strum::IntoEnumIterator;
use regex::Regex;
use thiserror::Error;

pub mod tokens;
pub use tokens::*;

pub type LexerResult = Result<Option<Token>, LexerErr>;

#[derive(Debug, Clone)]
pub struct Lexer {
    // the string we need to tokenize
    text: String,
    // the current position on the text that the lexer is sitting at
    position: Coordinate,
    // the linear position along the text
    charidx: usize,
    // the map of regexes to match on
    regexes: HashMap<TokenType, Regex>,
}

impl Lexer {
    pub fn new<S: AsRef<str>>(text: S) -> Self {
        let regexes = TokenType::iter()
            .map(|tty| {
                let r = tty.regex();
                (tty, r)
            })
            .collect();

        Self {
            text: text.as_ref().into(),
            position: Coordinate::zero(),
            charidx: 0,
            regexes
        }
    }

    pub fn set_text<S: AsRef<str>>(&mut self, text: S) {
        self.text = text.as_ref().into();
    }


    pub fn next_token(&mut self) -> LexerResult {
        
        self.strip_whitespace();

        if self.charidx >= self.text.len() {
            // all whitespace has been stripped and we have reached the end of the text
            return Ok(None)
        }

        let haystack = self.text.get(self.charidx..)
            .expect("unable to split along valid codepoint");

        let mut ret: Option<Token> = None;

        // for each of our tokens, test if its regex matches the haystack
        for (token, regex) in self.regexes.iter() {
            if let Some(capture) = regex.find(haystack) {
                ret = match token {
                    TokenType::Ident(_) => Some(Token {
                        ty: TokenType::Ident(capture.as_str().into()),
                        start: self.position,
                    }),
                    TokenType::Constant(_) => Some(Token {
                        ty: TokenType::Constant(capture.as_str().into()),
                        start: self.position,
                    }),
                    TokenType::DoubleQuote => self.parse_quotation()?,
                    otherwise => {
                        // if the token is doubleable and it matches, use that instead
                        if let Some(dbl) = otherwise.double_token() {
                            let dblregex = self.regexes.get(&dbl).unwrap();
                            if let Some(_) = dblregex.find(haystack) {
                                ret = Some(Token {
                                    ty: dbl,
                                    start: self.position,
                                });
                                break
                            }
                        }
                        Some(Token {
                            ty: otherwise.clone(),
                            start: self.position,
                        })
                    }
                };
                break
            }
        }

        // by this point, if ret is None, we have an invalid token
        if let Some(token) = ret {
            // remove lexed token from head of text
            self.strip_token(&token);
            Ok(Some(token))
        } else {
            let src = self.grab_until_whitespace();
            let span = Span::from_coord(self.position, 0, src.len());
            Err(LexerErr {src, span})
        }
    }

    fn strip_whitespace(&mut self) {
        // update charidx and position with the first non-whitespace char
        todo!()
    }

    fn strip_token(&mut self, token: &Token) {
        todo!()
    }

    /// Parses a quotation and returns it without modifying the `Lexer`'s internal state.
    fn parse_quotation(&self) -> LexerResult {
        todo!()
    }

    fn grab_until_whitespace(&self) -> String {
        todo!()
    }
}

#[derive(Clone, Debug, Error)]
#[error("{span} - encountered unknown token `{src}`")]
pub struct LexerErr {
    src: String,
    span: Span,
}

impl Iterator for Lexer {
    type Item = LexerResult;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_token() {
            Ok(None) => None,
            Ok(valid) => Some(Ok(valid)),
            Err(err) => Some(Err(err))
        }
    }
}

impl FusedIterator for Lexer {}