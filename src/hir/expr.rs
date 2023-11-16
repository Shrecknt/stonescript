use super::{Path, Punctuated, ToTokens};
use crate::{
    ast_item,
    token::{
        And, Bracket, Comma, Delimiter, Dot, Equals, GreaterThan, GreaterThanEquals, Ident,
        LessThan, LessThanEquals, Literal, Minus, Not, NotEquals, Or, Parenthesis, Percent, Plus,
        Punct, PunctToken, Slash, Star, ToTokenTree,
    },
    Parse, Span, Spanned, SyntaxError, SyntaxResult, TokenIter, TokenTree,
};

macro_rules! unary_op_parse_left {
    ($punct:ident right $name:ident) => {
        None
    };
    ($punct:ident left $name:ident) => {
        if let Some(token) = $punct.into() {
            Some(Self::$name(token))
        } else {
            None
        }
    };
}

macro_rules! unary_op_parse_right {
    ($punct:ident left $name:ident) => {
        None
    };
    ($punct:ident right $name:ident) => {
        if let Some(token) = $punct.into() {
            Some(Self::$name(token))
        } else {
            None
        }
    };
}

macro_rules! unary_op_span {
    ($inner:ident $expr:ident left) => {
        Span::from_start_end($inner.span(), $expr.span())
    };
    ($inner:ident $expr:ident right) => {
        Span::from_start_end($expr.span(), $inner.span())
    };
}

macro_rules! unary_op_to_tokens {
    ($stream:ident $inner:ident $expr:ident left) => {{
        $inner.write_into_stream($stream);
        $expr.write_into_stream($stream);
    }};
    ($stream:ident $inner:ident $expr:ident right) => {{
        $expr.write_into_stream($stream);
        $inner.write_into_stream($stream);
    }};
}

macro_rules! define_unary_op {
    ($($pos:ident $name:ident : $inner:ident),+ $(,)?) => {
        ast_item!(
            pub enum UnaryOp {
                $($name($inner)),+
            }
        );

        impl UnaryOp {
            fn parse_left(punct: Punct) -> SyntaxResult<Option<UnaryOp>> {
                Ok(match punct.inner() {
                    $(PunctToken::$inner => unary_op_parse_left!(punct $pos $name),)+
                    _ => None,
                })
            }

            fn parse_right(punct: Punct) -> Option<UnaryOp> {
                match punct.inner() {
                    $(PunctToken::$inner => unary_op_parse_right!(punct $pos $name),)+
                    _ => None,
                }
            }

            fn span_with_expr(&self, expr: &Expression) -> Span {
                match self {
                    $(
                        Self::$name(inner) => unary_op_span!(inner expr $pos),
                    )+
                }
            }

            fn to_tokens_with_expr(self, expr: Expression, stream: &mut Vec<TokenTree>) {
                match self {
                    $(
                        Self::$name(inner) => unary_op_to_tokens!(stream inner expr $pos),
                    )+
                }
            }
        }

        pub(crate) mod mir_unaryop {
            use crate::mir::ToMir;
            use super::UnaryOp;

            #[derive(Debug, Clone, PartialEq)]
            pub enum MirUnaryOp {
                $($name),+
            }

            impl ToMir for UnaryOp {
                type Output = MirUnaryOp;

                fn into_mir(self) -> Self::Output {
                    match self {
                        $(
                            Self::$name(_) => MirUnaryOp::$name,
                        )+
                    }
                }
            }
        }
    }
}

define_unary_op!(
    left Not: Not,
    left Negate: Minus,
);

macro_rules! define_binary_op {
    ($($name:ident : $inner:ident),+ $(,)?) => {
        ast_item!(
            pub enum BinaryOp {
                $($name($inner)),+
            }
        );

        impl BinaryOp {
            fn parse(token_iter: &mut TokenIter, punct: PunctToken) -> SyntaxResult<Option<BinaryOp>> {
                Ok(match punct {
                    $(
                        PunctToken::$inner => Some(Self::$name(token_iter.parse()?)),
                    )+
                    _ => None,
                })
            }
        }

        pub(crate) mod mir_binaryop {
            use crate::mir::ToMir;
            use super::BinaryOp;

            #[derive(Debug, Clone, PartialEq)]
            pub enum MirBinaryOp {
                $($name),+
            }

            impl ToMir for BinaryOp {
                type Output = MirBinaryOp;

                fn into_mir(self) -> Self::Output {
                    match self {
                        $(
                            Self::$name(_) => MirBinaryOp::$name,
                        )+
                    }
                }
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
    Or: Or,
);

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Literal(Literal),
    Variable(Path),
    Property(Box<Expression>, Dot, Ident),
    Call(Path, Parenthesis<Punctuated<Expression, Comma>>),
    Parenthesized(Box<Parenthesis<Expression>>),
    Index(Box<Expression>, Box<Bracket<Expression>>),
    UnaryOp(UnaryOp, Box<Expression>),
    BinaryOp(Box<Expression>, BinaryOp, Box<Expression>),
}

impl Expression {
    fn continue_parsing(self, token_iter: &mut TokenIter) -> SyntaxResult<Self> {
        if let Some(next_token) = token_iter.peek() {
            match next_token {
                TokenTree::Group(group) => match group.delimiter() {
                    Delimiter::Parenthesis => {
                        if let Self::Variable(path) = self {
                            let args = token_iter.parse()?;
                            Self::Call(path, args).continue_parsing(token_iter)
                        } else {
                            Ok(self)
                        }
                    }
                    Delimiter::Bracket => {
                        let index = token_iter.parse()?;
                        Self::Index(Box::new(self), Box::new(index)).continue_parsing(token_iter)
                    }
                    _ => Ok(self),
                },
                TokenTree::Punct(punct) => {
                    let token = punct.inner();
                    if let PunctToken::Dot = token {
                        let dot = token_iter.parse()?;
                        let ident = token_iter.parse()?;
                        Self::Property(Box::new(self), dot, ident).continue_parsing(token_iter)
                    } else if let Some(op) = UnaryOp::parse_right(*punct) {
                        Ok(Self::UnaryOp(op, Box::new(self)))
                    } else if let Some(op) = BinaryOp::parse(token_iter, token)? {
                        let right = token_iter.parse()?;
                        Ok(Self::BinaryOp(Box::new(self), op, Box::new(right)))
                    } else {
                        Ok(self)
                    }
                }
                _ => Ok(self),
            }
        } else {
            Ok(self)
        }
    }
}

impl Parse for Expression {
    fn parse(token_iter: &mut TokenIter) -> SyntaxResult<Self> {
        let left = match token_iter.expect_peek()? {
            TokenTree::Literal(_) => Self::Literal(token_iter.parse()?),
            TokenTree::Ident(_) => Self::Variable(token_iter.parse()?),
            TokenTree::Group(group) => {
                if group.delimiter() == Delimiter::Parenthesis {
                    let inner = token_iter.parse()?;
                    Self::Parenthesized(Box::new(inner))
                } else {
                    return token_iter.expect_consume()?.unexpected();
                }
            }
            TokenTree::Punct(_) => {
                if let Some(op) = UnaryOp::parse_left(token_iter.parse()?)? {
                    let right = token_iter.parse()?;
                    Self::UnaryOp(op, Box::new(right))
                } else {
                    return token_iter.expect_consume()?.unexpected();
                }
            }
        };

        left.continue_parsing(token_iter)
    }
}

impl Spanned for Expression {
    fn span(&self) -> Span {
        match self {
            Self::Literal(literal) => literal.span(),
            Self::Variable(variable) => variable.span(),
            Self::Parenthesized(inner) => inner.span(),
            Self::Property(left, _dot, name) => Span::from_start_end(left.span(), name.span()),
            Self::Call(left, args) => Span::from_start_end(left.span(), args.span()),
            Self::Index(left, index) => Span::from_start_end(left.span(), index.span()),
            Self::BinaryOp(left, _op, right) => Span::from_start_end(left.span(), right.span()),
            Self::UnaryOp(op, expr) => op.span_with_expr(expr),
        }
    }
}

impl ToTokens for Expression {
    fn write_into_stream(self, stream: &mut Vec<TokenTree>) {
        match self {
            Self::Literal(literal) => literal.write_into_stream(stream),
            Self::Variable(variable) => variable.write_into_stream(stream),
            Self::Parenthesized(inner) => inner.write_into_stream(stream),
            Self::Property(left, dot, name) => {
                left.write_into_stream(stream);
                dot.write_into_stream(stream);
                name.write_into_stream(stream);
            }
            Self::Call(left, args) => {
                left.write_into_stream(stream);
                args.write_into_stream(stream);
            }
            Self::Index(left, index) => {
                left.write_into_stream(stream);
                index.write_into_stream(stream);
            }
            Self::BinaryOp(left, op, right) => {
                left.write_into_stream(stream);
                op.write_into_stream(stream);
                right.write_into_stream(stream);
            }
            Self::UnaryOp(op, expr) => op.to_tokens_with_expr(*expr, stream),
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
