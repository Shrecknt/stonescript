use super::{cursor::Cursor, ParseError, ParseResult, ParseToken, ToTokenTree, TokenTree};
use crate::{Span, Spanned, TokenStream};
use std::{
    fmt::{self, Write},
    iter::FusedIterator,
};

macro_rules! define_delimiter {
    ($($variant:ident => $open:literal $close:literal),+) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Delimiter {
            $($variant,)+
        }

        impl Delimiter {
            pub fn from_open(open: char) -> Option<Self> {
                match open {
                    $($open => Some(Self::$variant),)+
                    _ => None,
                }
            }

            pub fn open(&self) -> char {
                match self {
                    $(Self::$variant => $open,)+
                }
            }

            pub fn close(&self) -> char {
                match self {
                    $(Self::$variant => $close,)+
                }
            }
        }

        pub mod ast {
            use crate::{Span, Spanned, token::{Group, Delimiter}, ast::ToTokens};

            $(
                #[derive(Debug, Clone, Copy, PartialEq, Eq)]
                pub struct $variant {
                    span: Span,
                }

                impl $variant {
                    pub(crate) fn new(span: Span) -> Self {
                        Self { span }
                    }

                    pub fn open(&self) -> char {
                        $open
                    }

                    pub fn close(&self) -> char {
                        $close
                    }

                    pub fn into_group<T: ToTokens>(self, contents: T) -> Group {
                        Group {
                            span: self.span,
                            delimiter: Delimiter::$variant,
                            tokens: contents.into_tokens(),
                        }
                    }
                }

                impl Spanned for $variant {
                    fn span(&self) -> Span {
                        self.span
                    }
                }
            )+
        }
    }
}

define_delimiter!(
    Brace => '{' '}',
    Bracket => '[' ']',
    Parenthesis => '(' ')'
);

#[derive(Clone, PartialEq)]
pub struct Group {
    span: Span,
    delimiter: Delimiter,
    tokens: TokenStream,
}

impl Group {
    pub fn delimiter(&self) -> Delimiter {
        self.delimiter
    }

    pub fn tokens(&self) -> &TokenStream {
        &self.tokens
    }

    pub fn into_tokens(self) -> TokenStream {
        self.tokens
    }
}

impl Spanned for Group {
    fn span(&self) -> Span {
        self.span
    }
}

impl ToTokenTree for Group {
    fn to_token_tree(self) -> TokenTree {
        TokenTree::Group(self)
    }
}

impl<T: FusedIterator<Item = char>> ParseToken<T> for Group {
    fn parse(start: char, mut cursor: Cursor<T>) -> ParseResult<Self> {
        let delimiter =
            Delimiter::from_open(start).ok_or(ParseError::InvalidStart(start, "group"))?;
        let closing_char = delimiter.close();
        let mut tokens = vec![];

        loop {
            let next_char = cursor.expect_peek()?;
            if next_char == closing_char {
                break;
            }

            if next_char.is_whitespace() {
                cursor.consume();
                continue;
            }

            if let Some(token) = cursor.apply_parsers()? {
                tokens.push(token)
            }
        }

        cursor.consume();

        Ok(Group {
            span: cursor.into_span(),
            delimiter,
            tokens: tokens.into(),
        })
    }
}

impl fmt::Debug for Group {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_char(self.delimiter.open())?;

        if f.alternate() && self.tokens.0.len() > 1 {
            f.write_char('\n')?;

            for s in format!("{:#?}", self.tokens).split('\n') {
                f.write_str("    ")?;
                f.write_str(s)?;
                f.write_char('\n')?;
            }
        } else {
            self.tokens.fmt(f)?;
        }

        f.write_char(self.delimiter.close())?;
        if f.alternate() {
            if let Delimiter::Brace = self.delimiter {
                f.write_char('\n')?;
                f.write_char('\n')?;
            }
        }

        Ok(())
    }
}
