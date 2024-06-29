use std::collections::HashMap;

use crate::core::symbol::Symbol;

use super::BasicBlock;

#[derive(Default, Clone, Debug)]
pub struct IRContext {
    /// Map of function symbol to the basic block entry point
    pub functions: HashMap<Symbol, usize>,
    pub basic_blocks: Vec<BasicBlock>,
    pub symbols: Vec<Symbol>,
}

impl IRContext {
    /// Create a new basic block, returning a reference to it.
    pub(super) fn new_basic_block(&mut self) -> usize {
        let id = self.basic_blocks.len();
        self.basic_blocks.push(BasicBlock::default());
        id
    }
}
