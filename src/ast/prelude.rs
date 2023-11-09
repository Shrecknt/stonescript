pub use super::parse::{Parse, TokenIter};
use crate::TokenTree;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SyntaxError {
    #[error("Unexpected token '{0:?}', expected {1}")]
    UnexpectedToken(TokenTree, &'static str),
    #[error("Unexpected end of file")]
    EarlyEof,
}

pub type SyntaxResult<T> = Result<T, SyntaxError>;

#[macro_export]
#[doc(hidden)]
macro_rules! _totoken_field {
    ($stream:ident Option<$inner:ty> = $value:expr) => {
        if let Some(value) = $value {
            value.write_into_stream($stream)
        }
    };
    ($stream:ident (Parenthesis, $inner:ty) = $value:expr) => {
        $value.0.into_group($value.1).write_into_stream($stream)
    };
    ($stream:ident (Brace, $inner:ty) = $value:expr) => {
        $value.0.into_group($value.1).write_into_stream($stream)
    };
    ($stream:ident (Bracket, $inner:ty) = $value:expr) => {
        $value.0.into_group($value.1).write_into_stream($stream)
    };
    ($stream:ident $other:ty = $value:expr) => {
        $value.write_into_stream($stream)
    }
}

#[macro_export]
#[doc(hidden)]
macro_rules! _parse_field {
    ($token_iter:ident (Parenthesis, $inner:ty)) => {
        $crate::ast::parenthesized($token_iter.parse()?)?
    };
    ($token_iter:ident (Brace, $inner:ty)) => {
        $crate::ast::braced($token_iter.parse()?)?
    };
    ($token_iter:ident (Bracket, $inner:ty)) => {
        $crate::ast::bracketed($token_iter.parse()?)?
    };
    ($token_iter:ident $other:ty) => {
        $token_iter.parse()?
    }
}

#[macro_export]
macro_rules! ast_item {
    ($vis:vis struct $ident:ident { $($fident:ident: $fty:tt $(<$($gen:ty),+>)?),+ }) => {
        #[derive(Debug, Clone, PartialEq)]
        $vis struct $ident {
            $(
                pub $fident: $fty $(<$($gen),+>)?,
            )+
        }

        impl $crate::Parse for $ident {
            fn parse(token_iter: &mut $crate::TokenIter) -> $crate::SyntaxResult<Self> {
                $(
                    let $fident = $crate::_parse_field!(token_iter $fty);
                )+

                Ok(Self {
                    $(
                        $fident,
                    )+
                })
            }
        }

        impl $crate::ast::ToTokens for $ident {
            fn write_into_stream(self, stream: &mut Vec<$crate::TokenTree>) {
                $(
                    $crate::_totoken_field!(stream $fty $(<$($gen),+>)? = self.$fident);
                )+
            }
        }
    };
    ($vis:vis enum $ident:ident { $($variant:ident($inner:ty)),+ }) => {
        #[derive(Debug, Clone, PartialEq)]
        $vis enum $ident {
            $(
                $variant($inner),
            )+
        }

        impl $crate::Spanned for $ident {
            fn span(&self) -> Span {
                match self {
                    $(
                        Self::$variant(inner) => inner.span(),
                    )+
                }
            }
        }

        impl $crate::ast::ToTokens for $ident {
            fn write_into_stream(self, stream: &mut Vec<$crate::TokenTree>) {
                match self {
                    $(
                        Self::$variant(inner) => inner.write_into_stream(stream),
                    )+
                }
            }
        }
    }
}