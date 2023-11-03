use crate::{Parse, token::Token, TokenIter, SyntaxResult, SyntaxError};

#[derive(Debug, Clone, PartialEq)]
pub struct Punctuated<T: Parse, P: Token> {
    inner: Vec<(T, P)>,
    last: Option<Box<T>>,
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
