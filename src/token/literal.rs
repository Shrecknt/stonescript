use super::{stream::Stream, ParseError, ParseResult, Span, Token};
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
    Command(String),
}

#[derive(Debug, Clone)]
pub struct Literal {
    pub span: Span,
    pub value: LiteralType,
}

impl<T: FusedIterator<Item = char>> Token<T> for Literal {
    fn parse(reader: &mut Stream<T>) -> ParseResult<Self> {
        let start_pos = reader.position;

        let first_char = reader.expect_peek()?;
        if first_char == '"' {
            let mut buffer = String::new();

            let mut escaped = false;

            reader.advance();
            loop {
                let char = reader.expect_next()?;
                if escaped {
                    buffer.push(char);
                    escaped = false;
                } else if char == '\\' {
                    escaped = true;
                } else if char == '"' {
                    break;
                } else {
                    buffer.push(char);
                }
            }

            Ok(Literal {
                span: Span::new(start_pos, buffer.len() + 2),
                value: LiteralType::String(buffer),
            })
        } else if first_char == '$' {
            let mut buffer = String::new();

            let mut escaped = false;

            reader.advance();
            loop {
                let char = reader.expect_next()?;
                if escaped {
                    if char != ';' {
                        buffer.push('\\');
                    }
                    buffer.push(char);
                    escaped = false;
                } else if char == '\\' {
                    escaped = true;
                } else if char == ';' {
                    break;
                } else {
                    buffer.push(char);
                }
            }

            Ok(Literal {
                span: Span::new(start_pos, buffer.len() + 2),
                value: LiteralType::Command(buffer),
            })
        } else if first_char == '-' || first_char.is_ascii_digit() {
            let mut buffer = String::from(first_char);

            reader.advance();
            loop {
                let char = reader.expect_next()?;
                if char == '.' || char.is_ascii_digit() {
                    buffer.push(char);
                } else {
                    let span = Span::new(start_pos, buffer.len() + 1);
                    return Ok(Literal {
                        span,
                        value: match char {
                            'b' => LiteralType::Byte(buffer.parse().map_err(|_| {
                                ParseError::UnexpectedToken(
                                    buffer,
                                    "byte",
                                    span,
                                )
                            })?),
                            's' => LiteralType::Short(buffer.parse().map_err(|_| {
                                ParseError::UnexpectedToken(
                                    buffer,
                                    "short",
                                    span,
                                )
                            })?),
                            'i' => LiteralType::Int(buffer.parse().map_err(|_| {
                                ParseError::UnexpectedToken(
                                    buffer,
                                    "int",
                                    span,
                                )
                            })?),
                            'l' => LiteralType::Long(buffer.parse().map_err(|_| {
                                ParseError::UnexpectedToken(
                                    buffer,
                                    "long",
                                    span,
                                )
                            })?),
                            'f' => LiteralType::Float(buffer.parse().map_err(|_| {
                                ParseError::UnexpectedToken(
                                    buffer,
                                    "float",
                                    span,
                                )
                            })?),
                            'd' => LiteralType::Double(buffer.parse().map_err(|_| {
                                ParseError::UnexpectedToken(
                                    buffer,
                                    "double",
                                    span,
                                )
                            })?),
                            _ => {
                                reader.position -= 1;
                                break;
                            }
                        },
                    });
                }
            }

            let span = Span::new(start_pos, buffer.len());

            Ok(Literal {
                span,
                value: LiteralType::Int(
                    buffer
                        .parse()
                        .map_err(|_| ParseError::UnexpectedToken(buffer, "int", span))?,
                ),
            })
        } else {
            Err(ParseError::UnexpectedToken(
                first_char.to_string(),
                "literal",
                Span::new(start_pos, 1)
            ))
        }
    }

    fn valid_start(start: char) -> bool {
        start == '"' || start == '$' || start == '-' || start.is_ascii_digit()
    }
}
