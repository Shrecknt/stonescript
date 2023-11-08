use super::{Assignment, Block, Declaration, Expression, FunctionDecl};
use crate::{
    token::{Assign, Colon, Delimiter, Function, Ident, Semicolon, Static, Token},
    Parse, SyntaxResult, TokenIter, TokenTree,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Block(Block),
    Function(FunctionDecl),
    Declaration(Declaration),
    Expression(Expression, Semicolon),
    Assignment(Assignment),
}

impl Parse for Statement {
    fn parse(token_iter: &mut TokenIter) -> SyntaxResult<Self> {
        let next_token = token_iter.expect_peek()?;
        if let TokenTree::Group(group) = next_token {
            if group.delimiter() == Delimiter::Brace {
                Ok(Self::Block(token_iter.parse()?))
            } else {
                let expr = token_iter.parse()?;
                let semicolon = token_iter.parse()?;
                Ok(Self::Expression(expr, semicolon))
            }
        } else if let Some(_) = Function::parse_token(next_token.clone()) {
            Ok(Self::Function(token_iter.parse()?))
        } else if let Some(_) = Static::parse_token(next_token.clone()) {
            let next2_token = token_iter.expect_peek_ahead(1)?;
            if let Some(_) = Function::parse_token(next2_token) {
                Ok(Self::Function(token_iter.parse()?))
            } else {
                Ok(Self::Declaration(token_iter.parse()?))
            }
        } else if let Some(_) = Ident::parse_token(next_token) {
            let next2_token = token_iter.expect_peek_ahead(1)?;
            if let Some(_) = Colon::parse_token(next2_token.clone()) {
                Ok(Self::Declaration(token_iter.parse()?))
            } else if let Some(_) = Assign::parse_token(next2_token) {
                Ok(Self::Assignment(token_iter.parse()?))
            } else {
                let expr = token_iter.parse()?;
                let semicolon = token_iter.parse()?;
                Ok(Self::Expression(expr, semicolon))
            }
        } else {
            let expr = token_iter.parse()?;
            let semicolon = token_iter.parse()?;
            Ok(Self::Expression(expr, semicolon))
        }
    }
}
