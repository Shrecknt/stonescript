use std::collections::HashMap;

use rand::{distributions::Alphanumeric, Rng};

use crate::{
    ast::{Expression, Statement},
    token::{Comma, Punct},
};

pub fn random_name(size: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(size)
        .map(char::from)
        .collect()
}

pub fn mangle_variables(ast: Vec<Statement>) -> Vec<Statement> {
    let mut scope = HashMap::<String, String>::new();
    for item in ast.clone() {
        if let Statement::Function(function) = item {
            scope.insert(function.ident.inner().to_string(), random_name(16));
        }
    }
    println!("Scope: {:?}", scope);
    let ast = mangle_block_variables(ast, scope);
    ast
}

fn mangle_block_variables(
    mut ast: Vec<Statement>,
    mut scope: HashMap<String, String>,
) -> Vec<Statement> {
    for statement in &mut ast {
        match statement {
            Statement::Block(block) => {
                block.contents = mangle_block_variables(block.contents.clone(), scope.clone());
            }
            Statement::Function(function) => {
                let function_scope = scope.clone();
                // TODO: Insert function arguments into function_scope
                function.block.contents =
                    mangle_block_variables(function.block.contents.clone(), function_scope);
            }
            Statement::Declaration(declaration) => {
                let mangled_name = random_name(16);
                scope.insert(declaration.ident.inner().to_string(), mangled_name.clone());
                declaration.ident.value.0 = mangled_name;
            }
            Statement::Expression(expression) => {
                expression.0 = mangle_expression_variables(expression.0.clone(), scope.clone());
            }
            Statement::Assignment(assignment) => {
                let mangled_name = random_name(16);
                scope.insert(
                    assignment.variable_name.inner().to_string(),
                    mangled_name.clone(),
                );
                assignment.variable_name.value.0 = mangled_name;
            }
            Statement::Return(r#return) => {
                r#return.1 = mangle_expression_variables(r#return.1.clone(), scope.clone());
            }
            Statement::While(r#while) => {
                r#while.block.contents =
                    mangle_block_variables(r#while.block.contents.clone(), scope.clone());
            }
            Statement::If(r#if) => {
                r#if.block.contents =
                    mangle_block_variables(r#if.block.contents.clone(), scope.clone());
            }
            Statement::For(r#for) => {
                let function_scope = scope.clone();
                // TODO: Insert iterator variable into function_scope
                r#for.block.contents =
                    mangle_block_variables(r#for.block.contents.clone(), function_scope);
            }
        }
    }
    ast
}

fn mangle_expression_variables(
    expression: Expression,
    scope: HashMap<String, String>,
) -> Expression {
    match expression {
        Expression::Variable(mut variable) => {
            let mangled_name = scope.get(&variable.value.0.to_string()).expect("").clone();
            variable.value.0 = mangled_name;
            Expression::Variable(variable)
        }
        Expression::Property(expression, _, _) => {
            mangle_expression_variables(*expression, scope.clone())
        }
        Expression::Call(expression, mut arguments) => {
            let expression = mangle_expression_variables(*expression, scope.clone());
            arguments.contents.inner = arguments
                .contents
                .inner
                .iter()
                .map(|v| v.clone())
                .collect::<Vec<(Expression, Comma)>>();
            Expression::Call(expression.into(), arguments)
        }
        Expression::Parenthesized(_) => todo!(),
        Expression::Index(_, _) => todo!(),
        Expression::UnaryOp(_, _) => todo!(),
        Expression::BinaryOp(_, _, _) => todo!(),
        _ => expression,
    }
}
