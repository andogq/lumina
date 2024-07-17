use index_vec::{define_index_type, IndexVec};

use crate::{
    repr::{identifier::FunctionIdx, ty::Ty},
    util::symbol_map::interner_symbol_map::Symbol,
};

use super::FunctionSignature;

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

define_index_type! {pub struct ScopeIdx = usize;}
define_index_type! {pub struct BindingIdx = usize;}

/// A binding within a specific scope.
pub struct ScopedBinding(ScopeIdx, BindingIdx);

pub struct ScopePart {
    /// Indicates that this scope (and potentially a descendant) is active.
    active: bool,

    /// All of the bindings present within this scope.
    bindings: IndexVec<BindingIdx, (Symbol, Ty)>,
}

impl ScopePart {
    /// Create a new scope part, and automatically mark it as active.
    pub fn new() -> Self {
        Self {
            active: true,
            bindings: IndexVec::new(),
        }
    }

    pub fn exit(&mut self) {
        self.active = false;
    }

    pub fn add(&mut self, symbol: Symbol, ty: Ty) -> BindingIdx {
        self.bindings.push((symbol, ty))
    }
}

pub struct Scope {
    scopes: IndexVec<ScopeIdx, ScopePart>,
}

impl Scope {
    /// Create a new instance, and automatically enter the base scope.
    pub fn new() -> Self {
        let mut scope = Self {
            scopes: IndexVec::new(),
        };

        // Automatically enter the first scope
        scope.enter();

        scope
    }

    /// Enter a new scope.
    pub fn enter(&mut self) -> ScopeIdx {
        self.scopes.push(ScopePart::new())
    }

    /// Exit the most recent scope.
    pub fn leave(&mut self) {
        let Some(scope) = self.scopes.last_mut() else {
            return;
        };

        scope.exit();
    }

    /// Register a symbol and associated type, and produce a unique binding for it.
    pub fn register(&mut self, symbol: Symbol, ty: Ty) -> ScopedBinding {
        // Fetch the currently active scope
        let active_scope_idx = self.active_scope();
        let active_scope = &mut self.scopes[active_scope_idx];

        // Register the binding type and symbol
        let binding_idx = active_scope.add(symbol, ty);

        ScopedBinding(active_scope_idx, binding_idx)
    }

    /// Attempt to retrieve a binding and type for the provided symbol. Will only search for items that are in scope.
    pub fn resolve(&mut self, symbol: Symbol) -> Option<(ScopedBinding, Ty)> {
        // Run through all possible scopes
        self.scopes
            .iter_enumerated()
            // Only include active scopes
            .filter(|(_, scope)| scope.active)
            // Extract all of the bindings from the scopes
            .flat_map(|(scope_idx, scope)| {
                scope
                    .bindings
                    .iter_enumerated()
                    // Only consider bindings that match the symbol
                    .filter(|(_, (test_symbol, _))| *test_symbol == symbol)
                    // Generate the scoped binding representation to track which scope the binding originated from
                    .map(move |(binding_idx, (_, ty))| (ScopedBinding(scope_idx, binding_idx), *ty))
            })
            .next_back()
    }

    /// Find the currently activated scope identifier.
    fn active_scope(&self) -> ScopeIdx {
        self.scopes
            .iter_enumerated()
            .rev()
            .find(|(_, scope)| scope.active)
            .map(|(idx, _)| idx)
            .expect("should always be at least one scope active")
    }
}

impl Default for Scope {
    fn default() -> Self {
        Self::new()
    }
}
