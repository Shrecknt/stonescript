pub use self::{
    assign::Assignment,
    block::Block,
    decl::Declaration,
    expr::Expression,
    func::{FunctionArg, FunctionDecl},
    punctuated::Punctuated,
    stmt::Statement,
    ty::Type,
};
use crate::{
    token::{Brace, Bracket, Delimiter, Group, Parenthesis},
    Parse, Spanned, SyntaxError, SyntaxResult, TokenIter, TokenTree,
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
