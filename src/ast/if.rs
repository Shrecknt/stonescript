use crate::{
    ast::{Block, Expression},
    ast_item,
    token::{Else, If, Parenthesis},
    Span, Spanned,
};

ast_item!(
    pub struct IfBlock {
        if_token: If,
        condition: Parenthesis<Expression>,
        block: Block,
        r#else: Option<ElseBlocks>,
    }
);

ast_item!(
    pub struct ElseBlock {
        else_token: Else,
        condition: Parenthesis<Expression>,
        block: Block,
    }
);

ast_item!(
    pub struct ElseIfBlock {
        else_token: Else,
        if_token: If,
        condition: Parenthesis<Expression>,
        block: Block,
        r#else: Option<Box<ElseBlocks>>,
    }
);

#[derive(Debug, Clone, PartialEq)]
pub enum ElseBlocks {
    ElseIf(ElseIfBlock),
    Else(ElseBlock),
}

impl Spanned for IfBlock {
    fn span(&self) -> Span {
        Span::from_start_end(self.if_token.span(), self.block.span())
    }
}

impl Spanned for ElseBlock {
    fn span(&self) -> Span {
        Span::from_start_end(self.else_token.span(), self.block.span())
    }
}

impl Spanned for ElseIfBlock {
    fn span(&self) -> Span {
        Span::from_start_end(self.else_token.span(), self.block.span())
    }
}
