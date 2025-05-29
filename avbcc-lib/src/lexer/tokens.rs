use regex::Regex;

#[derive(Debug, Clone)]
pub struct Token {
    pub(crate) ty: TokenType,
}

#[derive(Debug, Clone)]
pub enum TokenType {
    Ident(String),
    Literal(String),

    Return, // return
    KWInt,  // int
    KWLong,
    KWFloat,
    KWChar,
    Struct,
    Static,

    Comma,
    Semicolon,
    Star,
    Plus,
    Dash,
    Slash,
    Ampersand,
    Equals,
    DblEquals,
    DblAmpersand,
    SingleQuote,
    DoubleQuote,

    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBrkt,
    RightBrkt,
    LeftCarat,
    RightCarat,
}

impl TokenType {
    pub fn regex(&self) -> Regex {
        todo!()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Coordinate {
    pub(crate) line: u32,
    pub(crate) col: u32,
}

#[derive(Debug, Copy, Clone)]
pub struct Span {
    pub(crate) start: Coordinate,
    pub(crate) end: Coordinate,
}