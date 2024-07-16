use crate::{
    ctx::{Symbol, SymbolMapTrait},
    stage::parse::ParseCtx,
    util::symbol_map::SymbolMap,
};

#[derive(Default)]
pub struct CompilePass {
    symbols: SymbolMap,
}

impl SymbolMapTrait for CompilePass {
    fn intern<T>(&mut self, s: T) -> Symbol
    where
        T: AsRef<str>,
    {
        SymbolMapTrait::intern(&mut self.symbols, s)
    }

    fn get(&self, s: Symbol) -> String {
        SymbolMapTrait::get(&self.symbols, s)
    }

    fn dump_symbols(&self) -> SymbolMap {
        SymbolMapTrait::dump_symbols(&self.symbols)
    }
}

impl ParseCtx for CompilePass {}
