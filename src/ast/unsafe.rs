use crate::{ast::Block, ast_item, token::Unsafe, Span, Spanned};

ast_item!(
    pub struct UnsafeBlock {
        unsafe_token: Unsafe,
        block: Block,
    }
);

impl Spanned for UnsafeBlock {
    fn span(&self) -> Span {
        Span::from_start_end(self.unsafe_token.span(), self.block.span())
    }
}
