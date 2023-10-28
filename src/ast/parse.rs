use crate::token::{
    ident::{IdentType, Keyword},
    punct::PunctToken,
    TokenTree,
};
use crate::SyntaxError;

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
    let ident = if let Some(token) = &token_tree.next() {
        if let TokenTree::Ident(ident) = token {
            ident.clone()
        } else {
            return Err(SyntaxError::UnexpectedToken(token.clone()));
        }
    } else {
        return Err(SyntaxError::EarlyEof);
    };

    match &ident.token {
        IdentType::Keyword(keyword) => {
            if let Keyword::Function = keyword {
                let function_name = if let Some(token) = token_tree.next() {
                    if let TokenTree::Ident(ident) = token.clone() {
                        if let IdentType::VariableName(variable_name) = ident.token {
                            variable_name
                        } else {
                            return Err(SyntaxError::UnexpectedToken(token));
                        }
                    } else {
                        return Err(SyntaxError::UnexpectedToken(token));
                    }
                } else {
                    return Err(SyntaxError::EarlyEof);
                }
                .to_string();
                let arguments = if let Some(token) = token_tree.next() {
                    if let TokenTree::Group(group) = token {
                        parse(group.tokens, scope)?
                    } else {
                        return Err(SyntaxError::UnexpectedToken(token));
                    }
                } else {
                    return Err(SyntaxError::EarlyEof);
                };
                let mut token = if let Some(token) = token_tree.next() {
                    token
                } else {
                    return Err(SyntaxError::EarlyEof);
                };
                let mut return_type = Type::Void;
                if let TokenTree::Punct(punct) = token {
                    if let PunctToken::Colon = punct.token {
                        return_type = if let Ok(variable_type) = Type::try_from({
                            if let Some(token) = token_tree.next() {
                                if let TokenTree::Ident(ident) = token.clone() {
                                    if let IdentType::VariableName(variable_name) = ident.token {
                                        variable_name
                                    } else {
                                        return Err(SyntaxError::UnexpectedToken(token));
                                    }
                                } else {
                                    return Err(SyntaxError::UnexpectedToken(token));
                                }
                            } else {
                                return Err(SyntaxError::EarlyEof);
                            }
                        }) {
                            variable_type
                        } else {
                            return Err(SyntaxError::UnexpectedToken(token));
                        };
                    } else {
                        return Err(SyntaxError::UnexpectedToken(token));
                    }
                    token = if let Some(token) = token_tree.next() {
                        token
                    } else {
                        return Err(SyntaxError::EarlyEof);
                    };
                }
                let contents = if let TokenTree::Group(group) = token {
                    parse(group.tokens, scope)?
                } else {
                    return Err(SyntaxError::UnexpectedToken(token));
                };
                return Ok(AstNode::Function {
                    function_name,
                    arguments,
                    return_type,
                    contents,
                    is_static: true,
                });
            } else {
                return Err(SyntaxError::UnexpectedToken(TokenTree::Ident(ident)));
            }
        }
        IdentType::VariableName(_variable_name) => { /* variable things */ }
    }

    todo!()
}
