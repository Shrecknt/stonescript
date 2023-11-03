use super::{Block, Punctuated, Type};
use crate::{
    token::{Colon, Comma, Function, Ident, Parenthesis, Static},
    Parse, SyntaxResult, TokenIter,
};

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionArg {
    pub name: Ident,
    pub colon: Colon,
    pub ty: Type,
}

impl Parse for FunctionArg {
    fn parse(token_iter: &mut TokenIter) -> SyntaxResult<Self> {
        let name = token_iter.parse()?;
        let colon = token_iter.parse()?;
        let ty = token_iter.parse()?;

        Ok(Self { name, colon, ty })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDecl {
    pub staticness: Option<Static>,
    pub function_token: Function,
    pub ident: Ident,
    pub paren: Parenthesis,
    pub args: Punctuated<FunctionArg, Comma>,
    pub colon: Colon,
    pub return_type: Type,
    pub block: Block,
}

impl Parse for FunctionDecl {
    fn parse(token_iter: &mut TokenIter) -> SyntaxResult<Self> {
        let staticness = token_iter.parse()?;
        let function_token = token_iter.parse()?;
        let ident = token_iter.parse()?;
        let (paren, args) = token_iter.parenthesized()?;
        let colon = token_iter.parse()?;
        let return_type = token_iter.parse()?;
        let block = token_iter.parse()?;

        Ok(Self {
            staticness,
            function_token,
            ident,
            paren,
            args,
            colon,
            return_type,
            block,
        })
    }
}

// use super::{
//     Statement,
//     ty::Type,
//     stream::Stream,
//     SyntaxError, SyntaxResult,
// };
// use crate::{
//     expect_token,
//     token::{
//         ident::{IdentType, Keyword},
//         punct::PunctToken,
//         TokenTree,
//     }, ast::ty::Primitive,
// };

// #[derive(Debug, Clone, PartialEq)]
// pub struct Function {
//     pub function_name: String,
//     pub arguments: Vec<Statement>,
//     pub return_type: Type,
//     pub contents: Vec<Statement>,
//     pub is_static: bool,
// }

// impl Function {
//     pub(super) fn next(
//         token_tree: &mut Stream,
//         is_static: bool,
//     ) -> SyntaxResult<Function> {
//         let token = token_tree.expect_next()?;
//         let ident = expect_token!(token: TokenTree::Ident[a] = token);
//         let token = TokenTree::Ident(ident.clone());
//         let keyword = expect_token!(token: IdentType::Keyword[a] = ident.token);
//         expect_token!(token: Keyword::Function = keyword);

//         let token = token_tree.expect_next()?;
//         let ident = expect_token!(token: TokenTree::Ident[a] = token.clone());
//         let function_name =
//             expect_token!(token: IdentType::VariableName[a] = ident.token).to_string();

//         let token = token_tree.expect_next()?;
//         let _group = expect_token!(token: TokenTree::Group[a] = token);
//         // TODO: generate vec of arguments (Statement::Declaration)
//         let arguments = vec![];

//         let mut token = token_tree.expect_next()?;
//         let mut return_type = Type::Primitive(Primitive::Void);
//         if let TokenTree::Punct(punct) = token {
//             expect_token!(token: PunctToken::Colon = punct.token);
//             return_type = Type::from({
//                 let token = token_tree.expect_next()?;
//                 let ident = expect_token!(token: TokenTree::Ident[a] = token.clone());
//                 expect_token!(token: IdentType::VariableName[a] = ident.token).as_str()
//             });
//             token = token_tree.expect_next()?;
//         }

//         let contents = Stream::new(expect_token!(token: TokenTree::Group[a] = token).tokens)
//             .parse()?;

//         Ok(Function {
//             function_name: function_name.clone(),
//             arguments: arguments.clone(),
//             return_type,
//             contents,
//             is_static,
//         })
//     }
// }
