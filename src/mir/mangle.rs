use std::collections::HashMap;
use super::{to_mir::MirFunction, MirStatement};

pub enum Scope {
    Module(String),
    Function(MirFunction),
}

pub trait Mangle {
    fn mangle(self, parent_scope: Scope) -> Self;
}

impl Mangle for MirStatement {
    fn mangle(self, parent_scope: Scope) -> Self {
        let map = HashMap::new();

        
    }
}