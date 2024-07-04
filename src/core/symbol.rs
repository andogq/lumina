use string_interner::{DefaultBackend, DefaultSymbol, StringInterner};

pub type Symbol = DefaultSymbol;
pub type SymbolMap = StringInterner<DefaultBackend>;
