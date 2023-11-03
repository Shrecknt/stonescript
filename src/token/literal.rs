use super::{cursor::Cursor, ParseError, ParseResult, ParseToken, TokenTree};
use crate::{Span, Spanned};
use std::{fmt, iter::FusedIterator};

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralType {
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    String(String),
}

impl fmt::Display for LiteralType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Byte(value) => write!(f, "{}b", value),
            Self::Short(value) => write!(f, "{}s", value),
            Self::Int(value) => write!(f, "{}i", value),
            Self::Long(value) => write!(f, "{}l", value),
            Self::Float(value) => write!(f, "{}f", value),
            Self::Double(value) => write!(f, "{}d", value),
            Self::String(value) => write!(f, "{:?}", value),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Literal {
    span: Span,
    value: LiteralType,
}

impl Literal {
    pub fn inner(&self) -> &LiteralType {
        &self.value
    }
}

impl Spanned for Literal {
    fn span(&self) -> Span {
        self.span
    }
}

macro_rules! number_or_error {
    ($buffer:ident $span:ident; $variant:ident) => {
        LiteralType::$variant(
            $buffer.parse().map_err(move |_| {
                ParseError::UnexpectedToken($buffer, stringify!($variant), $span)
            })?,
        )
    };
}

macro_rules! parse_number {
    ($start:ident, $cursor:ident; $($char:literal = $variant:ident),+) => {
        let mut buffer = String::from($start);

        loop {
            let next_char = $cursor.expect_peek()?;
            if next_char == '.' || next_char.is_ascii_digit() {
                buffer.push(next_char);
                $cursor.consume();
                continue;
            }

            let span;
            let value = match next_char {
                $(
                    $char => {
                        $cursor.consume();
                        span = $cursor.into_span();
                        number_or_error!(buffer span; $variant)
                    }
                )+
                _ => {
                    span = $cursor.into_span();
                    number_or_error!(buffer span; Int)
                }
            };

            return Ok(Literal {
                span,
                value,
            });
        }
    }
}

impl<T: FusedIterator<Item = char>> ParseToken<T> for Literal {
    fn parse(start: char, mut cursor: Cursor<T>) -> ParseResult<Self> {
        if start == '"' {
            let mut buffer = String::new();
            let mut escaped = false;

            loop {
                let next_char = cursor.expect_consume()?;

                if escaped {
                    buffer.push(next_char);
                    escaped = false;
                } else if next_char == '\\' {
                    escaped = true;
                } else if next_char == '"' {
                    break;
                } else {
                    buffer.push(next_char);
                }
            }

            Ok(Literal {
                span: cursor.into_span(),
                value: LiteralType::String(buffer),
            })
        } else if start == '-' || start.is_ascii_digit() {
            parse_number!(
                start, cursor;
                'b' = Byte,
                's' = Short,
                'i' = Int,
                'l' = Long,
                'f' = Float,
                'd' = Double
            );
        } else {
            Err(ParseError::InvalidStart(start, "literal"))
        }
    }

    fn to_token_tree(self) -> TokenTree {
        TokenTree::Literal(self)
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}
