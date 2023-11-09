use crate::{token::{Ident, ToTokenTree}, Parse, Span, Spanned, SyntaxResult, TokenIter, TokenTree};

macro_rules! define_primitive {
    ($($variant:ident => $value:literal),+) => {
        #[derive(Debug, Clone, PartialEq)]
        pub enum Primitive {
            $($variant { span: Span }),+
        }

        impl Primitive {
            fn from_ident(ident: &Ident) -> Option<Self> {
                match ident.inner() {
                    $($value => Some(Primitive::$variant { span: ident.span() }),)+
                    _ => None,
                }
            }
        }

        impl Spanned for Primitive {
            fn span(&self) -> Span {
                match self {
                    $(
                        Self::$variant { span } => *span,
                    )+
                }
            }
        }

        impl ToTokenTree for Primitive {
            fn to_token_tree(self) -> TokenTree {
                match self {
                    $(
                        Self::$variant { span } => Ident::new_unchecked(span, $value).to_token_tree(),
                    )+
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
        if let Some(primitive) = Primitive::from_ident(&ident) {
            Ok(Self::Primitive(primitive))
        } else {
            Ok(Self::UserDefined(ident))
        }
    }
}

impl Spanned for Type {
    fn span(&self) -> Span {
        match self {
            Self::Primitive(primitive) => primitive.span(),
            Self::UserDefined(ident) => ident.span(),
        }
    }
}

impl ToTokenTree for Type {
    fn to_token_tree(self) -> TokenTree {
        match self {
            Self::Primitive(primitive) => primitive.to_token_tree(),
            Self::UserDefined(ident) => ident.to_token_tree(),
        }
    }
}