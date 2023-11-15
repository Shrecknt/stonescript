use crate::{
    ast_item,
    hir::{Block, Expression},
    token::{Parenthesis, While},
    Span, Spanned,
};

ast_item!(
    pub struct WhileLoop {
        while_token: While,
        condition: Parenthesis<Expression>,
        block: Block,
    }
);

impl Spanned for WhileLoop {
    fn span(&self) -> Span {
        Span::from_start_end(self.while_token.span(), self.block.span())
    }
}
