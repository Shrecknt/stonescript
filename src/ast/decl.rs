use super::{Expression, Type};
use crate::{
    token::{Assign, Colon, Ident, Semicolon, Static},
    Parse, SyntaxResult, TokenIter,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Declaration {
    pub staticness: Option<Static>,
    pub ident: Ident,
    pub colon: Colon,
    pub ty: Type,
    pub value: Option<(Assign, Expression)>,
    pub semicolon: Semicolon,
}

impl Parse for Declaration {
    fn parse(token_iter: &mut TokenIter) -> SyntaxResult<Self> {
        let staticness = token_iter.parse()?;
        let ident = token_iter.parse()?;
        let colon = token_iter.parse()?;
        let ty = token_iter.parse()?;

        let value = if let Some(assign) = token_iter.parse()? {
            Some((assign, token_iter.parse()?))
        } else {
            None
        };

        let semicolon = token_iter.parse()?;

        Ok(Self {
            staticness,
            ident,
            colon,
            ty,
            value,
            semicolon,
        })
    }
}
