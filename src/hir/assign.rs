use super::{Expression, Path};
use crate::{
    ast_item,
    token::{Assign, Semicolon},
    Span, Spanned,
};

ast_item!(
    pub struct Assignment {
        variable: Path,
        assign: Assign,
        value: Expression,
        semicolon: Semicolon,
    }
);

impl Spanned for Assignment {
    fn span(&self) -> Span {
        Span::from_start_end(self.variable.span(), self.semicolon.span())
    }
}
