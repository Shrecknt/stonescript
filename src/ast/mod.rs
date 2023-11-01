use thiserror::Error;
use crate::token::TokenTree;
use self::{func::Function, declaration::Declaration, assignment::Assignment};

pub mod stream;
pub mod expr;
pub mod func;
pub mod assignment;
pub mod r#type;
pub mod declaration;

#[derive(Debug, Error)]
pub enum SyntaxError {
    #[error("Unexpected token {0:?} while generating AST")]
    UnexpectedToken(TokenTree),
    #[error("Unexpected end of file")]
    EarlyEof,
}

pub type SyntaxResult<T> = Result<T, SyntaxError>;

/// given `expect_token!(token: Enum::Variant[field] = value)`, check if `value`
/// is of type `Enum::Variant`. If it is, return the inner value (`field`) of
/// `value`. If it isn't, return `Err(SyntaxError::UnexpectedToken(token))`. The
/// name `field` doesn't actually matter, it can be whatever you want, just make
/// the number of fields match the number of inner fields in your enum.
///
/// For example, you could assert that the variable `token` is an ident with the
/// expression: `expect_token!(token: TokenTree::Ident[a] = token)`, which then
/// returns the inner field of type `Ident`, and if it isn't an Ident, an error
/// is returned instead.
#[macro_export]
macro_rules! expect_token {
    ($token:ident: $variant:path $( [$($varp:ident),+] )? $( {$($varb:ident),+} )? = $value:expr) => {
        if let $variant $( ( $($varp),+ ) )? $( { $($varb),+ } )? = $value {
            ($( $($varp),+ )? $( $($varb),+ )?)
        } else {
            return Err(SyntaxError::UnexpectedToken($token));
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Block {
        contents: Vec<Statement>,
    },
    Function(Function),
    Call {
        function_name: String,
        arguments: Vec<Statement>,
    },
    Declaration(Declaration),
    Assignment(Assignment),
    Command(String),
}