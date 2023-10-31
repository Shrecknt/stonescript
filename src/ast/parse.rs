use crate::token::{
    ident::{IdentType, Keyword},
    literal::LiteralType,
    punct::PunctToken,
    TokenTree,
};
use crate::SyntaxError;

macro_rules! expect_token {
    ($token:ident: $variant:path $( [$($varp:ident),+] )? $( {$($varb:ident),+} )? = $value:expr) => {
        if let $variant $( ( $($varp),+ ) )? $( { $($varb),+ } )? = $value {
            ($( $($varp),+ )? $( $($varb),+ )?)
        } else {
            return Err(SyntaxError::UnexpectedToken($token));
        }
    }
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
#[derive(Debug, Clone)]
pub enum AstNode {
    Block {
        contents: Vec<AstNode>,
    },
    Function {
        function_name: String,
        arguments: Vec<AstNode>,
        return_type: Type,
        contents: Vec<AstNode>,
        is_static: bool,
    },
    Call {
        function_name: String,
        arguments: Vec<AstNode>,
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
    Command {
        contents: String,
    },
}

#[derive(Debug, Clone)]
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

pub fn parse(
    token_tree: Vec<TokenTree>,
    project_scope: &mut Vec<AstNode>,
) -> Result<Vec<AstNode>, SyntaxError> {
    let mut token_tree = TokenStream::new(token_tree);
    let mut function_scope = vec![];

    let mut token;
    loop {
        token = if let Some(token) = &token_tree.peek() {
            token.clone()
        } else {
            break;
        };

        match token.clone() {
            TokenTree::Group(group) => {
                let append_ast = parse(group.tokens, project_scope)?;
                project_scope.push(AstNode::Block {
                    contents: append_ast,
                });
                token_tree.advance();
            }
            TokenTree::Ident(ident) => match ident.token {
                IdentType::Keyword(Keyword::Static) => {
                    token_tree.advance();
                    next_static(&mut token_tree, project_scope, &mut function_scope)?;
                }
                IdentType::VariableName(_) => {
                    let assignment = next_assignment(&mut token_tree)?;
                    function_scope.push(assignment);
                }
                _ => return Err(SyntaxError::UnexpectedToken(token)),
            },
            TokenTree::Literal(literal) => {
                if let LiteralType::Command(contents) = literal.value {
                    function_scope.push(AstNode::Command { contents });
                    token_tree.advance();
                } else {
                    return Err(SyntaxError::UnexpectedToken(token));
                }
            }
            _ => {
                return Err(SyntaxError::UnexpectedToken(token));
            }
        }
    }

    Ok(function_scope)
}

pub fn next_static(
    token_tree: &mut TokenStream,
    project_scope: &mut Vec<AstNode>,
    function_scope: &mut Vec<AstNode>,
) -> Result<AstNode, SyntaxError> {
    let token = token_tree.peek().expect_token()?;
    let ident = expect_token!(token: TokenTree::Ident[a] = token);

    match &ident.token {
        IdentType::Keyword(_) => next_function(token_tree, project_scope, true),
        IdentType::VariableName(_) => {
            let variable_declaration = next_variable(token_tree, true)?;
            function_scope.push(variable_declaration.clone());
            Ok(variable_declaration)
        }
    }
}

fn next_function(
    token_tree: &mut TokenStream,
    project_scope: &mut Vec<AstNode>,
    is_static: bool,
) -> Result<AstNode, SyntaxError> {
    let token = token_tree.next().expect_token()?;
    let ident = expect_token!(token: TokenTree::Ident[a] = token);
    let token = TokenTree::Ident(ident.clone());
    let keyword = expect_token!(token: IdentType::Keyword[a] = ident.token);
    expect_token!(token: Keyword::Function = keyword);

    let token = token_tree.next().expect_token()?;
    let ident = expect_token!(token: TokenTree::Ident[a] = token.clone());
    let function_name = expect_token!(token: IdentType::VariableName[a] = ident.token).to_string();

    let token = token_tree.next().expect_token()?;
    #[allow(unused_variables)]
    let group = expect_token!(token: TokenTree::Group[a] = token);
    // TODO: generate vec of arguments (AstNode::Declaration)
    let arguments = vec![];

    let mut token = token_tree.next().expect_token()?;
    let mut return_type = Type::Void;
    if let TokenTree::Punct(punct) = token {
        expect_token!(token: PunctToken::Colon = punct.token);
        return_type = expect_token!(token: Ok[a] = Type::try_from({
            let token = token_tree.next().expect_token()?;
            let ident = expect_token!(token: TokenTree::Ident[a] = token.clone());
            expect_token!(token: IdentType::VariableName[a] = ident.token)
        }));
        token = token_tree.next().expect_token()?;
    }

    let contents = parse(
        expect_token!(token: TokenTree::Group[a] = token).tokens,
        project_scope,
    )?;

    project_scope.push(AstNode::Function {
        function_name: function_name.clone(),
        arguments: arguments.clone(),
        return_type,
        contents,
        is_static,
    });

    Ok(AstNode::Call {
        function_name,
        arguments,
    })
}

fn next_variable(token_tree: &mut TokenStream, is_static: bool) -> Result<AstNode, SyntaxError> {
    let token = token_tree.next().expect_token()?;
    let ident = expect_token!(token: TokenTree::Ident[a] = token);
    let token = TokenTree::Ident(ident.clone());
    let variable_name = expect_token!(token: IdentType::VariableName[a] = ident.token);

    let token = token_tree.next().expect_token()?;
    let punct = expect_token!(token: TokenTree::Punct[a] = token);
    expect_token!(token: PunctToken::Colon = punct.token);

    let token = token_tree.next().expect_token()?;
    let variable_type = expect_token!(token: Ok[a] = Type::try_from({
        let ident = expect_token!(token: TokenTree::Ident[a] = token.clone());
        expect_token!(token: IdentType::VariableName[a] = ident.token)
    }));

    let mut value = None;
    let token = token_tree.peek().expect_token()?;
    if let TokenTree::Punct(punct) = token {
        if let PunctToken::Semicolon = punct.token {
        } else {
            let token = token_tree.next().expect_token()?;
            expect_token!(token: PunctToken::Assignment = punct.token);
            let token = token_tree.next().expect_token()?;
            let ident = expect_token!(token: TokenTree::Ident[a] = token.clone());
            value = Some(expect_token!(token: IdentType::VariableName[a] = ident.token));
        }
    }

    let token = token_tree.next().expect_token()?;
    let punct = expect_token!(token: TokenTree::Punct[a] = token);
    expect_token!(token: PunctToken::Semicolon = punct.token);

    Ok(AstNode::Declaration {
        variable_name,
        variable_type,
        value,
        is_static,
    })
}

fn next_assignment(token_tree: &mut TokenStream) -> Result<AstNode, SyntaxError> {
    let token = token_tree.next().expect_token()?;
    let ident = expect_token!(token: TokenTree::Ident[a] = token.clone());
    let variable_name = expect_token!(token: IdentType::VariableName[a] = ident.token);

    let token = token_tree.next().expect_token()?;
    let punct = expect_token!(token: TokenTree::Punct[a] = token.clone());
    expect_token!(token: PunctToken::Assignment = punct.token);

    let token = token_tree.next().expect_token()?;
    let ident = expect_token!(token: TokenTree::Ident[a] = token.clone());
    let value = expect_token!(token: IdentType::VariableName[a] = ident.token);

    token_tree.advance();

    Ok(AstNode::Assignment {
        variable_name,
        value,
    })
}
