//! A context that implements all required traits for any stage in the compiler. Isn't functional,
//! but good enough to test with.

use crate::{
    ctx::{Symbol, SymbolMapTrait},
    util::symbol_map::SymbolMap,
};

// TODO: Move this to a better place
#[derive(Default)]
pub struct TestCtx {
    symbols: SymbolMap,
}

impl SymbolMapTrait for TestCtx {
    fn intern(&mut self, s: impl AsRef<str>) -> Symbol {
        SymbolMapTrait::intern(&mut self.symbols, s)
    }

    fn get(&self, s: Symbol) -> String {
        SymbolMapTrait::get(&self.symbols, s)
    }

    fn dump_symbols(&self) -> SymbolMap {
        SymbolMapTrait::dump_symbols(&self.symbols)
    }
}
