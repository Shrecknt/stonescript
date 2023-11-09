use super::{parenthesized, Assignment, Block, Declaration, Expression, FunctionDecl, ToTokens};
use crate::{
    token::{
        Assign, Colon, Delimiter, For, Function, Let, Parenthesis, Return, Semicolon, Static, While,
    },
    Parse, Span, Spanned, SyntaxResult, TokenIter, TokenTree,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Block(Block),
    Function(FunctionDecl),
    Declaration(Declaration),
    Expression(Expression, Semicolon),
    Assignment(Assignment),
    Return(Return, Expression, Semicolon),
    While {
        while_token: While,
        paren: Parenthesis,
        condition: Expression,
        block: Block,
    },
    For {
        for_token: For,
        paren: Parenthesis,
        init: Declaration,
        condition: Expression,
        semicolon: Semicolon,
        update: Box<Statement>,
        block: Block,
    },
}

impl Parse for Statement {
    fn parse(token_iter: &mut TokenIter) -> SyntaxResult<Self> {
        match token_iter.expect_peek()? {
            TokenTree::Group(group) => {
                if group.delimiter() == Delimiter::Brace {
                    return Ok(Self::Block(token_iter.parse()?));
                }
            }
            TokenTree::Ident(ident) => {
                if Return::is_ident(ident) {
                    let return_token = token_iter.parse()?;
                    let expr = token_iter.parse()?;
                    let semicolon = token_iter.parse()?;
                    return Ok(Self::Return(return_token, expr, semicolon));
                }

                if While::is_ident(ident) {
                    let while_token = token_iter.parse()?;
                    let (paren, condition) = parenthesized(token_iter.parse()?)?;
                    let block = token_iter.parse()?;

                    return Ok(Self::While {
                        while_token,
                        paren,
                        condition,
                        block,
                    });
                }

                if For::is_ident(ident) {
                    let for_token = token_iter.parse()?;
                    let (paren, (init, condition, semicolon, update)) =
                        parenthesized(token_iter.parse()?)?;
                    let block = token_iter.parse()?;

                    return Ok(Self::For {
                        for_token,
                        paren,
                        init,
                        condition,
                        semicolon,
                        update: Box::new(update),
                        block,
                    });
                }

                if Function::is_ident(ident) {
                    return Ok(Self::Function(token_iter.parse()?));
                }

                if Let::is_ident(ident) {
                    return Ok(Self::Declaration(token_iter.parse()?));
                }

                if Static::is_ident(ident) {
                    if let TokenTree::Ident(next_ident) = token_iter.expect_peek_ahead(1)? {
                        if Function::is_ident(next_ident) {
                            return Ok(Self::Function(token_iter.parse()?));
                        }
                    }

                    return Ok(Self::Declaration(token_iter.parse()?));
                }

                if let TokenTree::Punct(next_punct) = token_iter.expect_peek_ahead(1)? {
                    if Colon::is_punct(next_punct) {
                        return Ok(Self::Declaration(token_iter.parse()?));
                    }

                    if Assign::is_punct(next_punct) {
                        return Ok(Self::Assignment(token_iter.parse()?));
                    }
                }
            }
            _ => (),
        }

        let expr = token_iter.parse()?;
        let semicolon = token_iter.parse()?;
        Ok(Self::Expression(expr, semicolon))
    }
}

impl Spanned for Statement {
    fn span(&self) -> Span {
        match self {
            Self::Block(block) => block.span(),
            Self::Function(func) => func.span(),
            Self::Declaration(decl) => decl.span(),
            Self::Assignment(assign) => assign.span(),
            Self::Expression(expr, semicolon) => {
                Span::from_start_end(expr.span(), semicolon.span())
            }
            Self::Return(return_token, _expr, semicolon) => {
                Span::from_start_end(return_token.span(), semicolon.span())
            }
            Self::While {
                while_token,
                paren: _,
                condition: _,
                block,
            } => Span::from_start_end(while_token.span(), block.span()),
            Self::For {
                for_token,
                paren: _,
                init: _,
                condition: _,
                semicolon: _,
                update: _,
                block,
            } => Span::from_start_end(for_token.span(), block.span()),
        }
    }
}

impl ToTokens for Statement {
    fn write_into_stream(self, stream: &mut Vec<TokenTree>) {
        match self {
            Self::Block(block) => block.write_into_stream(stream),
            Self::Function(func) => func.write_into_stream(stream),
            Self::Declaration(decl) => decl.write_into_stream(stream),
            Self::Assignment(assign) => assign.write_into_stream(stream),
            Self::Expression(expr, semicolon) => {
                expr.write_into_stream(stream);
                semicolon.write_into_stream(stream);
            }
            Self::Return(return_token, expr, semicolon) => {
                return_token.write_into_stream(stream);
                expr.write_into_stream(stream);
                semicolon.write_into_stream(stream);
            }
            Self::While {
                while_token,
                paren,
                condition,
                block,
            } => {
                while_token.write_into_stream(stream);
                paren.into_group(condition).write_into_stream(stream);
                block.write_into_stream(stream);
            }
            Self::For {
                for_token,
                paren,
                init,
                condition,
                semicolon,
                update,
                block,
            } => {
                for_token.write_into_stream(stream);
                paren
                    .into_group((init, condition, semicolon, *update))
                    .write_into_stream(stream);
                block.write_into_stream(stream);
            }
        }
    }
}
