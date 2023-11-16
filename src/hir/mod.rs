pub use self::{
    assign::Assignment,
    decl::{DeclStart, Declaration},
    expr::Expression,
    func::{FunctionArg, FunctionDecl},
    punctuated::Punctuated,
    r#for::ForLoop,
    r#if::{ElseBlock, IfBlock},
    r#type::{Primitive, Type},
    r#while::WhileLoop,
    stmt::Statement,
};
use crate::{
    token::{Brace, Ident, PathSeparator, ToTokenTree},
    Spanned, TokenStream, TokenTree,
};

mod assign;
mod decl;
mod expr;
mod r#for;
mod func;
mod r#if;
mod parse;
pub(super) mod prelude;
mod punctuated;
mod stmt;
mod r#type;
mod r#while;

pub(crate) mod mir {
    pub use super::{
        expr::{mir_binaryop::MirBinaryOp, mir_unaryop::MirUnaryOp},
        r#type::mir::MirPrimitive,
    };
}

pub type Block = Brace<Vec<Statement>>;
pub type Path = Punctuated<Ident, PathSeparator>;

pub trait ToTokens: Spanned {
    fn write_into_stream(self, stream: &mut Vec<TokenTree>);
    fn into_tokens(self) -> TokenStream
    where
        Self: Sized,
    {
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
