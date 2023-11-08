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
    Parse, Spanned, SyntaxError, SyntaxResult, TokenIter, TokenTree, TokenStream, Span,
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

fn span_of_two(start: Span, end: Span) -> Span {
    Span::new(start.index, end.index + end.width - start.index)
}

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

impl<T: Spanned> Spanned for Vec<T> {
    fn span(&self) -> Span {
        if let [item] = self.as_slice() {
            item.span()
        } else if let [start_item, .., end_item] = self.as_slice() {
            span_of_two(start_item.span(), end_item.span())
        } else {
            panic!("totokens should not return an empty stream")
        }
    }
}

impl<T: ToTokens> ToTokens for Vec<T> {
    fn write_into_stream(self, stream: &mut Vec<TokenTree>) {
        for item in self {
            item.write_into_stream(stream)
        }
    }
}