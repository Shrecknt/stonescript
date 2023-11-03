use crate::{token::Brace, Parse, TokenIter, SyntaxResult};
use super::Statement;

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub brace: Brace,
    pub contents: Vec<Statement>,
}

impl Parse for Block {
    fn parse(token_iter: &mut TokenIter) -> SyntaxResult<Self> {
        let (brace, contents) = token_iter.braced()?;
        Ok(Self {
            brace,
            contents,
        })
    }
}