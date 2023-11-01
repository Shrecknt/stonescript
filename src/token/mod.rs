use thiserror::Error;

use self::{
    group::Group,
    ident::Ident,
    literal::Literal,
    punct::{Punct, PunctToken},
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

pub fn tokenise<T: FusedIterator<Item = char>>(
    reader: &mut Stream<T>,
    closing_char: Option<char>,
) -> ParseResult<Vec<TokenTree>> {
    let mut tokens = vec![];

    println!("Tokenising: {:?}", closing_char);

    loop {
        let next_char = if let Some(closing_char) = closing_char {
            if let Some(next_char) = reader.peek() {
                if next_char == closing_char {
                    reader.advance();
                    break;
                } else {
                    next_char
                }
            } else {
                return Err(ParseError::EarlyEof);
            }
        } else {
            if let Some(next_char) = reader.peek() {
                next_char
            } else {
                break;
            }
        };

        if next_char.is_whitespace() {
            reader.advance();
            continue;
        }

        match TokenTree::parse(reader)? {
            TokenTree::Punct(Punct {
                span: _,
                token: PunctToken::Comment,
            }) => {
                while let Some(next_char) = reader.next() {
                    if next_char == '\r' || next_char == '\n' {
                        break;
                    }
                }
            }
            other => tokens.push(other),
        }
    }

    Ok(tokens)
}
