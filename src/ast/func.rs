use super::{Block, Punctuated, Type};
use crate::{
    ast_item,
    token::{Colon, Comma, Function, Ident, Parenthesis, Static},
    Span, Spanned,
};

ast_item!(
    pub struct FunctionArg {
        name: Ident,
        colon: Colon,
        ty: Type,
    }
);

impl Spanned for FunctionArg {
    fn span(&self) -> Span {
        Span::from_start_end(self.name.span(), self.ty.span())
    }
}

ast_item!(
    pub struct FunctionDecl {
        staticness: Option<Static>,
        function_token: Function,
        ident: Ident,
        args: Parenthesis<Punctuated<FunctionArg, Comma>>,
        colon: Colon,
        return_type: Type,
        block: Block,
    }
);

impl Spanned for FunctionDecl {
    fn span(&self) -> Span {
        if let Some(static_token) = self.staticness {
            Span::from_start_end(static_token.span(), self.block.span())
        } else {
            Span::from_start_end(self.function_token.span(), self.block.span())
        }
    }
}
