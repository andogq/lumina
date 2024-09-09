use std::collections::HashSet;

use index_vec::IndexVec;

use crate::{
    repr::identifier::{FunctionIdx, ScopedBinding},
    ty::FunctionSignature,
};

use super::BasicBlock;

index_vec::define_index_type! {
    pub struct BasicBlockIdx = usize;
}

#[derive(Debug, Clone)]
pub struct Function {
    pub identifier: FunctionIdx,
    pub signature: FunctionSignature,
    pub basic_blocks: IndexVec<BasicBlockIdx, BasicBlock>,
    pub scope: HashSet<ScopedBinding>,
}
