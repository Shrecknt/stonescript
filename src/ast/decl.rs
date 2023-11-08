use super::{span_of_two, Expression, ToTokens, Type};
use crate::{
    token::{Assign, Colon, Ident, Semicolon, Static},
    Parse, Span, Spanned, SyntaxResult, TokenIter,
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
            let expr = token_iter.parse()?;
            Some((assign, expr))
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

impl Spanned for Declaration {
    fn span(&self) -> Span {
        if let Some(static_token) = self.staticness {
            span_of_two(static_token.span(), self.semicolon.span())
        } else {
            span_of_two(self.ident.span(), self.semicolon.span())
        }
    }
}

impl ToTokens for Declaration {
    fn write_into_stream(self, stream: &mut Vec<crate::TokenTree>) {
        if let Some(static_token) = self.staticness {
            static_token.write_into_stream(stream);
        }

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
