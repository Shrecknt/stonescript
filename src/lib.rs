use thiserror::Error;
use token::TokenTree;

pub mod ast;
pub mod config;
pub mod stream;
pub mod token;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Unexpected end of file")]
    EarlyEof,
    #[error("Unexpected {0:?} while parsing {1}")]
    UnexpectedToken(String, &'static str),
}

#[derive(Debug, Error)]
pub enum SyntaxError {
    #[error("Unexpected token {0:?} while generating AST")]
    UnexpectedToken(TokenTree),
    #[error("Unexpected end of file")]
    EarlyEof,
}

pub type ParseResult<T> = Result<T, ParseError>;

pub(crate) trait ExpectChar {
    fn expect_char(self) -> ParseResult<char>;
}

impl ExpectChar for Option<char> {
    fn expect_char(self) -> ParseResult<char> {
        self.ok_or(ParseError::EarlyEof)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::parse::{self, AstNode, Type},
        token::tokenise,
    };

    #[test]
    fn empty_static_function() -> Result<(), eyre::Report> {
        let input = "static function test() {}";
        let mut ast = vec![];
        let mut result = parse::parse(tokenise((&mut input.chars()).into())?, &mut ast)?;
        ast.append(&mut result);

        let expected = vec![AstNode::Function {
            function_name: "test".to_string(),
            arguments: vec![],
            return_type: Type::Void,
            contents: vec![],
            is_static: true,
        }];

        assert_eq!(ast, expected);

        Ok(())
    }
}
