use index_vec::IndexVec;

use crate::{
    repr::{identifier::*, ty::Ty},
    util::symbol_map::interner_symbol_map::Symbol,
};

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

impl Default for ScopePart {
    fn default() -> Self {
        Self::new()
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
    pub fn leave(&mut self) -> ScopeIdx {
        // Find the active scope
        let active = self.active_scope();

        // Exit the active scope
        self.scopes[active].exit();

        // Return the scope index
        active
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

    /// Get the information for a binding.
    pub fn get_binding(&self, binding: &ScopedBinding) -> (Symbol, Ty) {
        self.scopes[binding.0].bindings[binding.1]
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

#[cfg(test)]
mod test {
    use super::*;

    use rstest::*;
    use string_interner::Symbol as _;

    #[rstest]
    #[case::single_variable(&[(0, Ty::Int)])]
    #[case::multiple_variable(&[(0, Ty::Int), (1, Ty::Boolean)])]
    fn single_scope_variable_registration(#[case] variables: &[(usize, Ty)]) {
        let mut scope = Scope::new();

        // Convert all the identifier numbers into `Symbol`s.
        let variables = variables
            .iter()
            .map(|(symbol, ty)| (Symbol::try_from_usize(*symbol).unwrap(), *ty))
            .collect::<Vec<_>>();

        // Register all of the variables and capture the bindings
        let bindings = variables
            .iter()
            .map(|(symbol, ty)| scope.register(*symbol, *ty))
            .collect::<Vec<_>>();

        // Resolve each of the bindings and verify they match
        for ((symbol, ty), binding) in variables.into_iter().zip(bindings.into_iter()) {
            assert_eq!(scope.resolve(symbol).unwrap(), (binding, ty));
        }
    }

    #[test]
    fn shadowing() {
        let symbol = Symbol::try_from_usize(0).unwrap();

        let mut scope = Scope::new();

        // Register the underlying variable
        let int_binding = scope.register(symbol, Ty::Int);

        // Register the shadowed variable
        let boolean_binding = scope.register(symbol, Ty::Boolean);

        // Make sure that the bindings are unique
        assert_ne!(int_binding, boolean_binding);

        // Ensure that the boolean shadowed variable takes precedence
        assert_eq!(
            scope.resolve(symbol).unwrap(),
            (boolean_binding, Ty::Boolean)
        );
    }

    #[test]
    fn nested_scope() {
        let symbol_a = Symbol::try_from_usize(0).unwrap();
        let symbol_b = Symbol::try_from_usize(1).unwrap();

        let mut scope = Scope::new();

        // Register the first variable in the base scope
        let binding_a = scope.register(symbol_a, Ty::Int);

        // Enter a new scope
        let new_scope = scope.enter();

        // Register the second variable in the nested scope
        let binding_b = scope.register(symbol_b, Ty::Boolean);

        // Ensure the variables are placed in the correct locations
        assert_ne!(binding_a.0, new_scope, "first variable not in nested scope");
        assert_eq!(binding_b.0, new_scope, "nested variable in nested scope");

        // Make sure both variables can be resolved
        assert_eq!(scope.resolve(symbol_a).unwrap(), (binding_a, Ty::Int));
        assert_eq!(scope.resolve(symbol_b).unwrap(), (binding_b, Ty::Boolean));

        // Leave the scope
        scope.leave();

        // Verify variables
        assert_eq!(
            scope.resolve(symbol_a).unwrap(),
            (binding_a, Ty::Int),
            "outer variable must still be in scope"
        );
        assert!(
            scope.resolve(symbol_b).is_none(),
            "inner variable must no longer be in scope"
        );
    }

    #[test]
    fn nested_scope_shadowing() {
        let symbol = Symbol::try_from_usize(0).unwrap();

        let mut scope = Scope::new();

        // Register the first variable in the base scope
        let binding_int = scope.register(symbol, Ty::Int);

        // Enter a new scope
        let new_scope = scope.enter();

        // Shadow the variable
        let binding_boolean = scope.register(symbol, Ty::Boolean);

        // Ensure the variables are placed in the correct locations
        assert_ne!(
            binding_int.0, new_scope,
            "first variable not in nested scope"
        );
        assert_eq!(
            binding_boolean.0, new_scope,
            "nested variable in nested scope"
        );

        // Make sure the shadowed variable will be resolved
        assert_eq!(
            scope.resolve(symbol).unwrap(),
            (binding_boolean, Ty::Boolean)
        );

        // Leave the scope
        scope.leave();

        // Verify shadowed variable returned to original state
        assert_eq!(scope.resolve(symbol).unwrap(), (binding_int, Ty::Int),);
    }

    #[test]
    fn deeply_nested_scopes() {
        let mut scope = Scope::new();

        let scope_a = scope.enter();
        let scope_b = scope.enter();
        let scope_c = scope.enter();

        assert_eq!(scope.active_scope(), scope_c);
        assert_eq!(scope.leave(), scope_c);

        assert_eq!(scope.active_scope(), scope_b);
        assert_eq!(scope.leave(), scope_b);

        assert_eq!(scope.active_scope(), scope_a);
        assert_eq!(scope.leave(), scope_a);
    }
}
