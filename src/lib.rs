pub use self::{ast::prelude::*, token::prelude::*};
pub(crate) use private::Sealed;

mod private {
    pub trait Sealed {}
}

pub mod ast;
pub mod config;
pub mod token;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub index: usize,
    pub width: usize,
}

impl Span {
    pub fn new(index: usize, width: usize) -> Self {
        Span { index, width }
    }
}

pub trait Spanned {
    fn span(&self) -> Span;
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::{FunctionDecl, Primitive, Statement, Type},
        parse_str, TokenIter,
    };

    #[test]
    fn empty_static_function() -> eyre::Result<()> {
        let input = "static function test(): void {}";
        let tokens = parse_str(input)?;
        let ast: Vec<Statement> = TokenIter::from(&tokens).parse()?;

        if let [Statement::Function(FunctionDecl {
            staticness,
            function_token: _,
            ident,
            paren: _,
            args,
            colon: _,
            return_type,
            block,
        })] = ast.as_slice()
        {
            assert!(staticness.is_some());
            assert_eq!(ident.inner(), "test");
            assert!(args.is_empty());
            if let Type::Primitive(Primitive::Void { span: _ }) = return_type {
            } else {
                panic!("expected type `void` got `{:?}`", return_type);
            }
            assert!(block.contents.is_empty());

            return Ok(());
        }

        panic!("incorrect ast: {:?}", ast);
    }
}
