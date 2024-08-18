use std::collections::HashMap;

use index_vec::IndexVec;

use crate::{
    repr::{
        identifier::{FunctionIdx, ScopedBinding},
        ty::Ty,
    },
    stage::type_check::FunctionSignature,
};

use super::Symbol;

/// Handles all of the bindings and information for functions.
#[derive(Default, Debug, Clone)]
pub struct FunctionManager {
    /// Function registrations, stored against a unique index.
    registrations: IndexVec<FunctionIdx, FunctionRegistration>,

    /// Map to resolve a function's symbol into it's index.
    symbols: HashMap<Symbol, FunctionIdx>,
}

impl FunctionManager {
    /// Register a new function signature against a signature, producing a new index for it.
    pub fn register(&mut self, symbol: Symbol, signature: FunctionSignature) -> FunctionIdx {
        let idx = self
            .registrations
            .push(FunctionRegistration::new(signature));

        assert!(
            !self.symbols.contains_key(&symbol),
            "cannot register function with symbol that's already been registered"
        );
        self.symbols.insert(symbol, idx);

        idx
    }

    /// Attempt to get the index associated with a symbol.
    pub fn get_idx(&self, symbol: Symbol) -> Option<FunctionIdx> {
        self.symbols.get(&symbol).cloned()
    }

    /// Get the registration associated with a [`FunctionIdx`].
    pub fn get(&self, idx: FunctionIdx) -> Option<&FunctionRegistration> {
        self.registrations.get(idx)
    }

    /// Get the mutable registration associated with a [`FunctionIdx`].
    pub fn get_mut(&mut self, idx: FunctionIdx) -> Option<&mut FunctionRegistration> {
        self.registrations.get_mut(idx)
    }

    /// Retrieve the original symbol for a function.
    pub fn symbol_for(&self, idx: FunctionIdx) -> Option<Symbol> {
        self.symbols
            .iter()
            .find(|(_, test_idx)| idx == **test_idx)
            .map(|(symbol, _)| *symbol)
    }

    /// Produce an iterator of all function registrations.
    pub fn iter(&self) -> impl Iterator<Item = (FunctionIdx, &FunctionRegistration)> {
        self.registrations.iter_enumerated()
    }
}

/// compile process, including the signature, type information, and other representations.
#[derive(Debug, Clone)]
pub struct FunctionRegistration {
    /// Signature of the function.
    signature: FunctionSignature,

    /// Information for every binding within the function, containing symbol and type information.
    bindings: HashMap<ScopedBinding, (Symbol, Ty)>,
}

impl FunctionRegistration {
    /// Create a new function registration with the provided signature.
    fn new(signature: FunctionSignature) -> Self {
        Self {
            signature,
            bindings: HashMap::new(),
        }
    }

    /// Register a new binding and its information to this function.
    pub fn register_binding(&mut self, scoped_binding: ScopedBinding, symbol: Symbol, ty: Ty) {
        assert!(
            !self.bindings.contains_key(&scoped_binding),
            "cannot register binding that's already been registered"
        );

        self.bindings.insert(scoped_binding, (symbol, ty));
    }

    pub fn get_binding(&self, scoped_binding: ScopedBinding) -> Option<(Symbol, Ty)> {
        self.bindings.get(&scoped_binding).cloned()
    }

    /// Get a reference to the signature.
    pub fn get_signature(&self) -> &FunctionSignature {
        &self.signature
    }
}
