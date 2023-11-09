use super::{Expression, ToTokens, Type};
use crate::{
    token::{Assign, Colon, Ident, Semicolon, Static, Let},
    Parse, Span, Spanned, SyntaxResult, TokenIter, TokenTree, SyntaxError,
};

#[derive(Debug, Clone, PartialEq)]
pub enum DeclStart {
    Static(Static),
    Let(Let),
}

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

impl Spanned for DeclStart {
    fn span(&self) -> Span {
        match self {
            Self::Static(static_token) => static_token.span(),
            Self::Let(let_token) => let_token.span(),
        }
    }
}

impl ToTokens for DeclStart {
    fn write_into_stream(self, stream: &mut Vec<TokenTree>) {
        match self {
            Self::Static(static_token) => static_token.write_into_stream(stream),
            Self::Let(let_token) => let_token.write_into_stream(stream),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Declaration {
    pub start_token: DeclStart,
    pub ident: Ident,
    pub colon: Colon,
    pub ty: Type,
    pub value: Option<(Assign, Expression)>,
    pub semicolon: Semicolon,
}

impl Parse for Declaration {
    fn parse(token_iter: &mut TokenIter) -> SyntaxResult<Self> {
        let start_token = token_iter.parse()?;
        let ident = token_iter.parse()?;
        let colon = token_iter.parse()?;
        let ty = token_iter.parse()?;

        let value = if let Some(assign) = token_iter.parse()? {
            let expr = token_iter.parse()?;
            Some((assign, expr))
        } else {
            None
        };

        let semicolon = token_iter.parse()?;

        Ok(Self {
            start_token,
            ident,
            colon,
            ty,
            value,
            semicolon,
        })
    }
}

impl Spanned for Declaration {
    fn span(&self) -> Span {
        Span::from_start_end(self.start_token.span(), self.semicolon.span())
    }
}

impl ToTokens for Declaration {
    fn write_into_stream(self, stream: &mut Vec<crate::TokenTree>) {
        self.start_token.write_into_stream(stream);
        self.ident.write_into_stream(stream);
        self.colon.write_into_stream(stream);
        self.ty.write_into_stream(stream);

        if let Some((assign, value)) = self.value {
            assign.write_into_stream(stream);
            value.write_into_stream(stream);
        }

        self.semicolon.write_into_stream(stream)
    }
}
