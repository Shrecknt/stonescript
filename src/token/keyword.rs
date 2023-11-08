use crate::{
    token::{Ident, Token},
    Sealed, Span, Spanned, TokenTree,
};

macro_rules! define_keyword {
    ($($keyword:ident => $value:literal),+) => {
        $(
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            pub struct $keyword {
                span: Span,
            }

            impl Sealed for $keyword {}
            impl Token for $keyword {
                const NAME: &'static str = stringify!($keyword);

                fn parse_token(token_tree: TokenTree) -> Option<Self> {
                    let value = Ident::parse_token(token_tree)?;
                    if value.inner() == $value {
                        Some($keyword { span: value.span() })
                    } else {
                        None
                    }
                }
            }

            impl Spanned for $keyword {
                fn span(&self) -> Span {
                    self.span
                }
            }
        )+
    }
}

define_keyword!(
    Static => "static",
    For => "for",
    While => "while",
    Let => "let",
    Const => "const",
    Function => "function",
    As => "as",
    Null => "null",
    Return => "return",
    Throw => "throw"
);
