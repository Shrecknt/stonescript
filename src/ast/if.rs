use crate::{
    ast::{Block, Expression},
    ast_item,
    token::{Else, If, Parenthesis},
    Parse, Span, Spanned, SyntaxResult, TokenIter, TokenTree,
};

ast_item!(
    pub struct IfBlock {
        if_token: If,
        condition: Parenthesis<Expression>,
        block: Block,
        else_block: Option<(Else, ElseBlock)>,
    }
);

ast_item!(
    pub enum ElseBlock {
        ElseIf(Box<IfBlock>),
        Else(Block),
    }
);

impl Spanned for IfBlock {
    fn span(&self) -> Span {
        Span::from_start_end(self.if_token.span(), self.block.span())
    }
}

impl Parse for ElseBlock {
    fn parse(token_iter: &mut TokenIter) -> SyntaxResult<Self> {
        if let TokenTree::Group(_) = token_iter.expect_peek()? {
            let block = token_iter.parse()?;
            Ok(Self::Else(block))
        } else {
            let if_stmt = token_iter.parse()?;
            Ok(Self::ElseIf(Box::new(if_stmt)))
        }
    }
}
