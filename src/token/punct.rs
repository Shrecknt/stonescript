use super::{cursor::Cursor, ParseResult, ParseToken, ToTokenTree};
use crate::{ParseError, Span, Spanned, TokenTree};
use std::{fmt, iter::FusedIterator, str::FromStr};

macro_rules! optional_char {
    () => {
        None
    };
    ($char:literal) => {
        Some($char)
    };
}

pub struct InvalidPunct;

macro_rules! define_punct {
    ($($variant:ident => $char1:literal $($char2:literal)?),+) => {
        #[derive(Clone, Copy, PartialEq, Eq)]
        pub enum PunctToken {
            $($variant),+
        }

        impl From<PunctToken> for &'static str {
            fn from(value: PunctToken) -> &'static str {
                match value {
                    $(PunctToken::$variant => concat!($char1 $(, $char2)?),)+
                }
            }
        }

        impl FromStr for PunctToken {
            type Err = InvalidPunct;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $(concat!($char1 $(, $char2)?) => Ok(PunctToken::$variant),)+
                    _ => Err(InvalidPunct),
                }
            }
        }

        pub mod ast {
            use crate::{Span, Spanned, TokenTree, Sealed, token::{Token, ToTokenTree}};
            use super::{Punct, PunctToken};

            $(
                #[derive(Debug, Clone, Copy, PartialEq, Eq)]
                pub struct $variant {
                    span: Span
                }

                impl $variant {
                    pub fn is_punct(punct: &Punct) -> bool {
                        punct.value == PunctToken::$variant
                    }
                }

                impl From<Punct> for Option<$variant> {
                    fn from(value: Punct) -> Self {
                        if let Punct { span, value: PunctToken::$variant } = value {
                            Some($variant { span })
                        } else {
                            None
                        }
                    }
                }

                impl Sealed for $variant {}
                impl Token for $variant {
                    const NAME: &'static str = stringify!($variant);

                    fn parse_token(token_tree: TokenTree) -> Option<Self> {
                        Punct::parse_token(token_tree)?.into()
                    }
                }

                impl Spanned for $variant {
                    fn span(&self) -> Span {
                        self.span
                    }
                }

                impl ToTokenTree for $variant {
                    fn to_token_tree(self) -> TokenTree {
                        TokenTree::Punct(Punct { span: self.span, value: PunctToken::$variant })
                    }
                }
            )+
        }

        const PUNCT_MAP: &[(PunctToken, char, Option<char>)] = &[
            $((PunctToken::$variant, $char1, optional_char!($($char2)?))),+
        ];
    };
}

define_punct!(
    Plus => '+',
    Minus => '-',
    Star => '*',
    Slash => '/',
    Percent => '%',
    LessThan => '<',
    LessThanEquals => '<' '=',
    GreaterThan => '>',
    GreaterThanEquals => '>' '=',
    Assign => '=',
    Not => '!',
    NotEquals => '!' '=',
    Dot => '.',
    Ternary => '?',
    Colon => ':',
    Equals => '=' '=',
    And => '&' '&',
    Or => '|' '|',
    NullishCoalescing => '?' '?',
    Semicolon => ';',
    Comma => ',',
    Lambda => '-' '>'
);

impl fmt::Debug for PunctToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if f.alternate() {
            if let PunctToken::Semicolon = self {
                return f.write_str(";\n");
            }
        }
        
        f.write_str(<&'static str>::from(*self))
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Punct {
    span: Span,
    value: PunctToken,
}

impl Punct {
    pub fn inner(&self) -> PunctToken {
        self.value
    }
}

impl Spanned for Punct {
    fn span(&self) -> Span {
        self.span
    }
}

impl ToTokenTree for Punct {
    fn to_token_tree(self) -> TokenTree {
        TokenTree::Punct(self)
    }
}

impl<T: FusedIterator<Item = char>> ParseToken<T> for Punct {
    fn parse(start: char, mut cursor: Cursor<T>) -> ParseResult<Self> {
        if let Some(next_char) = cursor.peek() {
            let mut candidate = None;

            for (token, char1, char2) in PUNCT_MAP {
                if start != *char1 {
                    continue;
                }

                if let Some(char2) = *char2 {
                    if next_char == char2 {
                        candidate = Some(*token)
                    }
                } else if candidate.is_none() {
                    candidate = Some(*token)
                }
            }

            if let Some(candidate) = candidate {
                Ok(Punct {
                    span: cursor.into_span(),
                    value: candidate,
                })
            } else {
                Err(ParseError::InvalidStart(start, "punct"))
            }
        } else {
            let (token, _, _) = PUNCT_MAP
                .iter()
                .find(|(_, char1, char2)| start == *char1 && char2.is_none())
                .ok_or(ParseError::InvalidStart(start, "punct"))?;
            Ok(Punct {
                span: cursor.into_span(),
                value: *token,
            })
        }
    }
}

impl fmt::Debug for Punct {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.value)
    }
}
