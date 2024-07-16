/// A symbol map must be able to map a string to and from a symbol that represents it.
pub trait SymbolMap {
    /// The type of the symbol that will be produced.
    type Symbol;

    /// Intern the given string, producing a symbol that represents it.
    fn intern<T>(&mut self, s: T) -> Self::Symbol
    where
        T: AsRef<str>;

    /// Retrieve a string for the given symbol. It is assumed that it is not possible to obtain a
    /// [`Symbol`] without previously interning a value, which is why it does not return [`Option`].
    fn get(&self, s: Self::Symbol) -> String;

    // TODO: Get rid of this
    fn dump_symbols(&self) -> interner_symbol_map::InternerSymbolMap;
}

pub mod interner_symbol_map {
    //! A sample implementation of a symbol map that is backed by the [`string_interner`] crate.

    use super::*;
    use string_interner::{DefaultBackend, StringInterner};

    /// Alias for the default symbol for the [`string_interner`] crate.
    pub type Symbol = string_interner::DefaultSymbol;

    /// A symbol map backed by [`StringInterner`].
    pub type InternerSymbolMap = StringInterner<DefaultBackend>;

    impl SymbolMap for InternerSymbolMap {
        type Symbol = Symbol;

        fn intern<T>(&mut self, s: T) -> Self::Symbol
        where
            T: AsRef<str>,
        {
            self.get_or_intern(s)
        }

        fn get(&self, s: Self::Symbol) -> String {
            self.resolve(s).unwrap().to_string()
        }

        fn dump_symbols(&self) -> InternerSymbolMap {
            self.clone()
        }
    }
}
