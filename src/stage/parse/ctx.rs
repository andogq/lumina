use crate::util::symbol_map::{interner_symbol_map::Symbol, SymbolMap};

/// All required methods to provide a context for the parser.
pub trait ParseCtx: SymbolMap<Symbol = Symbol> {}

#[cfg(test)]
mockall::mock! {
    pub ParseCtx {}

    impl SymbolMap for ParseCtx {
        type Symbol = Symbol;

        #[mockall::concretize]
        fn intern<T>(&mut self, s: T) -> Symbol where T: AsRef<str>;
        fn get(&self, s: Symbol) -> String;
        fn dump_symbols(&self) -> crate::util::symbol_map::interner_symbol_map::InternerSymbolMap;
    }

    impl ParseCtx for ParseCtx {}
}
