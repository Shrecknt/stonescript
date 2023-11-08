use super::Punctuated;
use crate::{
    token::{Comma, Delimiter, Dot, Ident, Literal, Parenthesis, Punct, PunctToken, Not, Minus, Plus, Star, Slash, Percent, Equals, LessThan, LessThanEquals, GreaterThan, GreaterThanEquals, And, Or, Bracket, Group, NotEquals},
    Parse, TokenIter, TokenTree, SyntaxResult, SyntaxError
};

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Not(Not),
    Negate(Minus),
}

macro_rules! define_binary_op {
    ($($name:ident : $inner:ident),+) => {
        #[derive(Debug, Clone, PartialEq)]
        pub enum BinaryOp {
            $($name($inner),)+
        }

        impl BinaryOp {
            fn parse(token_iter: &mut TokenIter, punct: PunctToken) -> SyntaxResult<Option<(BinaryOp, Box<Expression>)>> {
                Ok(match punct {
                    $(
                        PunctToken::$inner => {
                            let op = token_iter.parse()?;
                            let right = token_iter.parse()?;
                            Some((Self::$name(op), Box::new(right)))
                        }
                    )+
                    _ => None
                })
            }
        }
    }
}

define_binary_op!(
    Add: Plus,
    Subtract: Minus,
    Multiply: Star,
    Divide: Slash,
    Modulo: Percent,
    Equals: Equals,
    NotEquals: NotEquals,
    LessThan: LessThan,
    LessThanEquals: LessThanEquals,
    GreaterThan: GreaterThan,
    GreaterThanEquals: GreaterThanEquals,
    And: And,
    Or: Or
);

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Literal(Literal),
    Variable(Ident),
    Property(Box<Expression>, Dot, Ident),
    Call(Box<Expression>, Parenthesis, Punctuated<Expression, Comma>),
    Parenthesized(Parenthesis, Box<Expression>),
    Index(Box<Expression>, Bracket, Box<Expression>),
    UnaryOp(UnaryOp, Box<Expression>),
    BinaryOp(Box<Expression>, BinaryOp, Box<Expression>),
}

fn continue_expr(token_iter: &mut TokenIter, left: Expression) -> SyntaxResult<Expression> {
    if let Some(next_token) = token_iter.peek() {
        match next_token {
            TokenTree::Group(group) => {
                match group.delimiter() {
                    Delimiter::Parenthesis => {
                        let (paren, inner) = token_iter.parenthesized()?;
                        continue_expr(token_iter, Expression::Call(Box::new(left), paren, inner))
                    }
                    Delimiter::Bracket => {
                        let (bracket, inner) = token_iter.bracketed()?;
                        continue_expr(token_iter, Expression::Index(Box::new(left), bracket, Box::new(inner)))
                    }
                    _ => Ok(left),
                }
            }
            TokenTree::Punct(punct) => {
                let token = punct.inner();
                if let PunctToken::Dot = token {
                    let dot = token_iter.parse()?;
                    let ident = token_iter.parse()?;
                    continue_expr(token_iter, Expression::Property(Box::new(left), dot, ident))
                } else if let Some((op, right)) = BinaryOp::parse(token_iter, token)? {
                    Ok(Expression::BinaryOp(Box::new(left), op, right))
                } else {
                    Ok(left)
                }
            }
            _ => Ok(left)
        }
    } else {
        Ok(left)
    }
}

impl Parse for Expression {
    fn parse(token_iter: &mut TokenIter) -> SyntaxResult<Self> {
        let left = match token_iter.expect_consume()? {
            TokenTree::Literal(literal) => Self::Literal(literal),
            TokenTree::Ident(ident) => Self::Variable(ident),
            TokenTree::Group(group) => {
                if group.delimiter() == Delimiter::Parenthesis {
                    let (paren, inner) = token_iter.parenthesized()?;
                    Self::Parenthesized(paren, Box::new(inner))
                } else {
                    return group.unexpected();
                }
            }
            TokenTree::Punct(punct) => {
                let op = if let Some(not) = punct.into() {
                    UnaryOp::Not(not)
                } else if let Some(minus) = punct.into() {
                    UnaryOp::Negate(minus)
                } else {
                    return punct.unexpected();
                };

                let expr = token_iter.parse()?;
                Self::UnaryOp(op, Box::new(expr))
            }
        };

        continue_expr(token_iter, left)
    }
}

trait UnexpectedToken where Self: Sized {
    fn to_token_tree(self) -> TokenTree;
    fn unexpected<T>(self) -> SyntaxResult<T> {
        Err(SyntaxError::UnexpectedToken(self.to_token_tree(), "expression"))
    }
}

impl UnexpectedToken for TokenTree {
    fn to_token_tree(self) -> TokenTree {
        self
    }
}

impl UnexpectedToken for Group {
    fn to_token_tree(self) -> TokenTree {
        TokenTree::Group(self)
    }
}

impl UnexpectedToken for Ident {
    fn to_token_tree(self) -> TokenTree {
        TokenTree::Ident(self)
    }
}

impl UnexpectedToken for Punct {
    fn to_token_tree(self) -> TokenTree {
        TokenTree::Punct(self)
    }
}

impl UnexpectedToken for Literal {
    fn to_token_tree(self) -> TokenTree {
        TokenTree::Literal(self)
    }
}