use crate::{
    stage::parse::ParseCtx,
    util::symbol_map::{interner_symbol_map::*, SymbolMap},
};

#[derive(Default)]
pub struct CompilePass {
    symbols: InternerSymbolMap,
}

impl SymbolMap for CompilePass {
    type Symbol = Symbol;

    fn intern<T>(&mut self, s: T) -> Symbol
    where
        T: AsRef<str>,
    {
        SymbolMap::intern(&mut self.symbols, s)
    }

    fn get(&self, s: Symbol) -> String {
        SymbolMap::get(&self.symbols, s)
    }

    fn dump_symbols(&self) -> InternerSymbolMap {
        SymbolMap::dump_symbols(&self.symbols)
    }
}

impl ParseCtx for CompilePass {}
