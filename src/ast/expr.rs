use crate::{expect_token, token::{punct::PunctToken, TokenTree, ident::IdentType, literal::LiteralType}};
use super::{SyntaxResult, stream::Stream, SyntaxError};

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    String(String),
    Ident(String),
    Add(Box<Expression>, Box<Expression>),
    Subtract(Box<Expression>, Box<Expression>),
    Multiply(Box<Expression>, Box<Expression>),
    Divide(Box<Expression>, Box<Expression>),
}

impl TryFrom<TokenTree> for Expression {
    type Error = SyntaxError;

    fn try_from(token: TokenTree) -> Result<Self, Self::Error> {
        match token.clone() {
            TokenTree::Ident(ident) => Ok(Expression::Ident(
                expect_token!(token: IdentType::VariableName[a] = ident.token),
            )),
            TokenTree::Literal(literal) => Ok(match literal.value {
                LiteralType::Byte(val) => Self::Byte(val),
                LiteralType::Short(val) => Self::Short(val),
                LiteralType::Int(val) => Self::Int(val),
                LiteralType::Long(val) => Self::Long(val),
                LiteralType::Float(val) => Self::Float(val),
                LiteralType::Double(val) => Self::Double(val),
                LiteralType::String(val) => Self::String(val),
                _ => return Err(Self::Error::UnexpectedToken(token)),
            }),
            TokenTree::Group(group) => Self::next(&mut group.tokens.into()),
            _ => Err(Self::Error::UnexpectedToken(token)),
        }
    }
}

impl Expression {
    pub(super) fn next(token_tree: &mut Stream) -> SyntaxResult<Self> {
        let token = token_tree.expect_next()?;
        let expr_a = Expression::try_from(token)?;
    
        let operator_token = match token_tree.peek() {
            Some(token) => match expect_token!(token: TokenTree::Punct[a] = token).token {
                PunctToken::Semicolon => return Ok(expr_a),
                _ => {
                    token_tree.advance();
                    token
                }
            },
            None => return Ok(expr_a),
        };

        let operator = expect_token!(operator_token: TokenTree::Punct[a] = operator_token).token;
        let expr_b = Self::next(token_tree)?;
    
        Ok(match operator {
            PunctToken::Add => Expression::Add(expr_a.into(), expr_b.into()),
            PunctToken::Subtract => Expression::Subtract(expr_a.into(), expr_b.into()),
            PunctToken::Multiply => Expression::Multiply(expr_a.into(), expr_b.into()),
            PunctToken::Slash => Expression::Divide(expr_a.into(), expr_b.into()),
            _ => return Err(SyntaxError::UnexpectedToken(operator_token)),
        })
    }
}