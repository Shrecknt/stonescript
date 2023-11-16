pub use self::{
    mangle::{Mangle, MangledVar, Scope},
    to_mir::{
        MirAssignment, MirDeclaration, MirElseBlock, MirExpression, MirFor, MirFunction, MirIf,
        MirPath, MirStatement, MirType, MirWhile, ToMir,
    },
};
pub use crate::hir::mir::{MirBinaryOp, MirUnaryOp};
use crate::private::Sealed;
use std::fmt::Debug;

pub trait VariableName: Sealed {
    type Path: Debug + Clone + PartialEq;
}

//mod denest;
mod mangle;
mod to_mir;
