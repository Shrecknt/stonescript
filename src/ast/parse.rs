use crate::{token::Token, SyntaxError, SyntaxResult, TokenStream, TokenTree};
use std::{collections::VecDeque, slice::Iter};

pub struct TokenIter<'a> {
    iterator: Iter<'a, TokenTree>,
    buffer: VecDeque<TokenTree>,
}

impl TokenIter<'_> {
    fn fill(&mut self, amount: usize) {
        for _ in 0..amount {
            if let Some(item) = self.iterator.next() {
                self.buffer.push_back(item.clone())
            }
        }
    }

    pub(crate) fn peek_ahead<'a>(&'a mut self, index: usize) -> Option<&'a TokenTree> {
        // SAFETY: `self.buffer` needs to be immutably borrowed for `'a`
        //         However, we need to borrow self mutably for `'2` to run `self.fill`
        //         The item reference does not exist when `self.fill` is run,
        //         and as such we can cast `'1` to `'a`

        if let Some(item) = self.buffer.get(index) {
            return Some(unsafe { &*(item as *const TokenTree) });
        }

        self.fill(index - self.buffer.len() + 1);
        self.buffer.get(index)
    }

    pub(crate) fn peek(&mut self) -> Option<&TokenTree> {
        self.peek_ahead(0)
    }

    pub(crate) fn consume(&mut self) -> Option<TokenTree> {
        if self.buffer.is_empty() {
            self.iterator.next().cloned()
        } else {
            self.buffer.pop_front()
        }
    }

    pub(crate) fn expect_peek_ahead(&mut self, index: usize) -> SyntaxResult<&TokenTree> {
        self.peek_ahead(index).ok_or(SyntaxError::EarlyEof)
    }

    pub(crate) fn expect_peek(&mut self) -> SyntaxResult<&TokenTree> {
        self.expect_peek_ahead(0)
    }

    pub(crate) fn expect_consume(&mut self) -> SyntaxResult<TokenTree> {
        self.consume().ok_or(SyntaxError::EarlyEof)
    }

    pub fn parse<T: Parse>(&mut self) -> SyntaxResult<T> {
        T::parse(self)
    }
}

impl<'a> From<&'a TokenStream> for TokenIter<'a> {
    fn from(value: &'a TokenStream) -> Self {
        Self {
            iterator: value.0.iter(),
            buffer: VecDeque::new(),
        }
    }
}

pub trait Parse
where
    Self: Sized,
{
    fn parse(token_iter: &mut TokenIter) -> SyntaxResult<Self>;
}

impl<T: Token> Parse for T {
    fn parse(token_iter: &mut TokenIter) -> SyntaxResult<Self> {
        let token = token_iter.expect_consume()?;
        if let Some(value) = Self::parse_token(token.clone()) {
            Ok(value)
        } else {
            Err(SyntaxError::UnexpectedToken(token, T::NAME))
        }
    }
}

impl<T: Token> Parse for Option<T> {
    fn parse(token_iter: &mut TokenIter) -> SyntaxResult<Self> {
        Ok(if let Some(token) = token_iter.peek() {
            if let Some(value) = T::parse_token(token.clone()) {
                token_iter.consume();
                Some(value)
            } else {
                None
            }
        } else {
            None
        })
    }
}

impl<T: Parse> Parse for Vec<T> {
    fn parse(token_iter: &mut TokenIter) -> SyntaxResult<Self> {
        let mut items = vec![];

        while let Some(_) = token_iter.peek() {
            items.push(token_iter.parse()?);
        }

        Ok(items)
    }
}
