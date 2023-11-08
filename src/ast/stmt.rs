use super::{span_of_two, Assignment, Block, Declaration, Expression, FunctionDecl, ToTokens};
use crate::{
    token::{Assign, Colon, Delimiter, Function, Semicolon, Static},
    Parse, Span, Spanned, SyntaxResult, TokenIter, TokenTree,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Block(Block),
    Function(FunctionDecl),
    Declaration(Declaration),
    Expression(Expression, Semicolon),
    Assignment(Assignment),
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
                if Function::is_ident(ident) {
                    return Ok(Self::Function(token_iter.parse()?));
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
            Self::Expression(expr, semicolon) => span_of_two(expr.span(), semicolon.span()),
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
        }
    }
}
