use super::{Span, Token};
use crate::{stream::Stream, ExpectChar, ParseError, ParseResult};
use std::iter::FusedIterator;

#[derive(Debug, Clone)]
pub enum LiteralType {
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    String(String),
}

#[derive(Debug, Clone)]
pub struct Literal {
    pub span: Span,
    pub value: LiteralType,
}

impl<T: FusedIterator<Item = char> > Token<T> for Literal {
    fn parse(reader: &mut Stream<T>) -> ParseResult<Self> {
        let start_pos = reader.position;

        let first_char = reader.peek().expect_char()?;
        if first_char == '"' {
            let mut buffer = String::new();

            reader.advance();
            loop {
                let char = reader.next().expect_char()?;
                if char == '"' {
                    break;
                } else {
                    buffer.push(char);
                }
            }

            Ok(Literal {
                span: Span::new(start_pos, buffer.len() + 2),
                value: LiteralType::String(buffer),
            })
        } else if first_char == '-' || first_char.is_ascii_digit() {
            let mut buffer = String::from(first_char);

            reader.advance();
            loop {
                let char = reader.next().expect_char()?;
                if char == '.' || char.is_ascii_digit() {
                    buffer.push(char);
                } else {
                    return Ok(Literal {
                        span: Span::new(start_pos, buffer.len() + 1),
                        value: match char {
                            'b' => LiteralType::Byte(
                                buffer
                                    .parse()
                                    .map_err(|_| ParseError::UnexpectedToken(buffer, "byte"))?,
                            ),
                            's' => LiteralType::Short(
                                buffer
                                    .parse()
                                    .map_err(|_| ParseError::UnexpectedToken(buffer, "short"))?,
                            ),
                            'i' => LiteralType::Int(
                                buffer
                                    .parse()
                                    .map_err(|_| ParseError::UnexpectedToken(buffer, "int"))?,
                            ),
                            'l' => LiteralType::Long(
                                buffer
                                    .parse()
                                    .map_err(|_| ParseError::UnexpectedToken(buffer, "long"))?,
                            ),
                            'f' => LiteralType::Float(
                                buffer
                                    .parse()
                                    .map_err(|_| ParseError::UnexpectedToken(buffer, "float"))?,
                            ),
                            'd' => LiteralType::Double(
                                buffer
                                    .parse()
                                    .map_err(|_| ParseError::UnexpectedToken(buffer, "double"))?,
                            ),
                            _ => {reader.position -= 1; break}
                        },
                    });
                }
            }

            Ok(Literal {
                span: Span::new(start_pos, buffer.len()),
                value: LiteralType::Int(
                    buffer
                        .parse()
                        .map_err(|_| ParseError::UnexpectedToken(buffer, "int"))?,
                ),
            })
        } else {
            Err(ParseError::UnexpectedToken(
                first_char.to_string(),
                "literal",
            ))
        }
    }

    fn valid_start(start: char) -> bool {
        start == '"' || start == '-' || start.is_ascii_digit()
    }
}
