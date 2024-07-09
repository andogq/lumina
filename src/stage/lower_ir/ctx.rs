use index_vec::{define_index_type, IndexVec};

use crate::{
    core::ctx::{Symbol, SymbolMap},
    repr::ir::Function,
};

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

    pub fn function_for_name(&mut self, name: impl AsRef<str>) -> Option<FunctionIdx> {
        let symbol = self.symbol_map.get(name)?;

        self.function_for_symbol(symbol)
    }

    // WTF
    pub fn function_for_symbol(&mut self, symbol: Symbol) -> Option<FunctionIdx> {
        self.functions
            .iter_enumerated()
            .find(|(_, f)| f.symbol == symbol)
            .map(|(i, _)| i)
    }
}
