use crate::token::{
    ident::{IdentType, Keyword},
    punct::PunctToken,
    TokenTree,
};
use crate::SyntaxError;

macro_rules! expect_token {
    ($token:ident: $variant:path = $value:expr) => {
        if let $variant(inner_value) = $value {
            inner_value
        } else {
            return Err(SyntaxError::UnexpectedToken($token));
        }
    };
}

macro_rules! check_token {
    ($token:ident: $variant:path = $value:expr) => {
        if let $variant = $value {
        } else {
            return Err(SyntaxError::UnexpectedToken($token));
        }
    };
}

pub struct TokenStream {
    pub tokens: Vec<TokenTree>,
    pub position: usize,
}

#[allow(unused)]
impl TokenStream {
    pub fn new(tokens: Vec<TokenTree>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    pub fn eof(&self) -> bool {
        self.position >= self.tokens.len()
    }

    pub fn peek(&mut self) -> Option<TokenTree> {
        self.tokens.get(self.position).cloned()
    }

    pub fn next(&mut self) -> Option<TokenTree> {
        let ret = self.peek();
        self.advance();
        ret
    }

    pub fn advance(&mut self) {
        self.position += 1;
    }
}

pub(crate) trait ExpectToken {
    fn expect_token(self) -> Result<TokenTree, SyntaxError>;
}

impl ExpectToken for Option<TokenTree> {
    fn expect_token(self) -> Result<TokenTree, SyntaxError> {
        self.ok_or(SyntaxError::EarlyEof)
    }
}

#[allow(unused)]
#[derive(Debug)]
pub enum AstNode {
    Function {
        function_name: String,
        arguments: Vec<crate::ast::parse::AstNode>,
        return_type: Type,
        contents: Vec<AstNode>,
        is_static: bool,
    },
    Declaration {
        variable_name: String,
        variable_type: Type,
        value: Option<String>,
        is_static: bool,
    },
    Assignment {
        variable_name: String,
        value: String,
    },
    TypedVariable {
        variable_name: String,
        variable_type: Type,
    },
}

#[derive(Debug)]
pub enum Type {
    Void,
    Unknown,
}

impl From<String> for Type {
    fn from(value: String) -> Self {
        match value.as_str() {
            "void" => Self::Void,
            _ => Self::Unknown,
        }
    }
}

pub fn parse(token_tree: Vec<TokenTree>, scope: &[String]) -> Result<Vec<AstNode>, SyntaxError> {
    let mut token_tree = TokenStream::new(token_tree);
    #[allow(unused_mut)]
    let mut scope = scope.to_owned();
    let mut project_scope = vec![];

    let mut token;
    loop {
        token = if let Some(token) = &token_tree.peek() {
            token.clone()
        } else {
            break;
        };

        match token {
            TokenTree::Group(group) => {
                let mut append_ast = parse(group.tokens, &scope)?;
                project_scope.append(&mut append_ast);
                token_tree.advance();
            }
            TokenTree::Ident(ident) => {
                println!("{:?}", ident);
                if let IdentType::Keyword(keyword) = ident.token {
                    if let Keyword::Static = keyword {
                        token_tree.advance();
                        next_static(&mut token_tree, &scope)?;
                    } else {
                        panic!("what 2 {:?}", keyword);
                    }
                } else {
                    panic!("what 1 {:?}", ident.token);
                }
            }
            _ => {
                return Err(SyntaxError::UnexpectedToken(token));
            }
        }
    }

    Ok(project_scope)
}

pub fn next_static(token_tree: &mut TokenStream, scope: &[String]) -> Result<AstNode, SyntaxError> {
    let token = token_tree.peek().expect_token()?;
    let ident = expect_token!(token: TokenTree::Ident = token);

    match &ident.token {
        IdentType::Keyword(_) => next_function(token_tree, scope, true),
        IdentType::VariableName(_) => next_variable(token_tree, true),
    }
}

fn next_function(
    token_tree: &mut TokenStream,
    scope: &[String],
    is_static: bool,
) -> Result<AstNode, SyntaxError> {
    let token = token_tree.next().expect_token()?;
    let ident = expect_token!(token: TokenTree::Ident = token);
    let token = TokenTree::Ident(ident.clone());
    let keyword = expect_token!(token: IdentType::Keyword = ident.token);
    check_token!(token: Keyword::Function = keyword);

    let token = token_tree.next().expect_token()?;
    let ident = expect_token!(token: TokenTree::Ident = token.clone());
    let function_name = expect_token!(token: IdentType::VariableName = ident.token).to_string();

    let token = token_tree.next().expect_token()?;
    let group = expect_token!(token: TokenTree::Group = token);
    let arguments = parse(group.tokens, scope)?;

    let mut token = token_tree.next().expect_token()?;
    let mut return_type = Type::Void;
    if let TokenTree::Punct(punct) = token {
        check_token!(token: PunctToken::Colon = punct.token);
        return_type = expect_token!(token: Ok = Type::try_from({
            let token = token_tree.next().expect_token()?;
            let ident = expect_token!(token: TokenTree::Ident = token.clone());
            expect_token!(token: IdentType::VariableName = ident.token)
        }));
        token = token_tree.next().expect_token()?;
    }

    let contents = parse(expect_token!(token: TokenTree::Group = token).tokens, scope)?;

    Ok(AstNode::Function {
        function_name,
        arguments,
        return_type,
        contents,
        is_static,
    })
}

fn next_variable(token_tree: &mut TokenStream, is_static: bool) -> Result<AstNode, SyntaxError> {
    let token = token_tree.next().expect_token()?;
    let ident = expect_token!(token: TokenTree::Ident = token);
    let token = TokenTree::Ident(ident.clone());
    let variable_name = expect_token!(token: IdentType::VariableName = ident.token);

    let token = token_tree.next().expect_token()?;
    let punct = expect_token!(token: TokenTree::Punct = token);
    check_token!(token: PunctToken::Colon = punct.token);

    let token = token_tree.next().expect_token()?;
    let variable_type = expect_token!(token: Ok = Type::try_from({
        let ident = expect_token!(token: TokenTree::Ident = token.clone());
        expect_token!(token: IdentType::VariableName = ident.token)
    }));

    let mut value = None;
    let token = token_tree.peek().expect_token()?;
    if let TokenTree::Punct(punct) = token {
        if let PunctToken::Semicolon = punct.token {
        } else {
            check_token!(token: PunctToken::Assignment = punct.token);
            let token = token_tree.next().expect_token()?;
            let ident = expect_token!(token: TokenTree::Ident = token.clone());
            value = Some(expect_token!(token: IdentType::VariableName = ident.token));
        }
    }

    Ok(AstNode::Declaration {
        variable_name,
        variable_type,
        value,
        is_static,
    })
}
