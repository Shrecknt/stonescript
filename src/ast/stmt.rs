use super::{
    Assignment, Block, Declaration, Expression, ForLoop, FunctionDecl, IfBlock, WhileLoop,
};
use crate::{
    ast_item,
    token::{
        Assign, Colon, Delimiter, For, Function, If, Let, Return, Semicolon, Static, Unsafe, While,
    },
    Parse, Span, SyntaxResult, TokenIter, TokenTree,
};

ast_item!(
    pub enum Statement {
        Block(Block),
        Function(FunctionDecl),
        Declaration(Declaration),
        Expression((Expression, Semicolon)),
        Assignment(Assignment),
        Return((Return, Expression, Semicolon)),
        While(WhileLoop),
        If(Box<IfBlock>),
        For(Box<ForLoop>),
        Unsafe((Unsafe, Block)),
    }
);

impl Parse for Statement {
    fn parse(token_iter: &mut TokenIter) -> SyntaxResult<Self> {
        match token_iter.expect_peek()? {
            TokenTree::Group(group) => {
                if group.delimiter() == Delimiter::Brace {
                    return Ok(Self::Block(token_iter.parse()?));
                }
            }
            TokenTree::Ident(ident) => {
                if Return::is_ident(ident) {
                    return Ok(Self::Return(token_iter.parse()?));
                }

                if Unsafe::is_ident(ident) {
                    return Ok(Self::Unsafe(token_iter.parse()?));
                }

                if While::is_ident(ident) {
                    return Ok(Self::While(token_iter.parse()?));
                }

                if If::is_ident(ident) {
                    return Ok(Self::If(Box::new(token_iter.parse()?)));
                }

                if For::is_ident(ident) {
                    return Ok(Self::For(Box::new(token_iter.parse()?)));
                }

                if Function::is_ident(ident) {
                    return Ok(Self::Function(token_iter.parse()?));
                }

                if Let::is_ident(ident) {
                    return Ok(Self::Declaration(token_iter.parse()?));
                }

                if Static::is_ident(ident) {
                    if let TokenTree::Ident(next_ident) = token_iter.expect_peek_ahead(1)? {
                        if Function::is_ident(next_ident) {
                            return Ok(Self::Function(token_iter.parse()?));
                        }
                    }

                    return Ok(Self::Declaration(token_iter.parse()?));
                }

                if let TokenTree::Punct(next_punct) = token_iter.expect_peek_ahead(1)? {
                    if Colon::is_punct(next_punct) {
                        return Ok(Self::Declaration(token_iter.parse()?));
                    }

                    if Assign::is_punct(next_punct) {
                        return Ok(Self::Assignment(token_iter.parse()?));
                    }
                }
            }
            _ => (),
        }

        let expr = token_iter.parse()?;
        Ok(Self::Expression(expr))
    }
}
