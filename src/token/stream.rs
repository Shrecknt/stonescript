use super::{ParseError, ParseResult, TokenTree, punct::{Punct, PunctToken}, Token};
use std::{
    io::{self, BufReader, Read},
    iter::FusedIterator,
    str::Chars,
};

pub struct CharReader<'a, T: Read> {
    reader: BufReader<&'a mut T>,
}

impl<'a, T: Read> CharReader<'a, T> {
    pub fn new(reader: &'a mut T) -> Self {
        Self {
            reader: BufReader::new(reader),
        }
    }

    fn read_bytes<const N: usize>(&mut self) -> io::Result<Option<[u8; N]>> {
        let mut buffer = [0; N];

        if let Err(err) = self.reader.read_exact(&mut buffer) {
            if err.kind() == io::ErrorKind::UnexpectedEof {
                return Ok(None);
            } else {
                return Err(err);
            }
        };

        Ok(Some(buffer))
    }

    pub fn next(&mut self) -> io::Result<Option<char>> {
        let first_byte = match self.read_bytes::<1>()? {
            Some(byte_buf) => byte_buf[0],
            None => return Ok(None),
        };

        Ok(match first_byte.leading_ones() {
            0 => Some(first_byte as char),
            2 => {
                let following_byte = match self.read_bytes::<1>()? {
                    Some(byte_buf) => byte_buf[0],
                    None => return Ok(None),
                };
                
                let mut value: u32 = 0;
                value += first_byte as u32 & 0b00001111;
                value <<= 6;
                value += following_byte as u32 & 0b00111111;

                char::from_u32(value)
            }
            3 => {
                let following_bytes = match self.read_bytes::<2>()? {
                    Some(byte_buf) => byte_buf,
                    None => return Ok(None),
                };

                let mut value: u32 = 0;
                value += first_byte as u32 & 0b00001111;
                value <<= 6;
                value += following_bytes[0] as u32 & 0b00111111;
                value <<= 6;
                value += following_bytes[1] as u32 & 0b00111111;

                char::from_u32(value)
            }
            4 => {
                let following_bytes = match self.read_bytes::<3>()? {
                    Some(byte_buf) => byte_buf,
                    None => return Ok(None),
                };
                
                let mut value: u32 = 0;
                value += first_byte as u32 & 0b00001111;
                value <<= 6;
                value += following_bytes[0] as u32 & 0b00111111;
                value <<= 6;
                value += following_bytes[1] as u32 & 0b00111111;
                value <<= 6;
                value += following_bytes[2] as u32 & 0b00111111;

                char::from_u32(value)
            }
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("invalid first utf-8 byte: {}", first_byte),
                ))
            }
        })
    }
}

impl<'a, T: Read> Iterator for CharReader<'a, T> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if let Ok(value) = self.next() {
            value
        } else {
            None
        }
    }
}

impl<'a, T: Read> FusedIterator for CharReader<'a, T> {}

#[derive(Debug)]
pub struct Stream<'a, T: FusedIterator<Item = char>> {
    iterator: &'a mut T,
    buffer: Vec<char>,
    pub position: usize,
}

impl<'a, T: FusedIterator<Item = char>> Stream<'a, T> {
    pub fn new(iterator: &'a mut T) -> Self {
        Self {
            iterator,
            buffer: vec![],
            position: 0,
        }
    }

    pub fn peek(&mut self) -> Option<char> {
        if let Some(char) = self.buffer.get(self.position) {
            Some(*char)
        } else if let Some(char) = self.iterator.next() {
            self.buffer.push(char);
            self.peek()
        } else {
            None
        }
    }

    pub fn expect_peek(&mut self) -> ParseResult<char> {
        self.peek().ok_or(ParseError::EarlyEof)
    }

    pub fn next(&mut self) -> Option<char> {
        let ret = self.peek();
        self.advance();
        ret
    }

    pub fn expect_next(&mut self) -> ParseResult<char> {
        self.next().ok_or(ParseError::EarlyEof)
    }

    pub fn advance(&mut self) {
        self.position += 1;
    }

    pub fn tokenise(&mut self, closing_char: Option<char>) -> ParseResult<Vec<TokenTree>> {
        let mut tokens = vec![];
    
        loop {
            let next_char = if let Some(closing_char) = closing_char {
                if let Some(next_char) = self.peek() {
                    if next_char == closing_char {
                        self.advance();
                        break;
                    } else {
                        next_char
                    }
                } else {
                    return Err(ParseError::EarlyEof);
                }
            } else {
                if let Some(next_char) = self.peek() {
                    next_char
                } else {
                    break;
                }
            };
    
            if next_char.is_whitespace() {
                self.advance();
                continue;
            }
    
            match TokenTree::parse(self)? {
                TokenTree::Punct(Punct {
                    span: _,
                    token: PunctToken::Comment,
                }) => {
                    while let Some(next_char) = self.next() {
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
    
}

impl<'a, T: FusedIterator<Item = char>> Iterator for Stream<'a, T> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }
}

impl<'a, T: FusedIterator<Item = char>> FusedIterator for Stream<'a, T> {}

impl<'a, 'b, T: Read> From<&'a mut CharReader<'b, T>> for Stream<'a, CharReader<'b, T>> {
    fn from(value: &'a mut CharReader<'b, T>) -> Self {
        Stream::new(value)
    }
}

impl<'a> From<&'a mut Chars<'a>> for Stream<'a, Chars<'a>> {
    fn from(value: &'a mut Chars<'a>) -> Self {
        Stream::new(value)
    }
}
