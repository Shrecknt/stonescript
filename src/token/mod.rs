use thiserror::Error;

use self::{
    group::Group,
    ident::Ident,
    literal::Literal,
    punct::Punct,
};
use std::iter::FusedIterator;
use stream::Stream;

pub mod group;
pub mod ident;
pub mod literal;
pub mod punct;
pub mod stream;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Unexpected end of file")]
    EarlyEof,
    #[error("Unexpected {0:?} while parsing {1}")]
    UnexpectedToken(String, &'static str, Span),
}

pub type ParseResult<T> = Result<T, ParseError>;

pub trait Token<T: FusedIterator<Item = char>>
where
    Self: Sized,
{
    fn parse(reader: &mut Stream<T>) -> ParseResult<Self>;

    fn valid_start(start: char) -> bool;
}

#[derive(Debug, Clone, Copy)]
pub struct Span {
    pub index: usize,
    pub width: usize,
}

impl Span {
    pub fn new(index: usize, width: usize) -> Self {
        Span { index, width }
    }
}

#[derive(Debug, Clone)]
pub enum TokenTree {
    Punct(Punct),
    Ident(Ident),
    Literal(Literal),
    Group(Group),
}

impl TokenTree {
    pub fn span(&self) -> Span {
        match self {
            Self::Punct(punct) => punct.span,
            Self::Ident(ident) => ident.span,
            Self::Literal(literal) => literal.span,
            Self::Group(group) => group.span,
        }
    }
}

impl<T: FusedIterator<Item = char>> Token<T> for TokenTree {
    fn parse(reader: &mut Stream<T>) -> ParseResult<Self> {
        let first_char = reader.expect_peek()?;
        if <Group as Token<T>>::valid_start(first_char) {
            Ok(TokenTree::Group(Group::parse(reader)?))
        } else if <Literal as Token<T>>::valid_start(first_char) {
            Ok(TokenTree::Literal(Literal::parse(reader)?))
        } else if <Punct as Token<T>>::valid_start(first_char) {
            Ok(TokenTree::Punct(Punct::parse(reader)?))
        } else if <Ident as Token<T>>::valid_start(first_char) {
            Ok(TokenTree::Ident(Ident::parse(reader)?))
        } else {
            Err(ParseError::UnexpectedToken(
                first_char.to_string(),
                "token tree",
                Span::new(reader.position, 1),
            ))
        }
    }

    fn valid_start(start: char) -> bool {
        <Group as Token<T>>::valid_start(start)
            || <Literal as Token<T>>::valid_start(start)
            || <Punct as Token<T>>::valid_start(start)
            || <Ident as Token<T>>::valid_start(start)
    }
}