use thiserror::Error;

pub mod ast;

pub struct Parser {

}

#[derive(Debug, Error)]
#[error("")]
pub struct ParseErr {

}
