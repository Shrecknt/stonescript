use super::{Span, Token, TokenTree, stream::Stream, ParseError, ParseResult};
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
        let open_char = reader.expect_next()?;
        let delimiter = Delimiter::from_open(open_char)
            .ok_or(ParseError::UnexpectedToken(open_char.to_string(), "group", Span::new(start_pos, 1)))?;

        let tokens = reader.tokenise(Some(delimiter.to_close()))?;
        let width = reader.position - start_pos;
        
        Ok(Group {
            span: Span::new(start_pos, width),
            delimiter,
            tokens,
        })
    }

    fn valid_start(start: char) -> bool {
        start == '{' || start == '[' || start == '('
    }
}
