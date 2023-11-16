use super::{
    MangledVar, MirAssignment, MirDeclaration, MirElseBlock, MirExpression, MirFor, MirFunction,
    MirIf, MirStatement, MirWhile,
};
use crate::token::XID;
use rustc_hash::FxHasher;
use std::{collections::HashMap, hash::Hasher};

fn hash_str(hasher: &mut FxHasher, value: &str) {
    hasher.write(value.as_bytes());
    hasher.write_u8(0xFF);
    // 0xFF cannot show up in UTF-8 strings, this marks the end of a string and makes hashes unique.
}

pub struct Scope<'a> {
    parent: Option<&'a Scope<'a>>,
    variables: HashMap<XID, MangledVar>,
    unnamed_counter: usize,
    id: MangledVar,
}

impl<'a> Scope<'a> {
    pub fn new(name: &str) -> Self {
        let mut hasher = FxHasher::default();
        hash_str(&mut hasher, name);
        let id = MangledVar(hasher.finish());

        Self {
            parent: None,
            variables: HashMap::new(),
            unnamed_counter: 0,
            id,
        }
    }

    pub fn new_child(&'a self, id: MangledVar) -> Self {
        Self {
            parent: Some(self),
            variables: HashMap::new(),
            unnamed_counter: 0,
            id,
        }
    }

    pub fn hash_named(&self, value: &str) -> MangledVar {
        let mut hasher = FxHasher::default();

        hasher.write_u64(self.id.0);
        hash_str(&mut hasher, value);

        MangledVar(hasher.finish())
    }

    pub fn hash_unnamed(&mut self) -> MangledVar {
        let mut hasher = FxHasher::default();

        hasher.write_u64(self.id.0);
        hasher.write_usize(self.unnamed_counter);
        self.unnamed_counter += 1;

        MangledVar(hasher.finish())
    }

    fn find_defined_variable_self(&self, variable: &XID) -> Option<MangledVar> {
        self.variables.get(variable).copied()
    }

    fn find_defined_variable(&self, variable: &XID) -> Option<MangledVar> {
        if let Some(parent) = self.parent {
            parent
                .find_defined_variable(variable)
                .or_else(|| self.find_defined_variable_self(variable))
        } else {
            self.find_defined_variable_self(variable)
        }
    }

    pub fn get_variable(&mut self, variable: XID) -> MangledVar {
        self.find_defined_variable(&variable)
            .unwrap_or_else(|| self.hash_named(variable.inner()))
    }

    pub fn new_variable(&mut self, variable: XID) -> MangledVar {
        let id = self.hash_named(variable.inner());
        self.variables.insert(variable, id);
        id
    }

    pub fn mangle_unnamed_child<T: Mangle>(&mut self, value: T) -> T::Output {
        let id = self.hash_unnamed();
        value.mangle(&mut self.new_child(id))
    }
}

pub trait Mangle {
    type Output;
    fn mangle(self, scope: &mut Scope) -> Self::Output;
}

impl<T: Mangle> Mangle for Vec<T> {
    type Output = Vec<T::Output>;
    fn mangle(self, scope: &mut Scope) -> Self::Output {
        self.into_iter().map(|item| item.mangle(scope)).collect()
    }
}

impl Mangle for MirStatement<XID> {
    type Output = MirStatement<MangledVar>;
    fn mangle(self, scope: &mut Scope) -> Self::Output {
        match self {
            MirStatement::Block(block) => MirStatement::Block(scope.mangle_unnamed_child(block)),
            MirStatement::Unsafe(block) => MirStatement::Unsafe(scope.mangle_unnamed_child(block)),
            MirStatement::Expression(expr) => MirStatement::Expression(expr.mangle(scope)),
            MirStatement::Return(value) => MirStatement::Return(value.mangle(scope)),
            MirStatement::Assignment(assign) => MirStatement::Assignment(assign.mangle(scope)),
            MirStatement::Declaration(decl) => MirStatement::Declaration(decl.mangle(scope)),
            MirStatement::Function(func) => MirStatement::Function(func.mangle(scope)),
            MirStatement::For(for_loop) => MirStatement::For(for_loop.mangle(scope)),
            MirStatement::While(while_loop) => MirStatement::While(while_loop.mangle(scope)),
            MirStatement::If(if_block) => MirStatement::If(if_block.mangle(scope)),
        }
    }
}

impl Mangle for MirExpression<XID> {
    type Output = MirExpression<MangledVar>;
    fn mangle(self, scope: &mut Scope) -> Self::Output {
        match self {
            MirExpression::Literal(literal) => MirExpression::Literal(literal),
            MirExpression::Variable(variable) => {
                MirExpression::Variable(scope.get_variable(variable))
            }
            MirExpression::Call(left, args) => {
                MirExpression::Call(Box::new(left.mangle(scope)), args.mangle(scope))
            }
            MirExpression::Index(left, index) => {
                MirExpression::Index(Box::new(left.mangle(scope)), Box::new(index.mangle(scope)))
            }
            MirExpression::Property(left, property) => {
                MirExpression::Property(Box::new(left.mangle(scope)), property)
            }
            MirExpression::UnaryOp(op, expr) => {
                MirExpression::UnaryOp(op, Box::new(expr.mangle(scope)))
            }
            MirExpression::BinaryOp(left, op, right) => MirExpression::BinaryOp(
                Box::new(left.mangle(scope)),
                op,
                Box::new(right.mangle(scope)),
            ),
        }
    }
}

impl Mangle for MirAssignment<XID> {
    type Output = MirAssignment<MangledVar>;
    fn mangle(self, scope: &mut Scope) -> Self::Output {
        MirAssignment {
            variable_name: scope.get_variable(self.variable_name),
            value: self.value.mangle(scope),
        }
    }
}

impl Mangle for MirDeclaration<XID> {
    type Output = MirDeclaration<MangledVar>;
    fn mangle(self, scope: &mut Scope) -> Self::Output {
        MirDeclaration {
            is_static: self.is_static,
            name: scope.new_variable(self.name),
            ty: self.ty,
            value: self.value.map(|val| val.mangle(scope)),
        }
    }
}

impl Mangle for MirFunction<XID> {
    type Output = MirFunction<MangledVar>;
    fn mangle(self, scope: &mut Scope) -> Self::Output {
        MirFunction {
            is_static: self.is_static,
            name: self.name,
            args: self
                .args
                .into_iter()
                .map(|(name, ty)| (scope.new_variable(name), ty))
                .collect(),
            return_type: self.return_type,
            block: scope.mangle_unnamed_child(self.block),
        }
    }
}

impl Mangle for MirFor<XID> {
    type Output = MirFor<MangledVar>;
    fn mangle(self, scope: &mut Scope) -> Self::Output {
        let id = scope.hash_unnamed();
        let mut child_scope = scope.new_child(id);

        MirFor {
            init: self.init.mangle(&mut child_scope),
            condition: self.condition.mangle(&mut child_scope),
            update: self.update.mangle(&mut child_scope),
            block: self.block.mangle(&mut child_scope),
        }
    }
}

impl Mangle for MirWhile<XID> {
    type Output = MirWhile<MangledVar>;
    fn mangle(self, scope: &mut Scope) -> Self::Output {
        MirWhile {
            condition: self.condition.mangle(scope),
            block: scope.mangle_unnamed_child(self.block),
        }
    }
}

impl Mangle for MirIf<XID> {
    type Output = MirIf<MangledVar>;
    fn mangle(self, scope: &mut Scope) -> Self::Output {
        MirIf {
            condition: self.condition.mangle(scope),
            block: scope.mangle_unnamed_child(self.block),
            else_block: self.else_block.map(|block| block.mangle(scope)),
        }
    }
}

impl Mangle for MirElseBlock<XID> {
    type Output = MirElseBlock<MangledVar>;
    fn mangle(self, scope: &mut Scope) -> Self::Output {
        match self {
            MirElseBlock::ElseIf(if_block) => {
                MirElseBlock::ElseIf(Box::new(if_block.mangle(scope)))
            }
            MirElseBlock::Else(block) => MirElseBlock::Else(scope.mangle_unnamed_child(block)),
        }
    }
}
