use super::Punctuated;
use crate::{
    token::{Comma, Delimiter, Dot, Ident, Literal, Parenthesis, Punct, PunctToken},
    Parse, TokenIter, TokenTree, SyntaxResult, SyntaxError
};

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Literal(Literal),
    Variable(Ident),
    Property(Box<Expression>, Dot, Ident),
    Call(Box<Expression>, Parenthesis, Punctuated<Expression, Comma>),
    Parenthesized(Parenthesis, Box<Expression>),
    Operation(Box<Expression>, Punct, Box<Expression>),
}

impl Parse for Expression {
    fn parse(token_iter: &mut TokenIter) -> SyntaxResult<Self> {
        let left = match token_iter.expect_consume()? {
            TokenTree::Literal(literal) => Expression::Literal(literal),
            TokenTree::Ident(ident) => Expression::Variable(ident),
            TokenTree::Group(group) => {
                if group.delimiter() == Delimiter::Parenthesis {
                    let (paren, inner) = token_iter.parenthesized()?;
                    Expression::Parenthesized(paren, Box::new(inner))
                } else {
                    return Err(SyntaxError::UnexpectedToken(TokenTree::Group(group), "expression"));
                }
            }
            other => return Err(SyntaxError::UnexpectedToken(other, "expression")),
        };

        if let Some(next_token) = token_iter.peek() {
            match next_token {
                TokenTree::Group(group) => {
                    if group.delimiter() == Delimiter::Parenthesis {
                        let (paren, inner) = token_iter.parenthesized()?;
                        Ok(Expression::Call(Box::new(left), paren, inner))
                    } else {
                        Err(SyntaxError::UnexpectedToken(TokenTree::Group(group), "expression"))
                    }
                }
                TokenTree::Punct(punct) => {
                    if punct.inner() == PunctToken::Dot {
                        let dot = token_iter.parse()?;
                        let ident = token_iter.parse()?;

                        Ok(Expression::Property(Box::new(left), dot, ident))
                    } else {
                        let punct = token_iter.parse()?;
                        let right = token_iter.parse()?;

                        Ok(Expression::Operation(
                            Box::new(left),
                            punct,
                            Box::new(right),
                        ))
                    }
                }
                _ => Ok(left),
            }
        } else {
            Ok(left)
        }
    }
}
