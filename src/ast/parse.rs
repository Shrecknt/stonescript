use crate::token::{
    ident::{IdentType, Keyword},
    literal::LiteralType,
    punct::PunctToken,
    TokenTree,
};
use crate::SyntaxError;

/// given `expect_token!(token: Enum::Variant[field] = value)`, check if `value`
/// is of type `Enum::Variant`. If it is, return the inner value (`field`) of
/// `value`. If it isn't, return `Err(SyntaxError::UnexpectedToken(token))`. The
/// name `field` doesn't actually matter, it can be whatever you want, just make
/// the number of fields match the number of inner fields in your enum.
///
/// For example, you could assert that the variable `token` is an ident with the
/// expression: `expect_token!(token: TokenTree::Ident[a] = token)`, which then
/// returns the inner field of type `Ident`, and if it isn't an Ident, an error
/// is returned instead.
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

impl From<Vec<TokenTree>> for TokenStream {
    fn from(tokens: Vec<TokenTree>) -> Self {
        Self {
            tokens,
            position: 0,
        }
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
        value: Option<Expression>,
        is_static: bool,
    },
    Assignment {
        variable_name: String,
        value: Expression,
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

#[derive(Debug, Clone)]
pub enum Expression {
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    String(String),
    Ident(String),
    Add(Box<Expression>, Box<Expression>),
    Subtract(Box<Expression>, Box<Expression>),
    Multiply(Box<Expression>, Box<Expression>),
    Divide(Box<Expression>, Box<Expression>),
}

impl TryFrom<TokenTree> for Expression {
    type Error = SyntaxError;

    fn try_from(token: TokenTree) -> Result<Self, Self::Error> {
        match token.clone() {
            TokenTree::Ident(ident) => Ok(Expression::Ident(
                expect_token!(token: IdentType::VariableName[a] = ident.token),
            )),
            TokenTree::Literal(literal) => Ok(match literal.value {
                LiteralType::Byte(val) => Self::Byte(val),
                LiteralType::Short(val) => Self::Short(val),
                LiteralType::Int(val) => Self::Int(val),
                LiteralType::Long(val) => Self::Long(val),
                LiteralType::Float(val) => Self::Float(val),
                LiteralType::Double(val) => Self::Double(val),
                LiteralType::String(val) => Self::String(val),
                _ => return Err(Self::Error::UnexpectedToken(token)),
            }),
            TokenTree::Group(group) => next_expression(&mut group.tokens.into()),
            _ => Err(Self::Error::UnexpectedToken(token)),
        }
    }
}

pub fn parse(
    token_tree: Vec<TokenTree>,
    project_scope: &mut Vec<AstNode>,
) -> Result<Vec<AstNode>, SyntaxError> {
    let mut token_tree = TokenStream::new(token_tree);
    let mut function_scope = vec![];

    // Iterate over `token_tree` until `token_tree.peek()` returns None, meaning
    // there are no more tokens in the file.
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

/// Get the next static variable or function in the token stream
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

/// Get the next function in the token stream
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

/// Get the next variable in the token stream
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
            value = Some(next_expression(token_tree)?);
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

/// Get the next assignment to a variable in the token stream
fn next_assignment(token_tree: &mut TokenStream) -> Result<AstNode, SyntaxError> {
    let token = token_tree.next().expect_token()?;
    let ident = expect_token!(token: TokenTree::Ident[a] = token.clone());
    let variable_name = expect_token!(token: IdentType::VariableName[a] = ident.token);

    let token = token_tree.next().expect_token()?;
    let punct = expect_token!(token: TokenTree::Punct[a] = token.clone());
    expect_token!(token: PunctToken::Assignment = punct.token);

    let value = next_expression(token_tree)?;

    let token = token_tree.next().expect_token()?;
    let punct = expect_token!(token: TokenTree::Punct[a] = token);
    expect_token!(token: PunctToken::Semicolon = punct.token);

    Ok(AstNode::Assignment {
        variable_name,
        value,
    })
}

/// Get the next expression in the token stream
fn next_expression(token_tree: &mut TokenStream) -> Result<Expression, SyntaxError> {
    let token = token_tree.next().expect_token()?;
    let expr_a = Expression::try_from(token)?;

    let operator_token = match token_tree.peek() {
        Some(token) => match expect_token!(token: TokenTree::Punct[a] = token).token {
            PunctToken::Semicolon => return Ok(expr_a),
            _ => {
                token_tree.advance();
                token
            }
        },
        None => return Ok(expr_a),
    };
    let operator = expect_token!(operator_token: TokenTree::Punct[a] = operator_token).token;

    let expr_b = next_expression(token_tree)?;

    Ok(match operator {
        PunctToken::Add => Expression::Add(expr_a.into(), expr_b.into()),
        PunctToken::Subtract => Expression::Subtract(expr_a.into(), expr_b.into()),
        PunctToken::Multiply => Expression::Multiply(expr_a.into(), expr_b.into()),
        PunctToken::Slash => Expression::Divide(expr_a.into(), expr_b.into()),
        _ => return Err(SyntaxError::UnexpectedToken(operator_token)),
    })
}
