use crate::{
    ast_item,
    token::{Ident, ToTokenTree},
    Parse, Span, Spanned, SyntaxResult, TokenIter, TokenTree,
};

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

        pub(crate) mod mir {
            use crate::mir::ToMir;
            use super::Primitive;

            #[derive(Debug, Clone, PartialEq)]
            pub enum MirPrimitive {
                $($variant),+
            }

            impl ToMir for Primitive {
                type Output = MirPrimitive;

                fn into_mir(self) -> Self::Output {
                    match self {
                        $(
                            Self::$variant { span: _ } => MirPrimitive::$variant,
                        )+
                    }
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

ast_item!(
    pub enum Type {
        Primitive(Primitive),
        UserDefined(Ident),
    }
);

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
