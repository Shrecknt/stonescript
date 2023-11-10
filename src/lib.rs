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
        Self { index, width }
    }

    pub fn from_start_end(start: Span, end: Span) -> Self {
        Self::new(start.index, end.index + end.width - start.index)
    }
}

pub trait Spanned {
    fn span(&self) -> Span;
}

impl<T: Spanned> Spanned for Vec<T> {
    fn span(&self) -> Span {
        if let [item] = self.as_slice() {
            item.span()
        } else if let [start_item, .., end_item] = self.as_slice() {
            Span::from_start_end(start_item.span(), end_item.span())
        } else {
            panic!("totokens should not return an empty stream")
        }
    }
}

impl<T0: Spanned> Spanned for (T0,) {
    fn span(&self) -> Span {
        self.0.span()
    }
}

macro_rules! tuple_spanned_impl {
    ($eni:tt, $sn:ident $($n:ident)*) => {
        #[allow(non_camel_case_types)]
        impl<$sn: Spanned, $($n: Spanned,)+> Spanned for ($sn, $($n,)+) {
            fn span(&self) -> Span {
                Span::from_start_end(self.0.span(), self.$eni.span())
            }
        }
    }
}

tuple_spanned_impl!(1, a b);
tuple_spanned_impl!(2, a b c);
tuple_spanned_impl!(3, a b c d);
tuple_spanned_impl!(4, a b c d e);
tuple_spanned_impl!(5, a b c d e f);
tuple_spanned_impl!(6, a b c d e f g);
tuple_spanned_impl!(7, a b c d e f g h);
tuple_spanned_impl!(8, a b c d e f g h i);
tuple_spanned_impl!(9, a b c d e f g h i j);

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
            args,
            colon: _,
            return_type,
            block,
        })] = ast.as_slice()
        {
            assert!(staticness.is_some());
            assert_eq!(ident.inner(), "test");
            assert!(args.contents().is_empty());
            if let Type::Primitive(Primitive::Void { span: _ }) = return_type {
            } else {
                panic!("expected type `void` got `{:?}`", return_type);
            }
            assert!(block.contents().is_empty());

            return Ok(());
        }

        panic!("incorrect ast: {:?}", ast);
    }
}
