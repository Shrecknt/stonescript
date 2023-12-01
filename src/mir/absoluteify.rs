use super::{
    MirAssignment, MirDeclaration, MirElseBlock, MirExpression, MirFor, MirFunction, MirIf,
    MirStatement, MirType, MirWhile, RelativePath, VariableName,
};
use crate::{private::Sealed, token::XID};
use std::{collections::HashMap, mem};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct AbsolutePath(Vec<AbsoluteVar>);
#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct AbsoluteVar(XID);
impl Sealed for AbsoluteVar {}
impl AbsoluteVar {
    pub fn inner(&self) -> &str {
        self.0.inner()
    }
}
impl AbsolutePath {
    pub fn inner(&self) -> &[AbsoluteVar] {
        &self.0
    }
}

impl VariableName for AbsoluteVar {
    type VariablePath = AbsolutePath;
    type OtherPath = AbsolutePath;
}

pub struct AbsoluteScope<'a> {
    parent: Option<&'a mut AbsoluteScope<'a>>,
    imports: HashMap<XID, AbsolutePath>,
}

impl<'a> AbsoluteScope<'a> {
    pub fn root_to_absolute<T: ToAbsolute>(value: T) -> T::Output {
        let mut scope = Self {
            parent: None,
            imports: HashMap::new(),
        };

        value.to_absolute(&mut scope)
    }

    pub fn new_child<T>(&mut self, closure: impl FnOnce(&mut AbsoluteScope) -> T) -> T {
        // SAFETY: Widens the scope of self, the reference does not live longer than this function.
        closure(&mut Self {
            parent: Some(unsafe { mem::transmute(self) }),
            imports: HashMap::new(),
        })
    }

    pub fn new_import(&mut self, path: RelativePath) -> AbsolutePath {
        let key = path.last().expect("Path should not be empty").clone();
        let path = path.to_absolute(self);

        self.imports.insert(key, path.clone());

        path
    }

    pub fn new_variable(&self, xid: XID) -> AbsoluteVar {
        AbsoluteVar(xid)
    }
}

pub trait ToAbsolute: Sized {
    type Output;
    fn to_absolute(self, scope: &mut AbsoluteScope) -> Self::Output;
}

impl ToAbsolute for RelativePath {
    type Output = AbsolutePath;
    fn to_absolute(self, scope: &mut AbsoluteScope) -> Self::Output {
        let (first, segments) = self.split_first().expect("Path should not be empty");
        if let Some(path) = scope.imports.get(first) {
            let mut path = path.clone();
            path.0
                .extend(segments.iter().map(|xid| scope.new_variable(xid.clone())));
            path
        } else if let Some(ref mut parent) = scope.parent {
            self.to_absolute(parent)
        } else {
            AbsolutePath(
                self.into_iter()
                    .map(|xid| scope.new_variable(xid))
                    .collect(),
            )
        }
    }
}

impl<T: ToAbsolute> ToAbsolute for Vec<T> {
    type Output = Vec<T::Output>;
    fn to_absolute(self, scope: &mut AbsoluteScope) -> Self::Output {
        self.into_iter()
            .map(|item| item.to_absolute(scope))
            .collect()
    }
}

impl ToAbsolute for MirStatement<XID> {
    type Output = MirStatement<AbsoluteVar>;
    fn to_absolute(self, scope: &mut AbsoluteScope) -> Self::Output {
        match self {
            Self::Import(path) => MirStatement::Import(scope.new_import(path.clone())),
            Self::Assignment(assign) => MirStatement::Assignment(assign.to_absolute(scope)),
            Self::Block(block) => {
                MirStatement::Block(scope.new_child(|scope| block.to_absolute(scope)))
            }
            Self::Declaration(decl) => MirStatement::Declaration(decl.to_absolute(scope)),
            Self::Expression(expr) => MirStatement::Expression(expr.to_absolute(scope)),
            Self::For(for_loop) => MirStatement::For(Box::new(for_loop.to_absolute(scope))),
            Self::Function(func) => MirStatement::Function(func.to_absolute(scope)),
            Self::If(if_block) => MirStatement::If(if_block.to_absolute(scope)),
            Self::Return(expr) => MirStatement::Return(expr.to_absolute(scope)),
            Self::Unsafe(block) => {
                MirStatement::Unsafe(scope.new_child(|scope| block.to_absolute(scope)))
            }
            Self::While(while_loop) => MirStatement::While(while_loop.to_absolute(scope)),
        }
    }
}

impl ToAbsolute for MirExpression<XID> {
    type Output = MirExpression<AbsoluteVar>;
    fn to_absolute(self, scope: &mut AbsoluteScope) -> Self::Output {
        match self {
            Self::BinaryOp(left, op, right) => MirExpression::BinaryOp(
                Box::new(left.to_absolute(scope)),
                op,
                Box::new(right.to_absolute(scope)),
            ),
            Self::Call(path, args) => {
                MirExpression::Call(path.to_absolute(scope), args.to_absolute(scope))
            }
            Self::Command(cmd) => MirExpression::Command(cmd),
            Self::Index(left, index) => MirExpression::Index(
                Box::new(left.to_absolute(scope)),
                Box::new(index.to_absolute(scope)),
            ),
            Self::Literal(literal) => MirExpression::Literal(literal),
            Self::Property(left, property) => {
                MirExpression::Property(Box::new(left.to_absolute(scope)), property)
            }
            Self::UnaryOp(op, expr) => {
                MirExpression::UnaryOp(op, Box::new(expr.to_absolute(scope)))
            }
            Self::Variable(path) => MirExpression::Variable(path.to_absolute(scope)),
        }
    }
}

impl ToAbsolute for MirAssignment<XID> {
    type Output = MirAssignment<AbsoluteVar>;
    fn to_absolute(self, scope: &mut AbsoluteScope) -> Self::Output {
        MirAssignment {
            variable: self.variable.to_absolute(scope),
            value: self.value.to_absolute(scope),
        }
    }
}

impl ToAbsolute for MirDeclaration<XID> {
    type Output = MirDeclaration<AbsoluteVar>;
    fn to_absolute(self, scope: &mut AbsoluteScope) -> Self::Output {
        MirDeclaration {
            is_static: self.is_static,
            name: scope.new_variable(self.name),
            ty: self.ty.to_absolute(scope),
            value: self.value.map(|val| val.to_absolute(scope)),
        }
    }
}

impl ToAbsolute for MirType<XID> {
    type Output = MirType<AbsoluteVar>;
    fn to_absolute(self, scope: &mut AbsoluteScope) -> Self::Output {
        match self {
            Self::Primitive(primitive) => MirType::Primitive(primitive),
            Self::UserDefined(path) => MirType::UserDefined(path.to_absolute(scope)),
        }
    }
}

impl ToAbsolute for MirFunction<XID> {
    type Output = MirFunction<AbsoluteVar>;
    fn to_absolute(self, scope: &mut AbsoluteScope) -> Self::Output {
        let (args, block) = scope.new_child(|scope| {
            (
                self.args
                    .into_iter()
                    .map(|(xid, ty)| (scope.new_variable(xid), ty.to_absolute(scope)))
                    .collect(),
                self.block.to_absolute(scope),
            )
        });

        MirFunction {
            is_static: self.is_static,
            name: self.name,
            args,
            return_type: self.return_type.to_absolute(scope),
            block,
        }
    }
}

impl ToAbsolute for MirFor<XID> {
    type Output = MirFor<AbsoluteVar>;
    fn to_absolute(self, scope: &mut AbsoluteScope) -> Self::Output {
        scope.new_child(|scope| MirFor {
            init: self.init.to_absolute(scope),
            condition: self.condition.to_absolute(scope),
            update: self.update.to_absolute(scope),
            block: self.block.to_absolute(scope),
        })
    }
}

impl ToAbsolute for MirIf<XID> {
    type Output = MirIf<AbsoluteVar>;
    fn to_absolute(self, scope: &mut AbsoluteScope) -> Self::Output {
        MirIf {
            condition: self.condition.to_absolute(scope),
            block: scope.new_child(|scope| self.block.to_absolute(scope)),
            else_block: self.else_block.map(|val| val.to_absolute(scope)),
        }
    }
}

impl ToAbsolute for MirElseBlock<XID> {
    type Output = MirElseBlock<AbsoluteVar>;
    fn to_absolute(self, scope: &mut AbsoluteScope) -> Self::Output {
        match self {
            Self::Else(block) => {
                MirElseBlock::Else(scope.new_child(|scope| block.to_absolute(scope)))
            }
            Self::ElseIf(if_block) => MirElseBlock::ElseIf(Box::new(if_block.to_absolute(scope))),
        }
    }
}

impl ToAbsolute for MirWhile<XID> {
    type Output = MirWhile<AbsoluteVar>;
    fn to_absolute(self, scope: &mut AbsoluteScope) -> Self::Output {
        MirWhile {
            condition: self.condition.to_absolute(scope),
            block: scope.new_child(|scope| self.block.to_absolute(scope)),
        }
    }
}
