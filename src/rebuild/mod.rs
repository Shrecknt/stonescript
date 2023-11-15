use crate::{ast::Statement, config::ProjectConfig};

use self::{
    denest::denest_statement, mangle::mangle_variables, rebuilt_statement::RebuiltStatement,
};

mod denest;
mod mangle;

pub mod rebuilt_statement;

pub fn rebuild_from_ast(
    ast: Vec<Statement>,
    project_config: &ProjectConfig,
) -> Vec<RebuiltStatement> {
    let ast = mangle_variables(ast, project_config);
    let mut rebuilt: Vec<RebuiltStatement> = vec![];
    for statement in ast {
        denest_statement(&mut rebuilt, statement);
    }
    rebuilt
}
