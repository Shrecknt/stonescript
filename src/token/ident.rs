use super::{cursor::Cursor, ParseError, ParseResult, ParseToken, ToTokenTree, TokenTree};
use crate::{private::Sealed, Span, Spanned};
use std::{fmt, iter::FusedIterator, str::FromStr};
use thiserror::Error;

#[derive(Error, Debug)]
#[error("invalid xid")]
pub struct InvalidXID;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Hash)]
pub struct XID(String);

impl XID {
    pub fn inner(&self) -> &str {
        &self.0
    }
}

impl Sealed for XID {}

impl FromStr for XID {
    type Err = InvalidXID;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();

        let first_char = chars.next().ok_or(InvalidXID)?;
        if !unicode_ident::is_xid_start(first_char) {
            return Err(InvalidXID);
        }

        while let Some(next_char) = chars.next() {
            if !unicode_ident::is_xid_continue(next_char) {
                return Err(InvalidXID);
            }
        }

        Ok(Self(s.to_string()))
    }
}

impl fmt::Display for XID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Clone, PartialEq)]
pub struct Ident {
    span: Span,
    value: XID,
}

impl Ident {
    pub fn inner(&self) -> &str {
        self.value.inner()
    }

    pub fn into_inner(self) -> XID {
        self.value
    }

    pub(crate) fn new_unchecked(span: Span, value: &str) -> Ident {
        Self {
            span,
            value: XID(value.to_string()),
        }
    }
}

impl Spanned for Ident {
    fn span(&self) -> Span {
        self.span
    }
}

impl ToTokenTree for Ident {
    fn to_token_tree(self) -> TokenTree {
        TokenTree::Ident(self)
    }
}

impl<T: FusedIterator<Item = char>> ParseToken<T> for Ident {
    fn parse(start: char, mut cursor: Cursor<T>) -> ParseResult<Self> {
        if !unicode_ident::is_xid_start(start) {
            return Err(ParseError::InvalidStart(start, "ident"));
        }

        let mut buffer = String::from(start);

        while let Some(next_char) = cursor.peek() {
            if unicode_ident::is_xid_continue(next_char) {
                buffer.push(next_char);
                cursor.consume();
            } else {
                break;
            }
        }

        Ok(Ident {
            span: cursor.into_span(),
            value: XID(buffer),
        })
    }
}

impl fmt::Debug for Ident {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.value.0)
    }
}
