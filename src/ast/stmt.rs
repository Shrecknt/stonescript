use super::{Assignment, Declaration, Expression, FunctionDecl};
use crate::{
    token::{Assign, Colon, Function, Ident, Static, Token},
    Parse, SyntaxResult, TokenIter,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Function(FunctionDecl),
    Declaration(Declaration),
    Expression(Expression),
    Assignment(Assignment),
}

impl Parse for Statement {
    fn parse(token_iter: &mut TokenIter) -> SyntaxResult<Self> {
        let next_token = token_iter.expect_peek()?;
        if let Some(_) = Function::parse_token(next_token.clone()) {
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
                Ok(Self::Expression(token_iter.parse()?))
            }
        } else {
            Ok(Self::Expression(token_iter.parse()?))
        }

        // Ok(match  {
        //     TokenTree::Ident(ident) => {
        //         if let
        //     }
        //     _ => {
        //         Self::Expression(token_iter.parse()?)
        //     }
        // })
        // if let Some(function_token) =  {
        //     return Ok(Statement::Function(FunctionDecl::parse(token_iter)?));
        // }

        // if let Some(staticness) = token_iter.parse::<Option<Static>>()? {
        //     return Ok(
        //         if let Some(function_token) = token_iter.parse::<Option<Function>>()? {
        //             Statement::Function(FunctionDecl::parse(token_iter)?)
        //         } else {
        //             Statement::
        //         }
        //     )
        // }
    }
}
