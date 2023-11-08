use super::{braced, Statement};
use crate::{
    token::{Brace, ToTokenTree},
    Parse, Span, Spanned, SyntaxResult, TokenIter, TokenTree,
};

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

impl Spanned for Block {
    fn span(&self) -> Span {
        self.brace.span()
    }
}

impl ToTokenTree for Block {
    fn to_token_tree(self) -> TokenTree {
        self.brace.into_group(self.contents).to_token_tree()
    }
}
