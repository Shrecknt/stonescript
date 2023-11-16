use std::fmt;

pub use self::{
    mangle::{Mangle, Scope},
    to_mir::{
        MirAssignment, MirDeclaration, MirElseBlock, MirExpression, MirFor, MirFunction, MirIf,
        MirStatement, MirType, MirWhile, ToMir,
    },
};
pub use crate::hir::mir::{MirBinaryOp, MirUnaryOp};
use crate::{private::Sealed, token::XID};

pub trait VariableName: Sealed {}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct MangledVar(u64);
impl fmt::Display for MangledVar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:016X}", self.0)
    }
}
impl fmt::Debug for MangledVar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MangledVar({})", self.to_string())
    }
}

impl Sealed for MangledVar {}
impl VariableName for XID {}
impl VariableName for MangledVar {}

//mod denest;
mod mangle;
mod to_mir;
