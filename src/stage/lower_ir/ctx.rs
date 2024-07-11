use index_vec::{define_index_type, IndexVec};

use crate::{repr::ir::Function, util::symbol_map::SymbolMap};

define_index_type! {pub struct FunctionIdx = usize;}

#[derive(Default, Clone, Debug)]
pub struct IRCtx {
    /// Map of function symbol to the basic block entry point
    pub functions: IndexVec<FunctionIdx, Function>,
    pub symbol_map: SymbolMap,
}

impl IRCtx {
    pub fn new(symbol_map: SymbolMap) -> Self {
        Self {
            symbol_map,
            ..Default::default()
        }
    }
}
