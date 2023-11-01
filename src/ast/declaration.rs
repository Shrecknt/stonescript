use crate::{expect_token, token::{TokenTree, ident::IdentType, punct::PunctToken}};
use super::{r#type::Type, expr::Expression, SyntaxResult, stream::Stream, SyntaxError};

#[derive(Debug, Clone, PartialEq)]
pub struct Declaration {
    variable_name: String,
    variable_type: Type,
    value: Option<Expression>,
    is_static: bool,
}

impl Declaration {
    pub(super) fn next(token_tree: &mut Stream, is_static: bool) -> SyntaxResult<Declaration> {
        let token = token_tree.expect_next()?;
        let ident = expect_token!(token: TokenTree::Ident[a] = token);
        let token = TokenTree::Ident(ident.clone());
        let variable_name = expect_token!(token: IdentType::VariableName[a] = ident.token);
    
        let token = token_tree.expect_next()?;
        let punct = expect_token!(token: TokenTree::Punct[a] = token);
        expect_token!(token: PunctToken::Colon = punct.token);
    
        let token = token_tree.expect_next()?;
        let variable_type = Type::from({
            let ident = expect_token!(token: TokenTree::Ident[a] = token.clone());
            expect_token!(token: IdentType::VariableName[a] = ident.token).as_str()
        });
    
        let mut value = None;
        let token = token_tree.expect_peek()?;
        if let TokenTree::Punct(punct) = token {
            if let PunctToken::Semicolon = punct.token {
            } else {
                let token = token_tree.expect_next()?;
                expect_token!(token: PunctToken::Assignment = punct.token);
                value = Some(Expression::next(token_tree)?);
            }
        }
    
        let token = token_tree.expect_next()?;
        let punct = expect_token!(token: TokenTree::Punct[a] = token);
        expect_token!(token: PunctToken::Semicolon = punct.token);
    
        Ok(Declaration {
            variable_name,
            variable_type,
            value,
            is_static,
        })
    }
}