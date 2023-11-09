use super::Expression;
use crate::{
    token::{Assign, Ident, Semicolon},
    Spanned, Span, ast_item, 
};

ast_item!(
    pub struct Assignment {
        variable_name: Ident,
        assign: Assign,
        value: Expression,
        semicolon: Semicolon
    }
);

impl Spanned for Assignment {
    fn span(&self) -> Span {
        Span::from_start_end(self.variable_name.span(), self.semicolon.span())
    }
}