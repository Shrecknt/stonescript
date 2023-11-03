use crate::{token::Ident, Parse, Span, Spanned, SyntaxResult, TokenIter};

macro_rules! define_primitive {
    ($($variant:ident => $value:literal),+) => {
        #[derive(Debug, Clone, PartialEq)]
        pub enum Primitive {
            $($variant { span: Span }),+
        }

        impl From<Ident> for Option<Primitive> {
            fn from(value: Ident) -> Self {
                match value.inner() {
                    $($value => Some(Primitive::$variant { span: value.span() }),)+
                    _ => None,
                }
            }
        }
    }
}

define_primitive!(
    Void => "void",
    Byte => "byte",
    Short => "short",
    Int => "int",
    Long => "long",
    Float => "float",
    Double => "double",
    String => "string"
);

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Primitive(Primitive),
    UserDefined(Ident),
}

impl Parse for Type {
    fn parse(token_iter: &mut TokenIter) -> SyntaxResult<Self> {
        let ident: Ident = token_iter.parse()?;
        if let Some(primitive) = ident.clone().into() {
            Ok(Type::Primitive(primitive))
        } else {
            Ok(Type::UserDefined(ident))
        }
    }
}
