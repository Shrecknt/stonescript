use super::{cursor::Cursor, Group, Ident, Literal, ParseResult, Punct, Token, ToTokenTree};
use crate::{Sealed, Span, Spanned};
use std::{fmt, iter::FusedIterator};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Unexpected end of file")]
    EarlyEof,
    #[error("Unexpected {0:?} while parsing {1}")]
    UnexpectedToken(String, &'static str, Span),
    #[error("Invalid starting character {0:?} for {1}")]
    InvalidStart(char, &'static str),
    #[error("This is not a token")]
    NotAToken,
}

macro_rules! define_token_tree {
    ($($token:ident),+) => {
        #[derive(Debug, Clone, PartialEq)]
        pub enum TokenTree {
            $($token($token),)+
        }

        impl Spanned for TokenTree {
            fn span(&self) -> Span {
                match self {
                    $(Self::$token(value) => value.span(),)+
                }
            }
        }

        impl fmt::Display for TokenTree {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match self {
                    $(Self::$token(value) => write!(f, "{}", value),)+
                }
            }
        }

        $(
            impl Sealed for $token {}
            impl Token for $token {
                const NAME: &'static str = stringify!($token);

                fn parse_token(token_tree: TokenTree) -> Option<Self> {
                    if let TokenTree::$token(value) = token_tree {
                        Some(value)
                    } else {
                        None
                    }
                }
            }
        )+
    }
}

define_token_tree!(Punct, Ident, Literal, Group);

impl ToTokenTree for TokenTree {
    fn to_token_tree(self) -> TokenTree {
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TokenStream(pub Vec<TokenTree>);
impl TokenStream {
    pub fn new() -> Self {
        Self(vec![])
    }
}

impl From<Vec<TokenTree>> for TokenStream {
    fn from(value: Vec<TokenTree>) -> Self {
        Self(value)
    }
}

impl fmt::Display for TokenStream {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(
            &self
                .0
                .iter()
                .map(|tt| format!("{}", tt))
                .fold(String::new(), |a, b| a + &b + " ")
                .trim_end(),
        )
    }
}

pub fn parse_str(value: &str) -> ParseResult<TokenStream> {
    parse_iter(value.chars())
}

pub fn parse_iter<T: FusedIterator<Item = char>>(mut iterator: T) -> ParseResult<TokenStream> {
    Cursor::run(&mut iterator, |mut cursor| {
        let mut tokens = vec![];

        while let Some(next_char) = cursor.peek() {
            if next_char.is_whitespace() {
                cursor.consume();
                continue;
            }

            if let Some(token) = cursor.apply_parsers()? {
                tokens.push(token)
            }
        }

        Ok(tokens.into())
    })
}
