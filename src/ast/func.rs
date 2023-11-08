use super::{parenthesized, Block, Punctuated, Type};
use crate::{
    token::{Colon, Comma, Function, Ident, Parenthesis, Static},
    Parse, SyntaxResult, TokenIter,
};

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionArg {
    pub name: Ident,
    pub colon: Colon,
    pub ty: Type,
}

impl Parse for FunctionArg {
    fn parse(token_iter: &mut TokenIter) -> SyntaxResult<Self> {
        let name = token_iter.parse()?;
        let colon = token_iter.parse()?;
        let ty = token_iter.parse()?;

        Ok(Self { name, colon, ty })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDecl {
    pub staticness: Option<Static>,
    pub function_token: Function,
    pub ident: Ident,
    pub paren: Parenthesis,
    pub args: Punctuated<FunctionArg, Comma>,
    pub colon: Colon,
    pub return_type: Type,
    pub block: Block,
}

impl Parse for FunctionDecl {
    fn parse(token_iter: &mut TokenIter) -> SyntaxResult<Self> {
        let staticness = token_iter.parse()?;
        let function_token = token_iter.parse()?;
        let ident = token_iter.parse()?;
        let (paren, args) = parenthesized(token_iter.parse()?)?;
        let colon = token_iter.parse()?;
        let return_type = token_iter.parse()?;
        let block = token_iter.parse()?;

        Ok(Self {
            staticness,
            function_token,
            ident,
            paren,
            args,
            colon,
            return_type,
            block,
        })
    }
}
