use crate::ast::{Assignment, Declaration};

#[derive(Debug, Clone, PartialEq)]
pub enum RebuiltStatement {
    Declaration(Declaration),
    Assignment(Assignment),
    Function((String, Vec<RebuiltStatement>)),
    Call(String),
}
