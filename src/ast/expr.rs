use super::{bracketed, parenthesized, Punctuated, ToTokens};
use crate::{
    token::{
        And, Bracket, Comma, Delimiter, Dot, Equals, GreaterThan, GreaterThanEquals, Ident,
        LessThan, LessThanEquals, Literal, Minus, Not, NotEquals, Or, Parenthesis, Percent, Plus,
        PunctToken, Slash, Star, ToTokenTree,
    },
    Parse, Span, Spanned, SyntaxError, SyntaxResult, TokenIter, TokenTree, ast_item,
};

ast_item!(
    pub enum UnaryOp {
        Not(Not),
        Negate(Minus)
    }
);

macro_rules! define_binary_op {
    ($($name:ident : $inner:ident),+) => {
        ast_item!(
            pub enum BinaryOp {
                $($name($inner)),+
            }
        );

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
            TokenTree::Group(group) => match group.delimiter() {
                Delimiter::Parenthesis => {
                    let (paren, inner) = parenthesized(token_iter.parse()?)?;
                    continue_expr(token_iter, Expression::Call(Box::new(left), paren, inner))
                }
                Delimiter::Bracket => {
                    let (bracket, inner) = bracketed(token_iter.parse()?)?;
                    continue_expr(
                        token_iter,
                        Expression::Index(Box::new(left), bracket, Box::new(inner)),
                    )
                }
                _ => Ok(left),
            },
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
            _ => Ok(left),
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
                    let (paren, inner) = parenthesized(group)?;
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

impl Spanned for Expression {
    fn span(&self) -> Span {
        match self {
            Self::Literal(literal) => literal.span(),
            Self::Variable(variable) => variable.span(),
            Self::Parenthesized(paren, _expr) => paren.span(),
            Self::Property(left, _dot, name) => Span::from_start_end(left.span(), name.span()),
            Self::Call(left, paren, _args) => Span::from_start_end(left.span(), paren.span()),
            Self::Index(left, bracket, _inner) => Span::from_start_end(left.span(), bracket.span()),
            Self::BinaryOp(left, _op, right) => Span::from_start_end(left.span(), right.span()),
            Self::UnaryOp(op, expr) => match op {
                UnaryOp::Negate(minus) => Span::from_start_end(minus.span(), expr.span()),
                UnaryOp::Not(not) => Span::from_start_end(not.span(), expr.span()),
            },
        }
    }
}

impl ToTokens for Expression {
    fn write_into_stream(self, stream: &mut Vec<TokenTree>) {
        match self {
            Self::Literal(literal) => literal.write_into_stream(stream),
            Self::Variable(variable) => variable.write_into_stream(stream),
            Self::Parenthesized(paren, expr) => paren.into_group(*expr).write_into_stream(stream),
            Self::Property(left, dot, name) => {
                left.write_into_stream(stream);
                dot.write_into_stream(stream);
                name.write_into_stream(stream);
            }
            Self::Call(left, paren, args) => {
                left.write_into_stream(stream);
                paren.into_group(args).write_into_stream(stream);
            }
            Self::Index(left, bracket, inner) => {
                left.write_into_stream(stream);
                bracket.into_group(*inner).write_into_stream(stream);
            }
            Self::BinaryOp(left, op, right) => {
                left.write_into_stream(stream);
                op.write_into_stream(stream);
                right.write_into_stream(stream);
            }
            Self::UnaryOp(op, expr) => match op {
                UnaryOp::Negate(minus) => {
                    minus.write_into_stream(stream);
                    expr.write_into_stream(stream);
                }
                UnaryOp::Not(not) => {
                    not.write_into_stream(stream);
                    expr.write_into_stream(stream);
                }
            },
        }
    }
}

trait UnexpectedToken
where
    Self: Sized + ToTokenTree,
{
    fn unexpected<T>(self) -> SyntaxResult<T> {
        Err(SyntaxError::UnexpectedToken(
            self.to_token_tree(),
            "expression",
        ))
    }
}

impl<T: ToTokenTree> UnexpectedToken for T {}
