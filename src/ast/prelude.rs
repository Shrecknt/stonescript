pub use super::parse::{Parse, TokenIter};
use crate::TokenTree;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SyntaxError {
    #[error("Unexpected token '{0:?}', expected {1}")]
    UnexpectedToken(TokenTree, &'static str),
    #[error("Unexpected end of file")]
    EarlyEof,
}

pub type SyntaxResult<T> = Result<T, SyntaxError>;
