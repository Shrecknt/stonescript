use super::ToTokens;
use crate::{
    token::{ToTokenTree, Token},
    Parse, Span, Spanned, SyntaxError, SyntaxResult, TokenIter, TokenTree,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Punctuated<T, P> {
    inner: Vec<(T, P)>,
    last: Option<Box<T>>,
}

impl<T, P> Punctuated<T, P> {
    pub fn into_tokens(self) -> Vec<T> {
        let mut map: Vec<T> = self.inner.into_iter().map(|(token, _)| token).collect();

        if let Some(last) = self.last {
            map.push(*last);
        }

        map
    }

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

impl<T: Spanned, P: Spanned> Spanned for Punctuated<T, P> {
    fn span(&self) -> Span {
        if let Some(last) = &self.last {
            if let Some(first) = self.inner.first() {
                Span::from_start_end(first.0.span(), last.span())
            } else {
                last.span()
            }
        } else {
            if let [item] = self.inner.as_slice() {
                Span::from_start_end(item.0.span(), item.1.span())
            } else if let [start_item, .., end_item] = self.inner.as_slice() {
                Span::from_start_end(start_item.0.span(), end_item.1.span())
            } else {
                panic!("cannot calculate span of empty punctuated")
            }
        }
    }
}

impl<T: ToTokens, P: ToTokenTree> ToTokens for Punctuated<T, P> {
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
