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
            use crate::{
                Span, Spanned, TokenIter, SyntaxResult, SyntaxError, Parse, TokenTree,
                token::{Group, Delimiter, ToTokenTree}, ast::ToTokens
            };

            $(
                #[derive(Debug, Clone, PartialEq)]
                pub struct $variant<T> {
                    span: Span,
                    pub contents: T, // GIVE ME PUBLIC ACCESS OR GIVE ME DEATH
                }

                impl<T> $variant<T> {
                    pub fn open(&self) -> char {
                        $open
                    }

                    pub fn close(&self) -> char {
                        $close
                    }

                    pub fn contents(&self) -> &T {
                        &self.contents
                    }
                }

                impl<T: Parse> TryFrom<Group> for $variant<T> {
                    type Error = SyntaxError;

                    fn try_from(value: Group) -> SyntaxResult<Self> {
                        if let Group {
                            span,
                            tokens,
                            delimiter: Delimiter::$variant,
                        } = value {
                            Ok(Self {
                                span,
                                contents: TokenIter::from(&tokens).parse()?,
                            })
                        } else {
                            Err(SyntaxError::UnexpectedToken(value.to_token_tree(), stringify!($variant)))
                        }
                    }
                }

                impl<T: Parse> Parse for $variant<T> {
                    fn parse(token_iter: &mut TokenIter) -> SyntaxResult<Self> {
                        let group: Group = token_iter.parse()?;
                        group.try_into()
                    }
                }

                impl<T> Spanned for $variant<T> {
                    fn span(&self) -> Span {
                        self.span
                    }
                }

                impl<T: ToTokens> ToTokenTree for $variant<T> {
                    fn to_token_tree(self) -> TokenTree {
                        Group {
                            span: self.span,
                            delimiter: Delimiter::$variant,
                            tokens: self.contents.into_tokens(),
                        }.to_token_tree()
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
