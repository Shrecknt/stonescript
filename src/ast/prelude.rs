pub use super::parse::{TokenIter, Parse};
use crate::TokenTree;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SyntaxError {
    #[error("Unexpected token {0:?} while generating {1}")]
    UnexpectedToken(TokenTree, &'static str),
    #[error("Unexpected end of file")]
    EarlyEof,
}

pub type SyntaxResult<T> = Result<T, SyntaxError>;
