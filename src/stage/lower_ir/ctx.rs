use std::collections::HashMap;

use index_vec::{define_index_type, IndexVec};

use crate::{
    repr::ir::Function,
    util::symbol_map::interner_symbol_map::{InternerSymbolMap, Symbol},
};

define_index_type! {pub struct FunctionIdx = usize;}

#[derive(Default, Clone, Debug)]
pub struct IRCtx {
    /// Map of function symbol to the basic block entry point
    pub functions: HashMap<Symbol, Function>,
    pub symbol_map: InternerSymbolMap,
}

impl IRCtx {
    pub fn new(symbol_map: InternerSymbolMap) -> Self {
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
