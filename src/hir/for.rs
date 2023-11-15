use super::Assignment;
use crate::{
    ast_item,
    hir::{Block, Declaration, Expression},
    token::{For, Parenthesis, Semicolon},
    Span, Spanned,
};

ast_item!(
    pub struct ForLoopInner {
        init: Declaration,
        condition: (Expression, Semicolon),
        update: Assignment,
    }
);

impl Spanned for ForLoopInner {
    fn span(&self) -> Span {
        Span::from_start_end(self.init.span(), self.update.span())
    }
}

ast_item!(
    pub struct ForLoop {
        for_token: For,
        inner: Parenthesis<ForLoopInner>,
        block: Block,
    }
);

impl Spanned for ForLoop {
    fn span(&self) -> Span {
        Span::from_start_end(self.for_token.span(), self.block.span())
    }
}
