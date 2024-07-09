use crate::util::symbol_map::SymbolMap;

pub type Symbol = string_interner::DefaultSymbol;

// TODO: Symbol type should be an associated type
pub trait SymbolMapTrait {
    // TODO: Get rido f this
    fn intern(&mut self, s: impl AsRef<str>) -> Symbol;
    fn get(&self, s: Symbol) -> String;
    fn dump_symbols(&self) -> SymbolMap;
}
