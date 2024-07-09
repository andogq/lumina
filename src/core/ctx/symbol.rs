use string_interner::{DefaultBackend, DefaultSymbol, StringInterner};

pub type Symbol = DefaultSymbol;
pub type SymbolMap = StringInterner<DefaultBackend>;

pub trait SymbolMapTrait {
    fn dump_symbols(&self) -> SymbolMap;
    fn intern(&mut self, s: impl AsRef<str>) -> Symbol;
    fn get(&self, s: Symbol) -> String;
}

impl SymbolMapTrait for SymbolMap {
    fn intern(&mut self, s: impl AsRef<str>) -> Symbol {
        self.get_or_intern(s)
    }

    fn get(&self, s: Symbol) -> String {
        self.resolve(s).unwrap().to_string()
    }

    fn dump_symbols(&self) -> SymbolMap {
        self.clone()
    }
}
