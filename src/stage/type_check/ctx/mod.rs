use crate::{compiler::Symbol, repr::identifier::FunctionIdx, util::scope::*};

use super::FunctionSignature;

pub trait TypeCheckCtx {
    /// Register a function's signature and associated symbol, to produce a unique identifier for the function.
    fn register_function(&mut self, symbol: Symbol, signature: FunctionSignature) -> FunctionIdx;

    /// Store the scope for a function
    fn set_function_scope(&mut self, function: FunctionIdx, scope: Scope);

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
        fn set_function_scope(&mut self, function: FunctionIdx, scope: Scope);
        fn get_function(&self, idx: FunctionIdx) -> FunctionSignature;
        fn lookup_function_symbol(&self, symbol: Symbol) -> Option<FunctionIdx>;
    }
}
