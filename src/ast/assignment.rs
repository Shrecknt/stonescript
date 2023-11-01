use crate::{expect_token, token::{TokenTree, punct::PunctToken, ident::IdentType}};
use super::{expr::Expression, SyntaxResult, stream::Stream, SyntaxError};

#[derive(Debug, Clone, PartialEq)]
pub struct Assignment {
    variable_name: String,
    value: Expression,
}

impl Assignment {
    pub(super) fn next(token_tree: &mut Stream) -> SyntaxResult<Self> {
        let token = token_tree.expect_next()?;
        let ident = expect_token!(token: TokenTree::Ident[a] = token.clone());
        let variable_name = expect_token!(token: IdentType::VariableName[a] = ident.token);

        let token = token_tree.expect_next()?;
        let punct = expect_token!(token: TokenTree::Punct[a] = token.clone());
        expect_token!(token: PunctToken::Assignment = punct.token);

        let value = Expression::next(token_tree)?;

        let token = token_tree.expect_next()?;
        let punct = expect_token!(token: TokenTree::Punct[a] = token);
        expect_token!(token: PunctToken::Semicolon = punct.token);

        Ok(Assignment {
            variable_name,
            value,
        })
    }
}