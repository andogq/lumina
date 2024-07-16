use crate::ctx::SymbolMapTrait;

/// All required methods to provide a context for the parser.
pub trait ParseCtx: SymbolMapTrait {}

#[cfg(test)]
mockall::mock! {
    pub ParseCtx {}

    impl SymbolMapTrait for ParseCtx {
        #[mockall::concretize]
        fn intern<T>(&mut self, s: T) -> crate::ctx::Symbol where T: AsRef<str>;
        fn get(&self, s: crate::ctx::Symbol) -> String;
        fn dump_symbols(&self) -> crate::util::symbol_map::SymbolMap;
    }

    impl ParseCtx for ParseCtx {}
}
