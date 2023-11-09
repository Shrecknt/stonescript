use super::{Expression, Type};
use crate::{
    token::{Assign, Colon, Ident, Semicolon, Static, Let},
    Parse, Span, Spanned, SyntaxResult, TokenIter, TokenTree, SyntaxError, ast_item,
};

ast_item!(
    pub enum DeclStart {
        Static(Static),
        Let(Let)
    }
);

impl Parse for DeclStart {
    fn parse(token_iter: &mut TokenIter) -> SyntaxResult<Self> {
        match token_iter.expect_peek()? {
            TokenTree::Ident(ident) => {
                if Static::is_ident(ident) {
                    Ok(Self::Static(token_iter.parse()?))
                } else if Let::is_ident(ident) {
                    Ok(Self::Let(token_iter.parse()?))
                } else {
                    Err(SyntaxError::UnexpectedToken(token_iter.expect_consume()?, "static or let"))
                }
            }
            _ => Err(SyntaxError::UnexpectedToken(token_iter.expect_consume()?, "static or let"))
        }
    }
}

ast_item!(
    pub struct Declaration {
        start_token: DeclStart,
        ident: Ident,
        colon: Colon,
        ty: Type,
        value: Option<(Assign, Expression)>,
        semicolon: Semicolon
    }
);

impl Spanned for Declaration {
    fn span(&self) -> Span {
        Span::from_start_end(self.start_token.span(), self.semicolon.span())
    }
}