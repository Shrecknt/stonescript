use super::{MirBinaryOp, MirUnaryOp};
use crate::{
    hir::{
        mir::MirPrimitive, Assignment, DeclStart, Declaration, ElseBlock, Expression,
        ForLoop, FunctionDecl, IfBlock, Statement, Type, WhileLoop,
    },
    token::{LiteralType, XID},
};

pub trait ToMir {
    type Output;

    fn into_mir(self) -> Self::Output;
}

#[derive(Debug, Clone, PartialEq)]
pub enum MirStatement {
    Block(Vec<MirStatement>),
    Unsafe(Vec<MirStatement>),
    Expression(MirExpression),
    Return(MirExpression),
    Assignment(MirAssignment),
    Declaration(MirDeclaration),
    Function(MirFunction),
    If(MirIf),
    While(MirWhile),
    For(MirFor),
}

impl ToMir for Statement {
    type Output = MirStatement;

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
            Self::For(for_loop) => MirStatement::For(for_loop.into_mir()),
        }
    }
}

impl ToMir for Vec<Statement> {
    type Output = Vec<MirStatement>;

    fn into_mir(self) -> Self::Output {
        self.into_iter()
            .map(ToMir::into_mir)
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum MirExpression {
    Literal(LiteralType),
    Variable(XID),
    Property(Box<MirExpression>, XID),
    Call(Box<MirExpression>, Vec<MirExpression>),
    Index(Box<MirExpression>, Box<MirExpression>),
    UnaryOp(MirUnaryOp, Box<MirExpression>),
    BinaryOp(Box<MirExpression>, MirBinaryOp, Box<MirExpression>),
}

impl ToMir for Expression {
    type Output = MirExpression;

    fn into_mir(self) -> Self::Output {
        match self {
            Self::Literal(literal) => MirExpression::Literal(literal.into_inner()),
            Self::Variable(ident) => MirExpression::Variable(ident.into_inner()),
            Self::Property(expr, _, ident) => {
                MirExpression::Property(Box::new(expr.into_mir()), ident.into_inner())
            }
            Self::Call(expr, args) => MirExpression::Call(
                Box::new(expr.into_mir()),
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
    UserDefined(XID),
}

impl ToMir for Type {
    type Output = MirType;

    fn into_mir(self) -> Self::Output {
        match self {
            Self::Primitive(primitive) => MirType::Primitive(primitive.into_mir()),
            Self::UserDefined(ident) => MirType::UserDefined(ident.into_inner()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MirDeclaration {
    pub is_static: bool,
    pub name: XID,
    pub ty: MirType,
    pub value: Option<MirExpression>,
}

impl ToMir for Declaration {
    type Output = MirDeclaration;

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
pub struct MirFunction {
    pub is_static: bool,
    pub name: XID,
    pub args: Vec<(XID, MirType)>,
    pub return_type: MirType,
    pub block: Vec<MirStatement>,
}

impl ToMir for FunctionDecl {
    type Output = MirFunction;

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
pub struct MirIf {
    pub condition: MirExpression,
    pub block: Vec<MirStatement>,
    pub else_block: Option<MirElseBlock>,
}

impl ToMir for IfBlock {
    type Output = MirIf;

    fn into_mir(self) -> Self::Output {
        MirIf {
            condition: self.condition.into_contents().into_mir(),
            block: self.block.into_contents().into_mir(),
            else_block: self.else_block.map(|(_, else_block)| else_block.into_mir()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum MirElseBlock {
    ElseIf(Box<MirIf>),
    Else(Vec<MirStatement>),
}

impl ToMir for ElseBlock {
    type Output = MirElseBlock;

    fn into_mir(self) -> Self::Output {
        match self {
            Self::ElseIf(else_if) => MirElseBlock::ElseIf(Box::new(else_if.into_mir())),
            Self::Else(block) => MirElseBlock::Else(block.into_contents().into_mir()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MirWhile {
    pub condition: MirExpression,
    pub block: Vec<MirStatement>,
}

impl ToMir for WhileLoop {
    type Output = MirWhile;

    fn into_mir(self) -> Self::Output {
        MirWhile {
            condition: self.condition.into_contents().into_mir(),
            block: self.block.into_contents().into_mir(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MirFor {
    pub init: MirDeclaration,
    pub condition: MirExpression,
    pub update: MirAssignment,
    pub block: Vec<MirStatement>,
}

impl ToMir for ForLoop {
    type Output = MirFor;

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
pub struct MirAssignment {
    pub variable_name: XID,
    pub value: MirExpression,
}

impl ToMir for Assignment {
    type Output = MirAssignment;

    fn into_mir(self) -> Self::Output {
        MirAssignment {
            variable_name: self.variable_name.into_inner(),
            value: self.value.into_mir(),
        }
    }
}
