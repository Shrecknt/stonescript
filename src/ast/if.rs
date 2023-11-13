use crate::{
    ast::{Block, Expression},
    ast_item,
    token::{If, Parenthesis},
    Span, Spanned,
};

ast_item!(
    pub struct IfBlock {
        if_token: If,
        condition: Parenthesis<Expression>,
        block: Block,
    }
);

impl Spanned for IfBlock {
    fn span(&self) -> Span {
        Span::from_start_end(self.if_token.span(), self.block.span())
    }
}
