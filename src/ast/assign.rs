use super::Expression;
use crate::{
    token::{Assign, Ident, Semicolon},
    Parse, SyntaxResult, TokenIter,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Assignment {
    pub variable_name: Ident,
    pub assign: Assign,
    pub value: Expression,
    pub semicolon: Semicolon,
}

impl Parse for Assignment {
    fn parse(token_iter: &mut TokenIter) -> SyntaxResult<Self> {
        let variable_name = token_iter.parse()?;
        let assign = token_iter.parse()?;
        let value = token_iter.parse()?;
        let semicolon = token_iter.parse()?;

        Ok(Assignment {
            variable_name,
            assign,
            value,
            semicolon,
        })
    }
}
