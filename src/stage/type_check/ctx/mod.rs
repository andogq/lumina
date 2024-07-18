mod scope;

use crate::{repr::identifier::FunctionIdx, util::symbol_map::interner_symbol_map::Symbol};

use super::FunctionSignature;

pub use self::scope::*;

pub trait TypeCheckCtx {
    /// Register a function's signature and associated symbol, to produce a unique identifier for the function.
    fn register_function(&mut self, symbol: Symbol, signature: FunctionSignature) -> FunctionIdx;

    /// Get the signature associated with a function identifier.
    fn get_function(&self, idx: FunctionIdx) -> FunctionSignature;

    /// Attempt to look up a symbol, returning the associated function's identifier if it exists.
    fn lookup_function_symbol(&self, symbol: Symbol) -> Option<FunctionIdx>;
}

#[cfg(test)]
mockall::mock! {
    pub TypeCheckCtx {}

    impl TypeCheckCtx for TypeCheckCtx {
        fn register_function(&mut self, symbol: Symbol, signature: FunctionSignature) -> FunctionIdx;
        fn get_function(&self, idx: FunctionIdx) -> FunctionSignature;
        fn lookup_function_symbol(&self, symbol: Symbol) -> Option<FunctionIdx>;
    }
}
