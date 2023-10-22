use super::{Span, Token};
use crate::{stream::Stream, ExpectChar, ParseError, ParseResult};
use lazy_static::lazy_static;
use std::{collections::HashMap, iter::FusedIterator};

macro_rules! define_keyword {
    ($($variant:ident => $value:literal),+) => {
        #[derive(Debug, Clone, Copy)]
        pub enum Keyword {
            $($variant),+
        }

        lazy_static! {
            static ref KEYWORD_MAP: HashMap<&'static str, Keyword> = {
                let mut map = HashMap::new();
                $(map.insert($value, Keyword::$variant);)+
                map
            };
        }
    }
}

define_keyword!(
    Static => "static",
    For => "for",
    While => "while",
    Let => "let",
    Const => "const",
    Function => "function",
    As => "as",
    Null => "null",
    Return => "return",
    Throw => "throw"
);

#[derive(Debug, Clone)]
pub enum IdentType {
    Keyword(Keyword),
    VariableName(String),
}

#[derive(Debug, Clone)]
pub struct Ident {
    pub span: Span,
    pub token: IdentType,
}

impl<T: FusedIterator<Item = char>> Token<T> for Ident {
    fn parse(reader: &mut Stream<T>) -> ParseResult<Self> {
        let start_pos = reader.position;
        let mut buffer = String::new();

        buffer.push({
            let char = reader.peek().expect_char()?;
            if unicode_ident::is_xid_start(char) {
                reader.advance();
                char
            } else {
                return Err(ParseError::UnexpectedToken(char.to_string(), "ident"));
            }
        });

        while let Some(char) = reader.peek() {
            if unicode_ident::is_xid_continue(char) {
                buffer.push(char);
                reader.advance();
            } else {
                break;
            }
        }

        Ok(Ident {
            span: Span::new(start_pos, buffer.len()),
            token: if let Some(keyword) = KEYWORD_MAP.get(&buffer.as_str()) {
                IdentType::Keyword(*keyword)
            } else {
                IdentType::VariableName(buffer)
            },
        })
    }

    fn valid_start(start: char) -> bool {
        unicode_ident::is_xid_start(start)
    }
}
