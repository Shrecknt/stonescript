use crate::{ast_item, token::{While, Parenthesis}, ast::{Expression, Block}, Spanned, Span};

ast_item!(
    pub struct WhileLoop {
        while_token: While,
        condition: (Parenthesis, Expression),
        block: Block
    }
);

impl Spanned for WhileLoop {
    fn span(&self) -> Span {
        Span::from_start_end(self.while_token.span(), self.block.span())
    }
}