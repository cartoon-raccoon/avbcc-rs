use thiserror::Error;

pub mod lexer;
pub mod parser;

pub mod reexports {
    pub use regex;
}

use lexer::LexerErr;
use parser::ParseErr;

pub struct Avbcc {
    
}

#[derive(Debug, Error)]
pub enum AvbccErr {
    #[error(transparent)]
    LexerErr(LexerErr),
    #[error(transparent)]
    ParseErr(ParseErr),
}
