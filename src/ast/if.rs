use crate::{
    ast::{Block, Expression},
    ast_item,
    token::{Else, If, Parenthesis},
    Parse, Span, Spanned, SyntaxError, TokenTree,
};

ast_item!(
    pub struct IfBlock {
        if_token: If,
        condition: Parenthesis<Expression>,
        block: Block,
        r#else: Option<Box<ElseBlocks>>,
    }
);

ast_item!(
    pub enum ElseBlocks {
        ElseIf(ElseIfBlock),
        Else(ElseBlock),
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

impl Parse for Option<Box<ElseBlocks>> {
    fn parse(token_iter: &mut crate::TokenIter) -> crate::SyntaxResult<Self> {
        match token_iter.expect_peek()? {
            TokenTree::Ident(ident) => {
                if Else::is_ident(ident) {
                    let else_token: Else = token_iter.parse()?;
                    match token_iter.expect_peek()? {
                        TokenTree::Punct(_) => Ok(Some(
                            ElseBlocks::ElseIf(ElseIfBlock {
                                else_token,
                                if_token: token_iter.parse()?,
                                condition: token_iter.parse()?,
                                block: token_iter.parse()?,
                                r#else: token_iter.parse()?,
                            })
                            .into(),
                        )),
                        _ => Ok(Some(
                            ElseBlocks::Else(ElseBlock {
                                else_token,
                                condition: token_iter.parse()?,
                                block: token_iter.parse()?,
                            })
                            .into(),
                        )),
                    }
                } else {
                    Err(SyntaxError::UnexpectedToken(
                        token_iter.expect_consume()?,
                        "else",
                    ))
                }
            }
            _ => Err(SyntaxError::UnexpectedToken(
                token_iter.expect_consume()?,
                "else",
            )),
        }
    }
}
