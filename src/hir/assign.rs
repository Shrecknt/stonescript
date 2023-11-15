use super::Expression;
use crate::{
    ast_item,
    token::{Assign, Ident, Semicolon},
    Span, Spanned,
};

ast_item!(
    pub struct Assignment {
        variable_name: Ident,
        assign: Assign,
        value: Expression,
        semicolon: Semicolon,
    }
);

impl Spanned for Assignment {
    fn span(&self) -> Span {
        Span::from_start_end(self.variable_name.span(), self.semicolon.span())
    }
}
