use crate::token::{
    ident::{Ident, IdentType, Keyword},
    punct::{self, Punct, PunctToken},
    TokenTree,
};
use crate::SyntaxError;

pub struct TokenStream {
    pub tokens: Vec<TokenTree>,
    pub position: usize,
}

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
        variable_type: String,
        value: Option<String>,
        is_static: bool,
    },
    Assignment {
        variable_name: String,
        value: String,
    },
    TypedVariable {
        variable_name: String,
        variable_type: String,
    },
}

#[derive(Debug)]
pub enum Type {
    Void,
}

impl From<String> for Type {
    fn from(value: String) -> Self {
        todo!()
    }
}

pub fn parse(token_tree: Vec<TokenTree>, scope: &Vec<String>) -> Result<Vec<AstNode>, SyntaxError> {
    let mut token_tree = TokenStream::new(token_tree);
    let mut scope = scope.clone();
    let mut project_scope = vec![];

    let mut token;
    loop {
        token = match &token_tree.peek() {
            Some(token) => token.clone(),
            None => {
                break;
            }
        };

        match token {
            TokenTree::Group(group) => {
                let mut append_ast = parse(group.tokens, &scope)?;
                project_scope.append(&mut append_ast);
                token_tree.advance();
            }
            TokenTree::Ident(ident) => {
                println!("{:?}", ident);
                match ident.token {
                    IdentType::Keyword(keyword) => match keyword {
                        Keyword::Static => {
                            token_tree.advance();
                            next_static(&mut token_tree, &mut scope.clone())?;
                        }
                        _ => {
                            panic!("what 2 {:?}", keyword);
                        }
                    },
                    _ => {
                        panic!("what 1 {:?}", ident.token);
                    }
                }
            }
            _ => {
                return Err(SyntaxError::UnexpectedToken(token));
            }
        }
    }

    Ok(project_scope)
}

pub fn next_static(
    token_tree: &mut TokenStream,
    scope: &mut Vec<String>,
) -> Result<AstNode, SyntaxError> {
    let ident = match &token_tree.next() {
        Some(token) => match token {
            TokenTree::Ident(ident) => ident.clone(),
            _ => return Err(SyntaxError::UnexpectedToken(token.clone())),
        },
        None => return Err(SyntaxError::EarlyEof),
    };

    match &ident.token {
        IdentType::Keyword(keyword) => match keyword {
            Keyword::Function => {
                let function_name = match token_tree.next() {
                    Some(token) => match token.clone() {
                        TokenTree::Ident(ident) => match ident.token {
                            IdentType::VariableName(variable_name) => variable_name,
                            _ => return Err(SyntaxError::UnexpectedToken(token)),
                        },
                        _ => return Err(SyntaxError::UnexpectedToken(token)),
                    },
                    None => return Err(SyntaxError::EarlyEof),
                }
                .to_string();
                let arguments = match token_tree.next() {
                    Some(token) => match token {
                        TokenTree::Group(group) => parse(group.tokens, &scope.clone())?,
                        _ => return Err(SyntaxError::UnexpectedToken(token)),
                    },
                    None => return Err(SyntaxError::EarlyEof),
                };
                let mut token = match token_tree.next() {
                    Some(token) => token,
                    None => return Err(SyntaxError::EarlyEof),
                };
                let mut return_type = Type::Void;
                if let TokenTree::Punct(punct) = token {
                    if let PunctToken::Colon = punct.token {
                        return_type = match Type::try_from(match token_tree.next() {
                            Some(token) => match token.clone() {
                                TokenTree::Ident(ident) => match ident.token {
                                    IdentType::VariableName(variable_name) => variable_name,
                                    _ => return Err(SyntaxError::UnexpectedToken(token)),
                                },
                                _ => return Err(SyntaxError::UnexpectedToken(token)),
                            },
                            None => return Err(SyntaxError::EarlyEof),
                        }) {
                            Ok(variable_type) => variable_type,
                            Err(_) => return Err(SyntaxError::UnexpectedToken(token)),
                        };
                    } else {
                        return Err(SyntaxError::UnexpectedToken(token));
                    }
                    token = match token_tree.next() {
                        Some(token) => token,
                        None => return Err(SyntaxError::EarlyEof),
                    };
                }
                let contents = match token {
                    TokenTree::Group(group) => parse(group.tokens, &scope)?,
                    _ => return Err(SyntaxError::UnexpectedToken(token)),
                };
                return Ok(AstNode::Function {
                    function_name,
                    arguments,
                    return_type,
                    contents,
                    is_static: true,
                });
            }
            _ => return Err(SyntaxError::UnexpectedToken(TokenTree::Ident(ident))),
        },
        IdentType::VariableName(variable_name) => { /* variable things */ }
    }

    todo!()
}
