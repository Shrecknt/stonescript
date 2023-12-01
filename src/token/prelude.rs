use super::{
    cursor::Cursor, Delimiter, Group, Ident, Literal, ParseResult, Punct, PunctToken, ToTokenTree,
    Token,
};
use crate::{Sealed, Span, Spanned};
use std::{
    fmt::{self, Write},
    iter::FusedIterator,
};
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
        #[derive(Clone, PartialEq)]
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

        impl fmt::Debug for TokenTree {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match self {
                    $(Self::$token(value) => value.fmt(f),)+
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

#[derive(Clone, PartialEq)]
pub struct TokenStream(pub Vec<TokenTree>);
impl TokenStream {
    pub fn new() -> Self {
        Self(vec![])
    }
}

impl Spanned for TokenStream {
    fn span(&self) -> Span {
        self.0.span()
    }
}

impl From<Vec<TokenTree>> for TokenStream {
    fn from(value: Vec<TokenTree>) -> Self {
        Self(value)
    }
}

fn handle_next_char(next_char: Option<&TokenTree>, f: &mut fmt::Formatter) -> fmt::Result {
    if let Some(next_char) = next_char {
        match next_char {
            TokenTree::Group(group) => match group.delimiter() {
                Delimiter::Bracket | Delimiter::Parenthesis => Ok(()),
                Delimiter::Brace => f.write_char(' '),
            },
            TokenTree::Ident(_) => f.write_char(' '),
            TokenTree::Literal(_) => f.write_char(' '),
            TokenTree::Punct(punct) => match punct.inner() {
                PunctToken::And
                | PunctToken::Equals
                | PunctToken::GreaterThan
                | PunctToken::GreaterThanEquals
                | PunctToken::LessThan
                | PunctToken::LessThanEquals
                | PunctToken::Minus
                | PunctToken::Star
                | PunctToken::Slash
                | PunctToken::Plus
                | PunctToken::Percent
                | PunctToken::Or
                | PunctToken::Assign => f.write_char(' '),
                _ => Ok(()),
            },
        }
    } else {
        Ok(())
    }
}

impl fmt::Debug for TokenStream {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut iter = self.0.iter().peekable();
        while let Some(cur_char) = iter.next() {
            let next_char = iter.peek().copied();

            cur_char.fmt(f)?;

            match cur_char {
                TokenTree::Literal(_) => handle_next_char(next_char, f)?,
                TokenTree::Ident(ident) => {
                    if ident.inner() == "for" {
                        if let Some(TokenTree::Group(group)) = next_char {
                            if group.delimiter() == Delimiter::Parenthesis {
                                f.write_char(' ')?;
                            }
                        }
                    }

                    handle_next_char(next_char, f)?
                }
                TokenTree::Group(_) => {
                    if !f.alternate() {
                        handle_next_char(next_char, f)?
                    }
                }
                TokenTree::Punct(punct) => match punct.inner() {
                    PunctToken::And
                    | PunctToken::Equals
                    | PunctToken::GreaterThan
                    | PunctToken::GreaterThanEquals
                    | PunctToken::LessThan
                    | PunctToken::LessThanEquals
                    | PunctToken::Minus
                    | PunctToken::Star
                    | PunctToken::Slash
                    | PunctToken::Plus
                    | PunctToken::Percent
                    | PunctToken::Or
                    | PunctToken::Assign
                    | PunctToken::Colon
                    | PunctToken::Comma => f.write_char(' ')?,
                    PunctToken::Semicolon => {
                        if let Some(_) = next_char {
                            if f.alternate() {
                                f.write_char('\n')?
                            } else {
                                f.write_char(' ')?
                            }
                        }
                    }
                    _ => (),
                },
            }
        }

        Ok(())
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
