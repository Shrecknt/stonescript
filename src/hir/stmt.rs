use super::{
    Assignment, Block, Declaration, Expression, ForLoop, FunctionDecl, IfBlock, Path, WhileLoop,
};
use crate::{
    ast_item,
    token::{
        Assign, Brace, Colon, Delimiter, For, Function, If, Import, Let, MacroPrefix,
        PathSeparator, Return, Semicolon, Static, Unsafe, While,
    },
    Parse, Span, SyntaxResult, TokenIter, TokenStream, TokenTree,
};

ast_item!(
    pub enum Statement {
        Block(Block),
        Function(FunctionDecl),
        Declaration(Declaration),
        Expression((Expression, Semicolon)),
        Assignment(Assignment),
        Return((Return, Expression, Semicolon)),
        While(WhileLoop),
        If(IfBlock),
        For(Box<ForLoop>),
        Unsafe((Unsafe, Block)),
        Macro((MacroPrefix, Path, Brace<TokenStream>)),
        Import((Import, Path, Semicolon)),
    }
);

fn is_stmt_macro(
    token_iter: &mut TokenIter,
    index: usize,
    was_seperator: bool,
) -> SyntaxResult<bool> {
    let token = token_iter.expect_peek_ahead(index)?;
    if was_seperator {
        if let TokenTree::Ident(_ident) = token {
            is_stmt_macro(token_iter, index + 1, false)
        } else {
            Ok(false)
        }
    } else {
        match token {
            TokenTree::Punct(punct) => {
                if PathSeparator::is_punct(punct) {
                    is_stmt_macro(token_iter, index + 1, true)
                } else {
                    Ok(false)
                }
            }
            TokenTree::Group(group) => {
                if group.delimiter() == Delimiter::Brace {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            _ => Ok(false),
        }
    }
}

impl Parse for Statement {
    fn parse(token_iter: &mut TokenIter) -> SyntaxResult<Self> {
        match token_iter.expect_peek()? {
            TokenTree::Punct(punct) => {
                if MacroPrefix::is_punct(punct) && is_stmt_macro(token_iter, 1, true)? {
                    return Ok(Self::Macro(token_iter.parse()?));
                }
            }
            TokenTree::Group(group) => {
                if group.delimiter() == Delimiter::Brace {
                    return Ok(Self::Block(token_iter.parse()?));
                }
            }
            TokenTree::Ident(ident) => {
                if Return::is_ident(ident) {
                    return Ok(Self::Return(token_iter.parse()?));
                }

                if Unsafe::is_ident(ident) {
                    return Ok(Self::Unsafe(token_iter.parse()?));
                }

                if While::is_ident(ident) {
                    return Ok(Self::While(token_iter.parse()?));
                }

                if If::is_ident(ident) {
                    return Ok(Self::If(token_iter.parse()?));
                }

                if For::is_ident(ident) {
                    return Ok(Self::For(Box::new(token_iter.parse()?)));
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
        Ok(Self::Expression(expr))
    }
}
