pub use self::{
    assign::Assignment,
    block::Block,
    decl::Declaration,
    expr::Expression,
    func::{FunctionArg, FunctionDecl},
    punctuated::Punctuated,
    stmt::Statement,
    ty::{Type, Primitive},
};
use crate::{
    token::{Brace, Bracket, Delimiter, Group, Parenthesis, ToTokenTree},
    Parse, Spanned, SyntaxError, SyntaxResult, TokenIter, TokenTree, TokenStream,
};

mod assign;
mod block;
mod decl;
mod expr;
mod func;
mod parse;
pub(super) mod prelude;
mod punctuated;
mod stmt;
mod ty;

macro_rules! define_group_parsers {
    ($($method_name:ident: $delimiter:ident),+) => {
        $(
            pub fn $method_name<T: Parse>(group: Group) -> SyntaxResult<($delimiter, T)> {
                if group.delimiter() == Delimiter::$delimiter {
                    let span = group.span();
                    let inner = group.into_tokens();
                    Ok(($delimiter::new(span), TokenIter::from(&inner).parse()?))
                } else {
                    Err(SyntaxError::UnexpectedToken(TokenTree::Group(group), stringify!($delimiter)))
                }
            }
        )+
    };
}

define_group_parsers!(
    braced: Brace,
    bracketed: Bracket,
    parenthesized: Parenthesis
);

pub trait ToTokens: Spanned {
    fn write_into_stream(self, stream: &mut Vec<TokenTree>);
    fn into_tokens(self) -> TokenStream where Self: Sized {
        let mut tokens = vec![];
        self.write_into_stream(&mut tokens);
        tokens.into()
    }
}

impl<T: ToTokenTree + Spanned> ToTokens for T {
    fn write_into_stream(self, stream: &mut Vec<TokenTree>) {
        stream.push(self.to_token_tree())
    }
}

impl<T: ToTokens> ToTokens for Vec<T> {
    fn write_into_stream(self, stream: &mut Vec<TokenTree>) {
        for item in self {
            item.write_into_stream(stream)
        }
    }
}

macro_rules! reverse_tuple {
    ($tup:expr, $fni:tt $($ni:tt)*) => {
        ($tup.$fni, $($tup.$ni,)*)
    }
}

macro_rules! tuple_totokens_impl {
    () => {};
    ($fn:ident $fni:tt $($n:ident $ni:tt)*) => {
        #[allow(non_camel_case_types)]
        impl<$fn: ToTokens, $($n: ToTokens,)*> ToTokens for ($fn, $($n,)*) {
            fn write_into_stream(self, stream: &mut Vec<TokenTree>) {
                let rev_self = reverse_tuple!(self, $fni $($ni)*);
                rev_self.$fni.write_into_stream(stream);
                $(
                    rev_self.$ni.write_into_stream(stream);
                )*
            }
        }

        tuple_totokens_impl!($($n $ni)*);
    }
}

tuple_totokens_impl!(a 9 b 8 c 7 d 6 e 5 f 4 g 3 h 2 i 1 j 0);