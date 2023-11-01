use super::{
    assignment::Assignment, declaration::Declaration, func::Function, Statement, SyntaxError,
    SyntaxResult,
};
use crate::{
    expect_token,
    token::{
        ident::{Ident, IdentType, Keyword},
        literal::LiteralType,
        TokenTree,
    },
};

pub struct Stream {
    pub tokens: Vec<TokenTree>,
    pub position: usize,
}

#[allow(unused)]
impl Stream {
    pub fn new(tokens: Vec<TokenTree>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    pub fn eof(&self) -> bool {
        self.position >= self.tokens.len()
    }

    pub fn peek(&self) -> Option<TokenTree> {
        self.tokens.get(self.position).cloned()
    }

    pub fn expect_peek(&self) -> SyntaxResult<TokenTree> {
        self.peek().ok_or(SyntaxError::EarlyEof)
    }

    pub fn next(&mut self) -> Option<TokenTree> {
        let ret = self.peek();
        self.advance();
        ret
    }

    pub fn expect_next(&mut self) -> SyntaxResult<TokenTree> {
        self.next().ok_or(SyntaxError::EarlyEof)
    }

    pub fn advance(&mut self) {
        self.position += 1;
    }

    pub fn parse(self, project_scope: &mut Vec<Statement>) -> SyntaxResult<Vec<Statement>> {
        let mut token_tree = Stream::new(self.tokens);
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
                    let append_ast = Stream::new(group.tokens).parse(project_scope)?;
                    project_scope.push(Statement::Block {
                        contents: append_ast,
                    });
                    token_tree.advance();
                }
                TokenTree::Ident(ident) => parse_ident(
                    ident,
                    token,
                    &mut token_tree,
                    project_scope,
                    &mut function_scope,
                )?,
                TokenTree::Literal(literal) => {
                    if let LiteralType::Command(contents) = literal.value {
                        function_scope.push(Statement::Command(contents));
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
}

impl From<Vec<TokenTree>> for Stream {
    fn from(tokens: Vec<TokenTree>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }
}

fn parse_ident(
    ident: Ident,
    token: TokenTree,
    token_tree: &mut Stream,
    project_scope: &mut Vec<Statement>,
    function_scope: &mut Vec<Statement>,
) -> SyntaxResult<()> {
    match ident.token {
        IdentType::Keyword(Keyword::Static) => {
            token_tree.advance();
            let token = token_tree.expect_peek()?;
            let ident = expect_token!(token: TokenTree::Ident[a] = token);

            match &ident.token {
                IdentType::Keyword(_) => {
                    let function_declaration =
                        Statement::Function(Function::next(token_tree, project_scope, true)?);
                    project_scope.push(function_declaration.clone())
                }
                IdentType::VariableName(_) => {
                    let variable_declaration =
                        Statement::Declaration(Declaration::next(token_tree, true)?);
                    function_scope.push(variable_declaration.clone())
                }
            };
        }
        IdentType::VariableName(_) => {
            let assignment = Assignment::next(token_tree)?;
            function_scope.push(Statement::Assignment(assignment));
        }
        _ => return Err(SyntaxError::UnexpectedToken(token)),
    };

    Ok(())
}
