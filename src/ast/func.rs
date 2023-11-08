use super::{parenthesized, Block, Punctuated, Type, span_of_two, ToTokens};
use crate::{
    token::{Colon, Comma, Function, Ident, Parenthesis, Static},
    Parse, SyntaxResult, TokenIter, Spanned, Span, TokenTree,
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

impl Spanned for FunctionArg {
    fn span(&self) -> Span {
        span_of_two(self.name.span(), self.ty.span())
    }
}

impl ToTokens for FunctionArg {
    fn write_into_stream(self, stream: &mut Vec<TokenTree>) {
        self.name.write_into_stream(stream);
        self.colon.write_into_stream(stream);
        self.ty.write_into_stream(stream);
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

impl Spanned for FunctionDecl {
    fn span(&self) -> Span {
        if let Some(static_token) = self.staticness {
            span_of_two(static_token.span(), self.block.span())
        } else {
            span_of_two(self.function_token.span(), self.block.span())
        }
    }
}

impl ToTokens for FunctionDecl {
    fn write_into_stream(self, stream: &mut Vec<TokenTree>) {
        if let Some(static_token) = self.staticness {
            static_token.write_into_stream(stream);
        }

        self.function_token.write_into_stream(stream);
        self.ident.write_into_stream(stream);
        self.paren.into_group(self.args).write_into_stream(stream);
        self.colon.write_into_stream(stream);
        self.return_type.write_into_stream(stream);
        self.block.write_into_stream(stream);
    }
}