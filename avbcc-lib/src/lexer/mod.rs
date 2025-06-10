use std::collections::HashMap;
use std::iter::{FusedIterator, Iterator};

use regex::Regex;
use strum::IntoEnumIterator;
use thiserror::Error;

pub mod tokens;
pub use tokens::*;

pub type LexerResult = Result<Option<Token>, LexerErr>;

#[derive(Debug, Clone)]
pub struct Lexer {
    // the string we need to tokenize
    text: String,
    // the current position on the text that the lexer is sitting at
    pos: Coordinate,
    // the linear position along the text
    charidx: usize,
    // the number of chars in the text
    charcount: usize,
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
            pos: Coordinate::zero(),
            charidx: 0,
            charcount: text.as_ref().chars().count(),
            regexes,
        }
    }

    pub fn set_text<S: AsRef<str>>(&mut self, text: S) {
        self.text = text.as_ref().into();
    }

    pub fn next_token(&mut self) -> LexerResult {
        self.strip_whitespace();

        if self.charidx >= self.charcount {
            // all whitespace has been stripped and we have reached the end of the text
            return Ok(None);
        }

        let haystack = self
            .text
            .get(self.charidx..)
            .expect("unable to split along valid codepoint");

        let mut ret: Option<Token> = None;

        // for each of our tokens, test if its regex matches the haystack
        for (token, regex) in self.regexes.iter() {
            if let Some(capture) = regex.find(haystack) {
                ret = match token {
                    TokenType::Ident(_) => Some(Token {
                        ty: TokenType::Ident(capture.as_str().into()),
                        start: self.pos,
                    }),
                    TokenType::Constant(_) => Some(Token {
                        ty: TokenType::Constant(capture.as_str().into()),
                        start: self.pos,
                    }),
                    TokenType::DoubleQuote(_) => self.parse_quotation()?,
                    otherwise => {
                        // if the token is doubleable and it matches, use that instead
                        if let Some(dbl) = otherwise.double_token() {
                            let dblregex = self.regexes.get(&dbl).unwrap();
                            if dblregex.find(haystack).is_some() {
                                ret = Some(Token {
                                    ty: dbl,
                                    start: self.pos,
                                });
                                break;
                            }
                        }
                        Some(Token {
                            ty: otherwise.clone(),
                            start: self.pos,
                        })
                    }
                };
                break;
            }
        }

        // by this point, if ret is None, we have an invalid token
        if let Some(token) = ret {
            // remove lexed token from head of text
            self.update_state(&token);
            Ok(Some(token))
        } else {
            let src = self.grab_until_whitespace();
            let span = Span::from_coord(self.pos, 0, src.chars().count());
            Err(LexerErr { src, span })
        }
    }

    fn strip_whitespace(&mut self) {
        // update charidx and position with the first non-whitespace char
        todo!()
    }

    /// Updates the lexer's internal state, tracking the current position and charidx.
    fn update_state(&mut self, token: &Token) {
        // update charidx value
        self.charidx += token.ty.len();
        // update current position
        match &token.ty {
            TokenType::SingleQuote(s) | TokenType::DoubleQuote(s) => {
                for c in s.chars() {
                    if c == '\n' {
                        // if the character is newline, reset col and increment line
                        self.pos.line += 1;
                        self.pos.col = 0;
                    } else {
                        // else, increment col
                        self.pos.col += 1;
                    }
                }
            }
            // other should not contain newlines, so we can
            other => {
                self.pos.update_col_rel(Direction::Right, other.len());
            }
        }
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
            Err(err) => Some(Err(err)),
        }
    }
}

impl FusedIterator for Lexer {}
