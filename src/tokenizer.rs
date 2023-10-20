use lazy_static::lazy_static;
use std::collections::HashMap;

use crate::stream::Stream;
use crate::tokens::{
    TokenType, TokenTypeSpecific, MAX_TOKEN_LENGTH, TOKENS_MAP, TOKEN_TYPE_SPECIFIC_MAP,
};

lazy_static! {
    static ref FLOAT_TYPES: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("d", "double");
        m.insert("f", "float");
        m
    };
    static ref INT_TYPES: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("i", "int");
        m.insert("s", "short");
        m.insert("l", "long");
        m
    };
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub specific: TokenTypeSpecific,
    pub value: String,
    pub position: Position,
}

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub row: u64,
    pub col: u64,
    pub position: usize,
    pub token_length: usize,
}

pub fn tokenize(stream: &mut Stream<char>) -> Result<Stream<Token>, eyre::Report> {
    let mut res = vec![];
    let mut token = read_next(stream);
    while let Some(ref cur_token) = token {
        res.push(cur_token.clone());
        token = read_next(stream);
    }

    Ok(res.into())
}

fn read_next(stream: &mut Stream<char>) -> Option<Token> {
    skip_whitespace(stream);
    if stream.eof(false) {
        return None;
    }
    let char = stream.peek(false, 0).unwrap();
    if char == '"' {
        return Some(read_string(stream));
    }
    if char.is_ascii_alphabetic() || char == '_' {
        return Some(read_word_like(stream));
    }
    {
        let token = read_token(stream);
        if token.is_some() {
            return token;
        }
    }
    if char.is_ascii_digit() || char == '-' {
        return Some(read_number(stream));
    }

    stream.yeet(format!("Unexpected token '{}'", char));
    unreachable!()
}

fn skip_whitespace(stream: &mut Stream<char>) {
    loop {
        let next = stream.peek(false, 0);
        if next.is_some() {
            let next = next.unwrap();
            if next.is_ascii_whitespace() {
                stream.skip();
                continue;
            }
        }
        return;
    }
}

#[inline]
fn get_or_escaped(char: char) -> char {
    match char {
        't' => '\t',
        'n' => '\n',
        _ => char,
    }
}

fn read_string(stream: &mut Stream<char>) -> Token {
    let mut position = Position {
        row: stream.row.unwrap(),
        col: stream.col.unwrap(),
        position: stream.position,
        token_length: 0,
    };
    stream.skip();
    let mut escaped = false;
    let mut string = String::new();
    loop {
        if stream.eof(false) {
            stream.yeet("Unexpected end of string literal".to_string());
            unreachable!()
        }
        let char = stream.next();
        if escaped {
            string.push(get_or_escaped(char));
            escaped = false;
            continue;
        }
        if char == '\\' {
            escaped = true;
            continue;
        }
        if char == '"' {
            position.token_length = string.len() + 2;
            return Token {
                token_type: TokenType::String,
                specific: TokenTypeSpecific::Literal,
                value: string,
                position,
            };
        }
        string.push(char);
    }
}

fn read_word_like(stream: &mut Stream<char>) -> Token {
    let mut position = Position {
        row: stream.row.unwrap(),
        col: stream.col.unwrap(),
        position: stream.position,
        token_length: 0,
    };
    let mut word = String::new();
    while !stream.eof(false) {
        let peek = stream.peek(false, 0).unwrap();
        if !(peek.is_ascii_alphabetic() || peek == '_') {
            break;
        }
        let char = stream.next();
        word.push(char);
    }
    position.token_length = word.len();
    Token {
        token_type: TokenType::Word,
        specific: TokenTypeSpecific::Generic,
        value: word,
        position,
    }
}

fn read_token(stream: &mut Stream<char>) -> Option<Token> {
    if stream.eof(false) {
        return None;
    }
    let mut position = Position {
        row: stream.row.unwrap(),
        col: stream.col.unwrap(),
        position: stream.position,
        token_length: 0,
    };
    let mut token = String::new();
    let mut i = MAX_TOKEN_LENGTH;
    'length_loop: while i > 0 {
        i -= 1;
        let mut test = String::new();
        for j in 0..i {
            let val = stream.peek(false, j);
            if val.is_none() {
                continue 'length_loop;
            }
            test.push(val.unwrap());
        }
        if TOKENS_MAP.get(test.as_str()).is_some() {
            token = test;
            break;
        }
    }
    if token == "" {
        return None;
    }
    position.token_length = token.len();
    for _ in 0..token.len() {
        stream.skip();
    }
    Some(Token {
        token_type: TokenType::Token,
        specific: *TOKENS_MAP.get(token.as_str()).unwrap(),
        value: token,
        position,
    })
}

fn read_number(stream: &mut Stream<char>) -> Token {
    let mut position = Position {
        row: stream.row.unwrap(),
        col: stream.col.unwrap(),
        position: stream.position,
        token_length: 0,
    };
    let mut number = String::new();
    let mut has_decimal = false;
    if stream.peek(false, 0) == Some('-') {
        number.push('-');
        stream.skip();
    }
    loop {
        if stream.eof(false) {
            position.token_length = number.len();
            if has_decimal {
                return Token {
                    token_type: TokenType::Number,
                    specific: TokenTypeSpecific::SignedFloat,
                    value: number,
                    position,
                };
            } else {
                return Token {
                    token_type: TokenType::Number,
                    specific: TokenTypeSpecific::SignedInt,
                    value: number,
                    position,
                };
            }
        }
        let mut char = stream.peek(false, 0).unwrap();
        if char == '.' {
            if has_decimal {
                stream.yeet("Unexpected double decimal point".to_string());
                unreachable!()
            }
            has_decimal = true;
            number.push('.');
            stream.skip();
            continue;
        }
        if char.is_ascii_digit() {
            stream.skip();
            number.push(char);
        } else {
            let mut specific = String::from("signed_");
            let mut parsed_number = number.parse::<f64>().unwrap_or(0.0);
            char = char.to_ascii_lowercase();
            if char == 'u' {
                if number.starts_with('-') {
                    stream.yeet("Unsigned number cannot be negative".to_string());
                    unreachable!()
                }
                position.token_length = number.len() + 2;
                stream.skip();
                parsed_number = parsed_number.trunc();
                if stream.eof(false) {
                    stream.yeet("Ah yes `unsigned EOF` my favorite data type".to_string());
                    unreachable!()
                }
                specific = String::from("unsigned_");
                let append_specific =
                    match INT_TYPES.get(stream.peek(false, 0).unwrap().to_string().as_str()) {
                        Some(append_specific) => append_specific,
                        None => {
                            stream.yeet(format!(
                                "Unknown integer type '{}'",
                                stream.peek(false, 0).unwrap()
                            ));
                            unreachable!()
                        }
                    };
                specific.push_str(append_specific);
                stream.skip();
            } else if char.is_ascii_whitespace()
                || TOKENS_MAP.get(char.to_string().as_str()).is_some()
            {
                position.token_length = number.len();
                if has_decimal {
                    return Token {
                        token_type: TokenType::Number,
                        specific: TokenTypeSpecific::SignedFloat,
                        value: parsed_number.to_string(),
                        position,
                    };
                } else {
                    return Token {
                        token_type: TokenType::Number,
                        specific: TokenTypeSpecific::SignedInt,
                        value: parsed_number.to_string(),
                        position,
                    };
                }
            } else {
                position.token_length = number.len() + 1;
                stream.skip();
                if INT_TYPES.get(char.to_string().as_str()).is_some() {
                    parsed_number = parsed_number.trunc();
                }
                let append_specific = match FLOAT_TYPES.get(char.to_string().as_str()) {
                    Some(r) => r,
                    None => match INT_TYPES.get(char.to_string().as_str()) {
                        Some(r) => r,
                        None => {
                            stream.yeet(format!("Unknown number type '{}'", char));
                            unreachable!()
                        }
                    },
                };
                specific.push_str(append_specific)
            }
            return Token {
                token_type: TokenType::Number,
                specific: *TOKEN_TYPE_SPECIFIC_MAP.get(specific.as_str()).unwrap(),
                value: parsed_number.to_string(),
                position,
            };
        }
    }
}
