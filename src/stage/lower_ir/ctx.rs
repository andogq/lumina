use std::collections::HashMap;

use index_vec::{define_index_type, IndexVec};

use crate::{ctx::Symbol, repr::ir::Function, util::symbol_map::SymbolMap};

define_index_type! {pub struct FunctionIdx = usize;}

#[derive(Default, Clone, Debug)]
pub struct IRCtx {
    /// Map of function symbol to the basic block entry point
    pub functions: HashMap<Symbol, Function>,
    pub symbol_map: SymbolMap,
}

impl IRCtx {
    pub fn new(symbol_map: SymbolMap) -> Self {
        Self {
            symbol_map,
            ..Default::default()
        }
    }

    pub fn function_for_name(&self, s: &str) -> Option<Function> {
        let symbol = self.symbol_map.get(s)?;
        self.functions.get(&symbol).cloned()
    }
}
