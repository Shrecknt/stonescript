use super::{tokenise, Span, Token, TokenTree};
use crate::{stream::Stream, ExpectChar, ParseError, ParseResult};
use std::iter::FusedIterator;

#[derive(Debug, Clone, Copy)]
pub enum Delimiter {
    Parenthesis,
    Brace,
    Bracket,
}

impl Delimiter {
    fn from_open(open: char) -> Option<Self> {
        match open {
            '{' => Some(Delimiter::Brace),
            '[' => Some(Delimiter::Bracket),
            '(' => Some(Delimiter::Parenthesis),
            _ => None,
        }
    }

    fn to_close(self) -> char {
        match self {
            Delimiter::Brace => '}',
            Delimiter::Bracket => ']',
            Delimiter::Parenthesis => ')',
        }
    }
}

#[derive(Debug, Clone)]
pub struct Group {
    pub span: Span,
    pub delimiter: Delimiter,
    pub tokens: Vec<TokenTree>,
}

impl<T: FusedIterator<Item = char>> Token<T> for Group {
    fn parse(reader: &mut Stream<T>) -> ParseResult<Self> {
        let start_pos = reader.position;

        // TODO: This is shit, it forces everything to be loaded into memory. Fix this.

        let open_char = reader.next().expect_char()?;
        let delimiter = Delimiter::from_open(open_char)
            .ok_or(ParseError::UnexpectedToken(open_char.to_string(), "group"))?;
        let close_char = delimiter.to_close();

        let mut buffer = String::new();
        let mut depth = 1;
        loop {
            let next_char = reader.next().expect_char()?;
            if next_char == open_char {
                depth += 1;
            } else if next_char == close_char {
                depth -= 1;
                if depth == 0 {
                    break;
                }
            }
            buffer.push(next_char);
        }

        let width = reader.position - start_pos;
        Ok(Group {
            span: Span::new(start_pos, width),
            delimiter,
            tokens: tokenise((&mut buffer.chars()).into())?,
        })
    }

    fn valid_start(start: char) -> bool {
        start == '{' || start == '[' || start == '('
    }
}
