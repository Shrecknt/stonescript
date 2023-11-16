use super::{MirBinaryOp, MirUnaryOp, VariableName};
use crate::{
    hir::{
        mir::MirPrimitive, Assignment, DeclStart, Declaration, ElseBlock, Expression, ForLoop,
        FunctionDecl, IfBlock, Path, Statement, Type, WhileLoop,
    },
    token::{LiteralType, XID},
};

pub trait ToMir {
    type Output;

    fn into_mir(self) -> Self::Output;
}

pub type MirPath = Vec<XID>;
impl VariableName for XID {
    type Path = MirPath;
}

impl ToMir for Path {
    type Output = MirPath;
    fn into_mir(self) -> Self::Output {
        self.into_tokens()
            .into_iter()
            .map(|ident| ident.into_inner())
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum MirStatement<V: VariableName> {
    Block(Vec<MirStatement<V>>),
    Unsafe(Vec<MirStatement<V>>),
    Expression(MirExpression<V>),
    Return(MirExpression<V>),
    Assignment(MirAssignment<V>),
    Declaration(MirDeclaration<V>),
    Function(MirFunction<V>),
    If(MirIf<V>),
    While(MirWhile<V>),
    For(Box<MirFor<V>>),
}

impl ToMir for Statement {
    type Output = MirStatement<XID>;

    fn into_mir(self) -> Self::Output {
        match self {
            Self::Block(block) => MirStatement::Block(block.into_contents().into_mir()),
            Self::Unsafe((_, block)) => MirStatement::Unsafe(block.into_contents().into_mir()),
            Self::Expression((expr, _)) => MirStatement::Expression(expr.into_mir()),
            Self::Return((_, expr, _)) => MirStatement::Return(expr.into_mir()),
            Self::Assignment(assign) => MirStatement::Assignment(assign.into_mir()),
            Self::Declaration(decl) => MirStatement::Declaration(decl.into_mir()),
            Self::Function(func) => MirStatement::Function(func.into_mir()),
            Self::If(if_block) => MirStatement::If(if_block.into_mir()),
            Self::While(while_loop) => MirStatement::While(while_loop.into_mir()),
            Self::For(for_loop) => MirStatement::For(Box::new(for_loop.into_mir())),
        }
    }
}

impl ToMir for Vec<Statement> {
    type Output = Vec<MirStatement<XID>>;

    fn into_mir(self) -> Self::Output {
        self.into_iter().map(ToMir::into_mir).collect()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum MirExpression<V: VariableName> {
    Literal(LiteralType),
    Variable(V::Path),
    Property(Box<MirExpression<V>>, XID),
    Call(MirPath, Vec<MirExpression<V>>),
    Index(Box<MirExpression<V>>, Box<MirExpression<V>>),
    UnaryOp(MirUnaryOp, Box<MirExpression<V>>),
    BinaryOp(Box<MirExpression<V>>, MirBinaryOp, Box<MirExpression<V>>),
}

impl ToMir for Expression {
    type Output = MirExpression<XID>;

    fn into_mir(self) -> Self::Output {
        match self {
            Self::Literal(literal) => MirExpression::Literal(literal.into_inner()),
            Self::Variable(path) => MirExpression::Variable(path.into_mir()),
            Self::Property(expr, _, ident) => {
                MirExpression::Property(Box::new(expr.into_mir()), ident.into_inner())
            }
            Self::Call(path, args) => MirExpression::Call(
                path.into_mir(),
                args.into_contents()
                    .into_tokens()
                    .into_iter()
                    .map(ToMir::into_mir)
                    .collect(),
            ),
            Self::Parenthesized(paren) => paren.into_contents().into_mir(),
            Self::Index(expr, args) => MirExpression::Index(
                Box::new(expr.into_mir()),
                Box::new(args.into_contents().into_mir()),
            ),
            Self::UnaryOp(op, expr) => {
                MirExpression::UnaryOp(op.into_mir(), Box::new(expr.into_mir()))
            }
            Self::BinaryOp(left, op, right) => MirExpression::BinaryOp(
                Box::new(left.into_mir()),
                op.into_mir(),
                Box::new(right.into_mir()),
            ),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum MirType {
    Primitive(MirPrimitive),
    UserDefined(MirPath),
}

impl ToMir for Type {
    type Output = MirType;

    fn into_mir(self) -> Self::Output {
        match self {
            Self::Primitive(primitive) => MirType::Primitive(primitive.into_mir()),
            Self::UserDefined(ident) => MirType::UserDefined(ident.into_mir()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MirDeclaration<V: VariableName> {
    pub is_static: bool,
    pub name: V,
    pub ty: MirType,
    pub value: Option<MirExpression<V>>,
}

impl ToMir for Declaration {
    type Output = MirDeclaration<XID>;

    fn into_mir(self) -> Self::Output {
        MirDeclaration {
            is_static: match self.start_token {
                DeclStart::Static(_) => true,
                DeclStart::Let(_) => false,
            },
            name: self.ident.into_inner(),
            ty: self.ty.into_mir(),
            value: self.value.map(|(_, expr)| expr.into_mir()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MirFunction<V: VariableName> {
    pub is_static: bool,
    pub name: XID,
    pub args: Vec<(V, MirType)>,
    pub return_type: MirType,
    pub block: Vec<MirStatement<V>>,
}

impl ToMir for FunctionDecl {
    type Output = MirFunction<XID>;

    fn into_mir(self) -> Self::Output {
        MirFunction {
            is_static: self.staticness.is_some(),
            name: self.ident.into_inner(),
            args: self
                .args
                .into_contents()
                .into_tokens()
                .into_iter()
                .map(|arg| (arg.name.into_inner(), arg.ty.into_mir()))
                .collect(),
            return_type: self.return_type.into_mir(),
            block: self.block.into_contents().into_mir(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MirIf<V: VariableName> {
    pub condition: MirExpression<V>,
    pub block: Vec<MirStatement<V>>,
    pub else_block: Option<MirElseBlock<V>>,
}

impl ToMir for IfBlock {
    type Output = MirIf<XID>;

    fn into_mir(self) -> Self::Output {
        MirIf {
            condition: self.condition.into_contents().into_mir(),
            block: self.block.into_contents().into_mir(),
            else_block: self.else_block.map(|(_, else_block)| else_block.into_mir()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum MirElseBlock<V: VariableName> {
    ElseIf(Box<MirIf<V>>),
    Else(Vec<MirStatement<V>>),
}

impl ToMir for ElseBlock {
    type Output = MirElseBlock<XID>;

    fn into_mir(self) -> Self::Output {
        match self {
            Self::ElseIf(else_if) => MirElseBlock::ElseIf(Box::new(else_if.into_mir())),
            Self::Else(block) => MirElseBlock::Else(block.into_contents().into_mir()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MirWhile<V: VariableName> {
    pub condition: MirExpression<V>,
    pub block: Vec<MirStatement<V>>,
}

impl ToMir for WhileLoop {
    type Output = MirWhile<XID>;

    fn into_mir(self) -> Self::Output {
        MirWhile {
            condition: self.condition.into_contents().into_mir(),
            block: self.block.into_contents().into_mir(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MirFor<V: VariableName> {
    pub init: MirDeclaration<V>,
    pub condition: MirExpression<V>,
    pub update: MirStatement<V>,
    pub block: Vec<MirStatement<V>>,
}

impl ToMir for ForLoop {
    type Output = MirFor<XID>;

    fn into_mir(self) -> Self::Output {
        let inner = self.inner.into_contents();
        MirFor {
            init: inner.init.into_mir(),
            condition: inner.condition.0.into_mir(),
            update: inner.update.into_mir(),
            block: self.block.into_contents().into_mir(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MirAssignment<V: VariableName> {
    pub variable: V::Path,
    pub value: MirExpression<V>,
}

impl ToMir for Assignment {
    type Output = MirAssignment<XID>;

    fn into_mir(self) -> Self::Output {
        MirAssignment {
            variable: self.variable.into_mir(),
            value: self.value.into_mir(),
        }
    }
}
