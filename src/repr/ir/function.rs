use std::collections::HashSet;

use index_vec::IndexVec;

use crate::{
    repr::{
        ast::typed as ast,
        identifier::{FunctionIdx, ScopedBinding},
    },
    stage::type_check::FunctionSignature,
};

use super::BasicBlock;

index_vec::define_index_type! {
    pub struct BasicBlockIdx = usize;
}

#[derive(Debug, Clone)]
pub struct Function {
    pub symbol: FunctionIdx,
    pub signature: FunctionSignature,
    pub basic_blocks: IndexVec<BasicBlockIdx, BasicBlock>,
    pub scope: HashSet<ScopedBinding>,
}

impl Function {
    pub fn new(function: &ast::Function) -> Self {
        Self {
            symbol: function.name,
            signature: FunctionSignature::from(function),
            basic_blocks: IndexVec::new(),
            scope: dbg!(function
                .parameters
                .iter()
                .map(|(binding, _)| dbg!(*binding))
                .collect()),
        }
    }
}
