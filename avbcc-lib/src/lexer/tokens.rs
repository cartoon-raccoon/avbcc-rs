use std::fmt::{Display};

use regex::Regex;
use strum_macros::{AsRefStr, EnumIter};

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub ty: TokenType,
    pub start: Coordinate,
}

#[derive(Debug, Clone, PartialEq, EnumIter, Eq, Hash, AsRefStr)]
pub enum TokenType {
    Ident(String),
    Constant(String),

    // keywords
    #[strum(serialize="unsigned")]
    Unsigned,
    #[strum(serialize="signed")]
    Signed,
    #[strum(serialize="char")]
    KWChar,
    #[strum(serialize="short")]
    KWShort,
    #[strum(serialize="int")]
    KWInt,  // int
    #[strum(serialize="long")]
    KWLong,
    #[strum(serialize="float")]
    KWFloat,
    #[strum(serialize="double")]
    KWDouble,
    #[strum(serialize="_Bool")]
    KWBool,
    #[strum(serialize="struct")]
    Struct,
    #[strum(serialize="enum")]
    Enum,
    #[strum(serialize="union")]
    Union,
    #[strum(serialize="static")]
    Static,
    #[strum(serialize="extern")]
    Extern,
    #[strum(serialize="typedef")]
    Typedef,
    #[strum(serialize="void")]
    Void,
    #[strum(serialize="const")]
    Const,
    //True,
    //False,

    // control-flow tokens
    #[strum(serialize="if")]
    If,
    #[strum(serialize="else")]
    Else,
    #[strum(serialize="switch")]
    Switch,
    #[strum(serialize="case")]
    Case,
    #[strum(serialize="default")]
    Default,
    #[strum(serialize="goto")]
    Goto,
    #[strum(serialize="do")]
    Do,
    #[strum(serialize="for")]
    For,
    #[strum(serialize="while")]
    While,
    #[strum(serialize="continue")]
    Continue,
    #[strum(serialize="break")]
    Break,
    #[strum(serialize="return")]
    Return,

    #[strum(serialize=",")]
    Comma,
    #[strum(serialize=";")]
    Semicolon,
    #[strum(serialize=":")]
    Colon,
    #[strum(serialize="!")]
    Bang,

    // double-able tokens
    #[strum(serialize="*")]
    Star,
    #[strum(serialize="**")]
    DblStar,
    #[strum(serialize="+")]
    Plus,
    #[strum(serialize="++")]
    DblPlus,
    #[strum(serialize="-")]
    Dash,
    #[strum(serialize="+--")]
    DblDash,
    #[strum(serialize="/")]
    Slash,
    #[strum(serialize="//")]
    DblSlash,
    #[strum(serialize="&")]
    Ampersand,
    #[strum(serialize="&&")]
    DblAmpersand,
    #[strum(serialize="|")]
    Pipe,
    #[strum(serialize="||")]
    DblPipe,
    #[strum(serialize="=")]
    Equals,
    #[strum(serialize="==")]
    DblEquals,

    #[strum(serialize="'")]
    SingleQuote,
    #[strum(serialize="\"")]
    DoubleQuote,

    #[strum(serialize="(")]
    LeftParen,
    #[strum(serialize=")")]
    RightParen,
    #[strum(serialize="{")]
    LeftBrace,
    #[strum(serialize="}")]
    RightBrace,
    #[strum(serialize="[")]
    LeftBrkt,
    #[strum(serialize="]")]
    RightBrkt,
    #[strum(serialize="<")]
    LeftCarat,
    #[strum(serialize=">")]
    RightCarat,
    #[strum(serialize="<<")]
    DblLeftCarat,
    #[strum(serialize=">>")]
    DblRightCarat,
}

impl TokenType {
    pub fn regex(&self) -> Regex {
        let s = match self {
            Self::Ident(_) => "^[a-zA-Z_]\\w*\\b",
            Self::Constant(_) => "^[0-9]+\\b",
            
            Self::Unsigned => "^unsigned\\b",
            Self::Signed => "^signed\\b",
            Self::KWChar => "^char\\b",
            Self::KWShort => "^short\\b",
            Self::KWInt => "^int\\b",
            Self::KWLong => "^long\\b",
            Self::KWFloat => "^float\\b",
            Self::KWDouble => "^double\\b",
            Self::KWBool => "^_Bool",
            Self::Struct => "^struct\\b",
            Self::Enum => "^enum\\b",
            Self::Union => "^union\\b",
            Self::Static => "^static\\b",
            Self::Extern => "^extern\\b",
            Self::Typedef => "^typedef\\b",
            Self::Void => "^void\\b",
            Self::Const => "^const\\b",
            
            Self::If => "^if\\b",
            Self::Else => "^else\\b",
            Self::Switch => "^switch\\b",
            Self::Case => "^case\\b",
            Self::Default => "^default\\b",
            Self::Goto => "^goto\\b",
            Self::Do => "^do\\b",
            Self::For => "^for\\b",
            Self::While => "^while\\b",
            Self::Continue => "^continue\\b",
            Self::Break => "^break\\b",
            Self::Return => "^return\\b",

            Self::Comma => "^,",
            Self::Semicolon => "^;",
            Self::Colon => "^:",
            Self::Bang=>"^!",

            Self::Star => "^\\*",
            Self::DblStar => "^\\*\\*",
            Self::Plus => "^\\+",
            Self::DblPlus => "^\\+\\+",
            Self::Dash => "^-",
            Self::DblDash => "^--",
            Self::Slash => "^\\/",
            Self::DblSlash => "^\\/\\/",
            Self::Pipe => "^\\|",
            Self::DblPipe => "^\\|\\|",
            Self::Ampersand => "^&",
            Self::DblAmpersand => "^&&",
            Self::Equals => "^=",
            Self::DblEquals => "^==",

            Self::SingleQuote => "^'",
            Self::DoubleQuote => "^\"",

            Self::LeftParen => "^\\(",
            Self::RightParen => "^\\)",
            Self::LeftBrace => "^{",
            Self::RightBrace => "^}",
            Self::LeftBrkt => "^\\[",
            Self::RightBrkt => "^\\]",
            Self::LeftCarat => "^<",
            Self::DblLeftCarat => "^<<",
            Self::RightCarat => "^>",
            Self::DblRightCarat => "^>>",
        };

        Regex::new(s).unwrap()
    }

    /// Returns the double version of the token, if such a token exists.
    /// If no such double version exists, returns `None`.
    /// 
    /// NOTE: `TokenType::SingleQuote` is not considered a double quote, as
    /// it is a separate Unicode codepoint from `"`, not a double-up of `'`.
    /// As such, calling `double_up` on `TokenType::SingleQuote` will return
    /// `None`.
    pub fn double_token(&self) -> Option<Self> {
        match self {
            Self::Star       => Some(Self::DblStar),
            Self::Plus       => Some(Self::DblPlus),
            Self::Dash       => Some(Self::DblDash),
            Self::Slash      => Some(Self::DblSlash),
            Self::Ampersand  => Some(Self::DblAmpersand),
            Self::Pipe       => Some(Self::DblPipe),
            Self::Equals     => Some(Self::DblEquals),
            Self::LeftCarat  => Some(Self::DblLeftCarat),
            Self::RightCarat => Some(Self::DblRightCarat),
            _                => None
        }
    }
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::Ident(s) | Self::Constant(s) => {
                write!(f, "{}", s)
            },
            other => {
                write!(f, "{}", other.as_ref())
            }
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
    /// Create a zeroed Coordinate (`line` and `col` set to 0).
    pub fn zero() -> Self {
        Self {line: 0, col: 0}
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

impl Display for Coordinate{
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

        Self {start, end}
    }
}

impl Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        if self.start.line == self.end.line {
            write!(f, "{}:{}-{}", self.start.line, self.start.col, self.end.col)
        } else {
            write!(f, "{}-{}", self.start, self.end)
        }
    }
}