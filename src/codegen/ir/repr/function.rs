use std::collections::HashSet;

use index_vec::IndexVec;

use crate::core::ctx::Symbol;

use super::BasicBlock;

index_vec::define_index_type! {
    pub struct BasicBlockIdx = usize;
}

#[derive(Debug, Clone)]
pub struct Function {
    pub symbol: Symbol,
    pub basic_blocks: IndexVec<BasicBlockIdx, BasicBlock>,
    pub scope: HashSet<Symbol>,
}

impl Function {
    pub fn new(symbol: Symbol) -> Self {
        Self {
            symbol,
            basic_blocks: IndexVec::new(),
            scope: HashSet::new(),
        }
    }
}
