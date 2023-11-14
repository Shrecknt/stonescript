use crate::{
    ast::{Assignment, Declaration, Expression},
    token::{Return, Semicolon},
};

#[derive(Debug, Clone, PartialEq)]
pub enum RebuiltStatement {
    Declaration(Declaration),
    Assignment(Assignment),
    Function((String, Vec<RebuiltStatement>)),
    Call(String),
    Return((Return, Expression, Semicolon)),
}
