use super::{
    Statement,
    r#type::Type,
    stream::Stream,
    SyntaxError, SyntaxResult,
};
use crate::{
    expect_token,
    token::{
        ident::{IdentType, Keyword},
        punct::PunctToken,
        TokenTree,
    }, ast::r#type::Primitive,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub function_name: String,
    pub arguments: Vec<Statement>,
    pub return_type: Type,
    pub contents: Vec<Statement>,
    pub is_static: bool,
}

impl Function {
    pub(super) fn next(
        token_tree: &mut Stream,
        project_scope: &mut Vec<Statement>,
        is_static: bool,
    ) -> SyntaxResult<Function> {
        let token = token_tree.expect_next()?;
        let ident = expect_token!(token: TokenTree::Ident[a] = token);
        let token = TokenTree::Ident(ident.clone());
        let keyword = expect_token!(token: IdentType::Keyword[a] = ident.token);
        expect_token!(token: Keyword::Function = keyword);

        let token = token_tree.expect_next()?;
        let ident = expect_token!(token: TokenTree::Ident[a] = token.clone());
        let function_name =
            expect_token!(token: IdentType::VariableName[a] = ident.token).to_string();

        let token = token_tree.expect_next()?;
        #[allow(unused_variables)]
        let group = expect_token!(token: TokenTree::Group[a] = token);
        // TODO: generate vec of arguments (Statement::Declaration)
        let arguments = vec![];

        let mut token = token_tree.expect_next()?;
        let mut return_type = Type::Primitive(Primitive::Void);
        if let TokenTree::Punct(punct) = token {
            expect_token!(token: PunctToken::Colon = punct.token);
            return_type = Type::from({
                let token = token_tree.expect_next()?;
                let ident = expect_token!(token: TokenTree::Ident[a] = token.clone());
                expect_token!(token: IdentType::VariableName[a] = ident.token).as_str()
            });
            token = token_tree.expect_next()?;
        }

        let contents = Stream::new(expect_token!(token: TokenTree::Group[a] = token).tokens)
            .parse(project_scope)?;

        Ok(Function {
            function_name: function_name.clone(),
            arguments: arguments.clone(),
            return_type,
            contents,
            is_static,
        })
    }
}
