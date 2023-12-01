pub use self::{
    absoluteify::{AbsolutePath, AbsoluteScope, AbsoluteVar, ToAbsolute},
    mangle::{Mangle, MangleScope, MangledVar},
    to_mir::{
        MirAssignment, MirDeclaration, MirElseBlock, MirExpression, MirFor, MirFunction, MirIf,
        MirStatement, MirType, MirWhile, RelativePath, ToMir,
    },
};
pub use crate::hir::mir::{MirBinaryOp, MirUnaryOp};
use crate::private::Sealed;
use std::fmt::Debug;

pub trait VariableName: Sealed {
    type VariablePath: Debug + Clone + PartialEq;
    type OtherPath: Debug + Clone + PartialEq;
}

//mod denest;
mod absoluteify;
mod mangle;
mod to_mir;
