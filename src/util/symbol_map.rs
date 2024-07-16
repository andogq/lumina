use string_interner::{DefaultBackend, StringInterner};

use crate::ctx::{Symbol, SymbolMapTrait};

pub type SymbolMap = StringInterner<DefaultBackend>;

impl SymbolMapTrait for SymbolMap {
    fn intern<T>(&mut self, s: T) -> Symbol
    where
        T: AsRef<str>,
    {
        self.get_or_intern(s)
    }

    fn get(&self, s: Symbol) -> String {
        self.resolve(s).unwrap().to_string()
    }

    fn dump_symbols(&self) -> SymbolMap {
        self.clone()
    }
}
