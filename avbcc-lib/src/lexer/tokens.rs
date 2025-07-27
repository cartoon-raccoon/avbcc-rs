use std::fmt::Debug;
use std::convert::AsRef;
use std::any::Any;

use regex::Regex;
use strum_macros::{AsRefStr, Display, EnumIs, EnumIter, EnumProperty};
use strum::{EnumProperty};

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub ty: TokenType,
    pub start: Coordinate,
    //pub srcfile: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, AsRefStr, EnumIter, EnumIs, EnumProperty, Display)]
pub enum TokenType {
    #[strum(serialize = "unsigned", props(regex = "^unsigned\\b"))]
    Unsigned,
    #[strum(serialize = "signed", props(regex = "^signed\\b"))]
    Signed,
    #[strum(serialize = "char", props(regex = "^char\\b"))]
    KWChar,
    #[strum(serialize = "short", props(regex = "^short\\b"))]
    KWShort,
    #[strum(serialize = "int", props(regex = "^int\\b"))]
    KWInt,
    #[strum(serialize = "long", props(regex = "^long\\b"))]
    KWLong,
    #[strum(serialize = "float", props(regex = "^float\\b"))]
    KWFloat,
    #[strum(serialize = "double", props(regex = "^double\\b"))]
    KWDouble,
    #[strum(serialize = "_Bool", props(regex = "^_Bool"))]
    KWBool,
    #[strum(serialize = "struct", props(regex = "^struct\\b"))]
    Struct,
    #[strum(serialize = "enum", props(regex = "^enum\\b"))]
    Enum,
    #[strum(serialize = "union", props(regex = "^union\\b"))]
    Union,
    #[strum(serialize = "static", props(regex = "^static\\b"))]
    Static,
    #[strum(serialize = "extern", props(regex = "^extern\\b"))]
    Extern,
    #[strum(serialize = "typedef", props(regex = "^typedef\\b"))]
    Typedef,
    #[strum(serialize = "void", props(regex = "^void\\b"))]
    Void,
    #[strum(serialize = "const", props(regex = "^const\\b"))]
    Const,
    //True,
    //False,

    // control-flow tokens
    #[strum(serialize = "if", props(regex = "^if\\b"))]
    If,
    #[strum(serialize = "else", props(regex = "^else\\b"))]
    Else,
    #[strum(serialize = "switch", props(regex = "^switch\\b"))]
    Switch,
    #[strum(serialize = "case", props(regex = "^case\\b"))]
    Case,
    #[strum(serialize = "default", props(regex = "^default\\b"))]
    Default,
    #[strum(serialize = "goto", props(regex = "^goto\\b"))]
    Goto,
    #[strum(serialize = "do", props(regex = "^do\\b"))]
    Do,
    #[strum(serialize = "for", props(regex = "^for\\b"))]
    For,
    #[strum(serialize = "while", props(regex = "^while\\b"))]
    While,
    #[strum(serialize = "continue", props(regex = "^continue\\b"))]
    Continue,
    #[strum(serialize = "break", props(regex = "^break\\b"))]
    Break,
    #[strum(serialize = "return", props(regex = "^return\\b"))]
    Return,

    #[strum(serialize = "**", props(regex = "^\\*\\*"))]
    DblStar,
    #[strum(serialize = "++", props(regex = "^\\+\\+"))]
    DblPlus,
    #[strum(serialize = "--", props(regex = "^--"))]
    DblDash,
    #[strum(serialize = "->", props(regex = "^->"))]
    Arrow,
    #[strum(serialize = "//", props(regex = "^\\/\\/"))]
    DblSlash,
    #[strum(serialize = "&&", props(regex = "^&&"))]
    DblAmpersand,
    #[strum(serialize = "||", props(regex = "^\\|\\|"))]
    DblPipe,
    #[strum(serialize = "==", props(regex = "^=="))]
    DblEquals,
    #[strum(serialize = "<<", props(regex = "^<<"))]
    DblLeftCarat,
    #[strum(serialize = ">>", props(regex = "^>>"))]
    DblRightCarat,

    #[strum(serialize = ",", props(regex = "^,"))]
    Comma,
    #[strum(serialize = ";", props(regex = "^;"))]
    Semicolon,
    #[strum(serialize = ":", props(regex = "^:"))]
    Colon,
    #[strum(serialize = "!", props(regex = "^!"))]
    Bang,

    // double-able tokens
    /* place double variants before single variants,
    so if matching on them fails we fall back on the single variant */
    
    #[strum(serialize = "*", props(regex = "^\\*"))]
    Star,
    #[strum(serialize = "+", props(regex = "^\\+"))]
    Plus,
    #[strum(serialize = "-", props(regex = "^-"))]
    Dash,
    #[strum(serialize = "/", props(regex = "^\\/"))]
    Slash,
    #[strum(serialize = "&", props(regex = "^&"))]
    Ampersand,
    #[strum(serialize = "|", props(regex = "^\\|"))]
    Pipe,
    #[strum(serialize = "=", props(regex = "^="))]
    Equals,

    #[strum(serialize = "(", props(regex = "^\\("))]
    LeftParen,
    #[strum(serialize = ")", props(regex = "^\\)"))]
    RightParen,
    #[strum(serialize = "{{", props(regex = "^\\{"))]
    LeftBrace,
    #[strum(serialize = "}}", props(regex = "^\\}"))]
    RightBrace,
    #[strum(serialize = "[", props(regex = "^\\["))]
    LeftBrkt,
    #[strum(serialize = "]", props(regex = "^\\]"))]
    RightBrkt,
    #[strum(serialize = "<", props(regex = "^<"))]
    LeftCarat,
    #[strum(serialize = ">", props(regex = "^>"))]
    RightCarat,

    #[strum(to_string = "'{0}'", props(regex = "^'"))]
    SingleQuote(String),
    #[strum(to_string = "\"{0}\"", props(regex = "^\""))]
    DoubleQuote(String),

    #[strum(to_string = "{0}", props(regex = "^[a-zA-Z_][a-zA-Z0-9_]*\\b"))]
    Ident(String),
    #[strum(to_string = "{0}", props(regex = "^[0-9\\.]+\\b|^0x[0-9a-f]+\\b|^0b[0-1]+\\b"))]
    Constant(String),
}

impl TokenType {
    /// Returns a Regex that parses the TokenType.
    ///
    /// The returned regex matches a token appearing at the beginning of the haystack.
    /// It will not match any tokens further down in the string.
    ///
    /// ## Parsing quotations
    /// Quotations such as string and char literals cannot be easily parsed with a
    /// regular expression. As such, the regex returned on such tokens only detects the
    /// actual quotation mark, `"` or `'`.
    pub fn regex(&self) -> Regex {
        let s = self.get_str("regex").expect("unable to get regex for token");

        Regex::new(s).unwrap()
    }

    /// Returns the length of the token, in characters, *not* bytes.
    pub fn len(&self) -> usize {
        match self {
            TokenType::SingleQuote(s) | TokenType::DoubleQuote(s) => s.len() + 2,
            TokenType::Ident(s) | TokenType::Constant(s) => s.len(),
            otherwise => otherwise.as_ref().len()
        }
    }
}

/// A cardinal direction on a 1D line.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Direction {
    Left,
    Right,
}

/// The position of a character within a block of text.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Coordinate {
    /// The line (y-axis) that the character sits on.
    pub line: usize,
    /// The column (x-axis) along the line of the character.
    pub col: usize,
}

impl Coordinate {
    /// Create a Coordinate pointing to the first column of the first line
    /// (line and col both set to 1).
    pub fn start() -> Self {
        Self { line: 1, col: 1 }
    }

    /// Update the Coordinate's `line` relative to its current value.
    pub fn update_line_rel(&mut self, dir: Direction, delta: usize) {
        match dir {
            Direction::Left => self.line -= delta,
            Direction::Right => self.line += delta,
        }
    }

    /// Update the Coordinate's `col` relative to its current value.
    pub fn update_col_rel(&mut self, dir: Direction, delta: usize) {
        match dir {
            Direction::Left => self.col -= delta,
            Direction::Right => self.col += delta,
        }
    }

    pub fn update_line_abs(&mut self, line: usize) {
        self.line = line
    }

    pub fn update_col_abs(&mut self, col: usize) {
        self.col = col
    }
}

impl std::fmt::Display for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}:{}", self.line, self.col)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Span {
    pub start: Coordinate,
    pub end: Coordinate,
}

impl Span {
    pub fn from_coord(start: Coordinate, delta_line: usize, delta_col: usize) -> Self {
        let end = Coordinate {
            line: start.line + delta_line,
            col: start.col + delta_col,
        };

        Self { start, end }
    }
}

impl std::fmt::Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        if self.start.line == self.end.line {
            write!(f, "{}:{}-{}", self.start.line, self.start.col, self.end.col)
        } else {
            write!(f, "{}-{}", self.start, self.end)
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn check_ident_token_serialization() {
        let ident = TokenType::Ident(String::from("hello"));
        assert_eq!(ident.to_string(), String::from("hello"));
        assert_eq!(ident.len(), 5);
    }

    #[test]
    fn check_const_token_serialization() {
        let constant = TokenType::Constant(String::from("34823"));
        assert_eq!(constant.to_string(), String::from("34823"));
        assert_eq!(constant.len(), 5);
    }

    #[test]
    fn check_quotation_token_serialization() {
        let quotation = TokenType::DoubleQuote(String::from("hello"));
        assert_eq!(quotation.to_string(), String::from("\"hello\""));
        assert_eq!(quotation.len(), 7);
    }

    #[test]
    fn check_nonvariable_token_serialization() {
        let dblplus = TokenType::DblPlus;
        assert_eq!(dblplus.to_string(), String::from("++"));
        assert_eq!(dblplus.len(), 2);
    }
}
