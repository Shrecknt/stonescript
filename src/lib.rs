pub use self::{token::prelude::*, ast::prelude::*};
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

// #[cfg(test)]
// mod tests {
//     use crate::ast::{Statement, ty::{Type, Primitive}, func::Function};

//     #[test]
//     fn empty_static_function() -> Result<(), eyre::Report> {
//         let input = "static function test() {}";
//         let mut ast = vec![];
//         let mut result = TokenStream::new(CharStream::new(&mut input.chars()).tokenise(None)?).parse(&mut ast)?;
//         ast.append(&mut result);

//         let expected = vec![Statement::Function(Function {
//             function_name: "test".to_string(),
//             arguments: vec![],
//             return_type: Type::Primitive(Primitive::Void),
//             contents: vec![],
//             is_static: true,
//         })];

//         assert_eq!(ast, expected);

//         Ok(())
//     }
// }
