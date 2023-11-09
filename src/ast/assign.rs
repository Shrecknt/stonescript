use super::{Expression, ToTokens};
use crate::{
    token::{Assign, Ident, Semicolon},
    Parse, SyntaxResult, TokenIter, Spanned, Span, TokenTree, 
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

impl Spanned for Assignment {
    fn span(&self) -> Span {
        Span::from_start_end(self.variable_name.span(), self.semicolon.span())
    }
}

impl ToTokens for Assignment {
    fn write_into_stream(self, stream: &mut Vec<TokenTree>) {
        self.variable_name.write_into_stream(stream);
        self.assign.write_into_stream(stream);
        self.value.write_into_stream(stream);
        self.semicolon.write_into_stream(stream);
    }
}