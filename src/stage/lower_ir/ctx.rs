use std::collections::HashMap;

use crate::{
    repr::{identifier::FunctionIdx, ir::Function},
    util::symbol_map::interner_symbol_map::InternerSymbolMap,
};

#[derive(Default, Clone, Debug)]
pub struct IRCtx {
    /// Map of function symbol to the basic block entry point
    pub functions: HashMap<FunctionIdx, Function>,
    pub symbol_map: InternerSymbolMap,
}

impl IRCtx {
    pub fn new(symbol_map: InternerSymbolMap) -> Self {
        Self {
            symbol_map,
            ..Default::default()
        }
    }
}
