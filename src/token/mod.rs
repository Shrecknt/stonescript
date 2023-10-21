use self::{group::Group, ident::Ident, literal::Literal, punct::Punct};
use crate::{stream::Stream, ExpectChar, ParseError, ParseResult};
use std::iter::FusedIterator;

mod group;
mod ident;
mod literal;
mod punct;

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
        let first_char = reader.peek().expect_char()?;
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

pub fn tokenise<T: FusedIterator<Item = char>>(mut reader: Stream<T>) -> ParseResult<Vec<TokenTree>> {
    let mut tokens = vec![];
    
    loop {
        if let Some(next_char) = reader.peek() {
            match next_char {
                ' ' => {
                    reader.advance();
                    continue;
                }
                '\n' => {
                    reader.advance();
                    continue;
                }
                '\r' => {
                    reader.advance();
                    if let Some('\n') = reader.next() {
                        continue;
                    } else {
                        return Err(ParseError::UnexpectedToken(next_char.to_string(), "tokens"))
                    }
                }
                _ => ()
            }

            tokens.push(TokenTree::parse(&mut reader)?);
        } else {
            break;
        }
    }

    Ok(tokens)
}