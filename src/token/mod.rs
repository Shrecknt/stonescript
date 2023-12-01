use self::cursor::Cursor;
pub use self::{
    group::{hir::*, Delimiter, Group},
    ident::{Ident, InvalidXID, XID},
    keyword::*,
    literal::{Literal, LiteralType},
    punct::{hir::*, InvalidPunct, Punct, PunctToken},
    reader::parse_from_reader,
};
use crate::{ParseError, Sealed, Spanned, TokenTree};
use std::iter::FusedIterator;

mod cursor;
mod group;
mod ident;
mod keyword;
mod literal;
pub(super) mod prelude;
mod punct;
mod reader;

type ParseResult<T> = Result<T, ParseError>;

pub trait ToTokenTree: Spanned {
    fn to_token_tree(self) -> TokenTree;
}

trait ParseToken<T: FusedIterator<Item = char>>
where
    Self: Sized + ToTokenTree,
{
    fn parse(start: char, cursor: Cursor<T>) -> ParseResult<Self>;
    fn parse_to_token_tree(start: char, cursor: Cursor<T>) -> ParseResult<TokenTree> {
        Ok(Self::to_token_tree(Self::parse(start, cursor)?))
    }
}

pub trait Token: Sized + ToTokenTree + Sealed {
    const NAME: &'static str;
    fn parse_token(token_tree: TokenTree) -> Option<Self>;
}
