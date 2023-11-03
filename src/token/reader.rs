use super::ParseResult;
use crate::{parse_iter, TokenStream};
use std::{
    io::{self, BufReader, Read},
    iter::FusedIterator,
};

pub struct CharReader<T: Read> {
    reader: BufReader<T>,
}

impl<T: Read> CharReader<T> {
    pub fn new(reader: T) -> Self {
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

impl<T: Read> Iterator for CharReader<T> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if let Ok(value) = self.next() {
            value
        } else {
            None
        }
    }
}

impl<T: Read> FusedIterator for CharReader<T> {}

pub fn parse_from_reader<T: Read>(reader: T) -> ParseResult<TokenStream> {
    parse_iter(CharReader::new(reader))
}
