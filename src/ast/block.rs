use super::{braced, Statement};
use crate::{token::Brace, Parse, SyntaxResult, TokenIter};

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub brace: Brace,
    pub contents: Vec<Statement>,
}

impl Parse for Block {
    fn parse(token_iter: &mut TokenIter) -> SyntaxResult<Self> {
        let (brace, contents) = braced(token_iter.parse()?)?;
        Ok(Self { brace, contents })
    }
}
