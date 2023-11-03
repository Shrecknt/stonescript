use super::{
    group::Group, ident::Ident, literal::Literal, punct::Punct, ParseError, ParseResult,
    ParseToken, TokenTree,
};
use crate::Span;
use std::{collections::VecDeque, iter::FusedIterator};

struct CursorInner<'i, T: FusedIterator<Item = char> + 'i> {
    iterator: &'i mut T,
    position: usize,
    buffer: VecDeque<char>,
}

pub struct Cursor<'a, T: FusedIterator<Item = char> + 'a> {
    inner: *mut CursorInner<'a, T>,
    start_pos: usize,
}

impl<'a, T: FusedIterator<Item = char> + 'a> Cursor<'a, T> {
    fn inner<'z>(&'z self) -> &'z CursorInner<'a, T> {
        // SAFETY: Cursor can only be created in Cursor::run, and is destroyed before the CursorInner
        unsafe { &*self.inner }
    }

    fn inner_mut<'z>(&'z mut self) -> &'z mut CursorInner<'a, T> {
        // SAFETY: Cursor can only be created in Cursor::run, and is destroyed before the CursorInner
        unsafe { &mut *self.inner }
    }

    fn skip_comment(start: char, mut cursor: Cursor<T>) -> ParseResult<TokenTree> {
        if start == '/' && cursor.peek() == Some('/') {
            while let Some(char) = cursor.consume() {
                if char == '\n' || char == '\r' {
                    break;
                }
            }

            Err(ParseError::NotAToken)
        } else {
            Err(ParseError::InvalidStart(start, "comment"))
        }
    }

    const PARSERS: &'a [fn(char, Cursor<T>) -> ParseResult<TokenTree>] = &[
        Self::skip_comment,
        Group::parse_to_token_tree,
        Literal::parse_to_token_tree,
        Punct::parse_to_token_tree,
        Ident::parse_to_token_tree,
    ];

    pub fn run<R>(iterator: &'a mut T, mut closure: impl FnMut(Cursor<T>) -> R) -> R {
        let mut inner = Box::new(CursorInner {
            iterator,
            position: 0,
            buffer: VecDeque::new(),
        });

        closure(Cursor {
            inner: &mut *inner,
            start_pos: 0,
        })
    }

    pub(super) fn fill(&mut self, amount: usize) {
        let iter: Vec<char> = self
            .inner_mut()
            .iterator
            .take(amount)
            .collect();
        self.inner_mut().buffer.extend(iter)
    }

    pub(super) fn peek_ahead(&mut self, index: usize) -> Option<char> {
        self.inner().buffer.get(index).copied().or_else(|| {
            self.fill(index - self.inner().buffer.len() + 1);
            self.inner().buffer.get(index).copied()
        })
    }

    pub(super) fn expect_peek_ahead(&mut self, index: usize) -> ParseResult<char> {
        self.peek_ahead(index).ok_or(ParseError::EarlyEof)
    }

    pub(super) fn peek(&mut self) -> Option<char> {
        self.peek_ahead(0)
    }

    pub(super) fn expect_peek(&mut self) -> ParseResult<char> {
        self.expect_peek_ahead(0)
    }

    pub(super) fn consume(&mut self) -> Option<char> {
        let inner = self.inner_mut();
        inner.position += 1;
        if inner.buffer.is_empty() {
            inner.iterator.next()
        } else {
            inner.buffer.pop_front()
        }
    }

    pub(super) fn expect_consume(&mut self) -> ParseResult<char> {
        self.consume().ok_or(ParseError::EarlyEof)
    }

    pub(super) fn into_span(self) -> Span {
        Span {
            index: self.start_pos,
            width: self.inner().position - self.start_pos,
        }
    }

    pub(super) fn apply_parsers(&mut self) -> ParseResult<Option<TokenTree>> {
        let start_pos = self.inner().position;
        let start = self.expect_consume()?;

        for parser in Self::PARSERS {
            let cursor = Self {
                inner: self.inner.clone(),
                start_pos,
            };

            return match parser(start, cursor) {
                Ok(token) => Ok(Some(token)),
                Err(ParseError::NotAToken) => Ok(None),
                Err(ParseError::InvalidStart(_, _)) => continue,
                Err(other_err) => Err(other_err),
            };
        }

        Err(ParseError::UnexpectedToken(
            start.to_string(),
            "token tree",
            Span::new(self.inner().position, 1),
        ))
    }
}
