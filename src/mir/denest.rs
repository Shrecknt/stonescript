use super::rebuilt_statement::RebuiltStatement;
use crate::hir::Statement;

pub fn denest_statement(
    root: &mut Vec<RebuiltStatement>,
    statement: Statement,
) -> RebuiltStatement {
    match statement {
        Statement::Block(block) => {
            let mut block_contents = vec![];
            for item in block.contents() {
                block_contents.push(denest_statement(root, item.clone()));
            }
            let block_name = random_name(16);
            root.push(RebuiltStatement::Function((
                block_name.clone(),
                block_contents,
            )));
            RebuiltStatement::Call(block_name)
        }
        Statement::Function(function) => todo!(),
        Statement::Declaration(declaration) => RebuiltStatement::Declaration(declaration),
        Statement::Expression(expression) => todo!(),
        Statement::Assignment(assignment) => RebuiltStatement::Assignment(assignment),
        Statement::Return(r#return) => RebuiltStatement::Return(r#return),
        Statement::While(r#while) => todo!(),
        Statement::For(r#for) => todo!(),
        Statement::If(r#if) => todo!(),
        Statement::Unsafe(r#unsafe) => todo!(),
    }
}
