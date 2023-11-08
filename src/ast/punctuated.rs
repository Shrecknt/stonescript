use crate::{token::{Token, ToTokenTree}, Parse, SyntaxError, SyntaxResult, TokenIter, Spanned, Span, TokenTree};

use super::{span_of_two, ToTokens};

#[derive(Debug, Clone, PartialEq)]
pub struct Punctuated<T: Parse, P: Token> {
    inner: Vec<(T, P)>,
    last: Option<Box<T>>,
}

impl<T: Parse, P: Token> Punctuated<T, P> {
    pub fn is_empty(&self) -> bool {
        self.inner.len() == 0 && self.last.is_none()
    }
}

impl<T: Parse, P: Token> Parse for Punctuated<T, P> {
    fn parse(token_iter: &mut TokenIter) -> SyntaxResult<Self> {
        let mut inner = vec![];
        let mut last = None;

        loop {
            match token_iter.parse() {
                Ok(token) => {
                    if let Some(punct) = token_iter.parse()? {
                        inner.push((token, punct))
                    } else {
                        last = Some(Box::new(token));
                        break;
                    }
                }
                Err(SyntaxError::EarlyEof) => break,
                Err(other_err) => return Err(other_err),
            }
        }

        Ok(Self { inner, last })
    }
}

impl<T: Parse + Spanned, P: Token + Spanned> Spanned for Punctuated<T, P> {
    fn span(&self) -> Span {
        if let Some(last) = &self.last {
            if let Some(first) = self.inner.first() {
                span_of_two(first.0.span(), last.span())
            } else {
                last.span()
            }
        } else {
            if let [item] = self.inner.as_slice() {
                span_of_two(item.0.span(), item.1.span())
            } else if let [start_item, .., end_item] = self.inner.as_slice() {
                span_of_two(start_item.0.span(), end_item.1.span())
            } else {
                panic!("cannot calculate span of empty punctuated")
            }
        }
    }
}

impl<T: Parse + ToTokens, P: Token + Spanned + ToTokenTree> ToTokens for Punctuated<T, P> {
    fn write_into_stream(self, stream: &mut Vec<TokenTree>) {
        for (token, punct) in self.inner {
            token.write_into_stream(stream);
            punct.write_into_stream(stream);
        }

        if let Some(token) = self.last {
            token.write_into_stream(stream)
        }
    }
}