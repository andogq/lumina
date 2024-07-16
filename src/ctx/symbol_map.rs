use crate::util::symbol_map::SymbolMap;

pub type Symbol = string_interner::DefaultSymbol;

// TODO: Symbol type should be an associated type
pub trait SymbolMapTrait {
    fn intern<T>(&mut self, s: T) -> Symbol
    where
        T: AsRef<str>;
    fn get(&self, s: Symbol) -> String;

    // TODO: Get rid of this
    fn dump_symbols(&self) -> SymbolMap;
}
