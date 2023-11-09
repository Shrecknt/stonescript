use crate::{
    ast::{Block, Declaration, Expression, Statement},
    ast_item,
    token::{For, Parenthesis, Semicolon}, Spanned, Span,
};

ast_item!(
    pub struct ForLoop {
        for_token: For,
        inner: (Parenthesis, (Declaration, (Expression, Semicolon), Statement)),
        block: Block
    }
);

impl Spanned for ForLoop {
    fn span(&self) -> Span {
        Span::from_start_end(self.for_token.span(), self.block.span())
    }
}