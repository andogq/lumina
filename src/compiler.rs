use std::collections::HashMap;

use index_vec::IndexVec;
use string_interner::{DefaultBackend, DefaultSymbol, StringInterner};

use crate::{
    repr::{
        identifier::{FunctionIdx, ScopedBinding},
        ir,
        ty::Ty,
    },
    stage::{
        self,
        parse::ParseError,
        type_check::{FunctionSignature, TyError},
    },
};

/// A symbol represents an interned string.
pub type Symbol = DefaultSymbol;

/// A function registration contains all of the information relating to a function throughout the
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
    fn register(&mut self, symbol: Symbol, signature: FunctionSignature) -> FunctionIdx {
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
    fn get_idx(&self, symbol: Symbol) -> Option<FunctionIdx> {
        self.symbols.get(&symbol).cloned()
    }

    /// Get the registration associated with a [`FunctionIdx`].
    fn get_function(&self, idx: FunctionIdx) -> Option<&FunctionRegistration> {
        self.registrations.get(idx)
    }

    /// Get the mutable registration associated with a [`FunctionIdx`].
    fn get_function_mut(&mut self, idx: FunctionIdx) -> Option<&mut FunctionRegistration> {
        self.registrations.get_mut(idx)
    }

    /// Retrieve the original symbol for a function.
    pub fn get_function_symbol(&self, idx: FunctionIdx) -> Option<Symbol> {
        self.symbols
            .iter()
            .find(|(_, test_idx)| idx == **test_idx)
            .map(|(symbol, _)| *symbol)
    }

    /// Produce an iterator of all function registrations.
    pub fn functions(&self) -> impl Iterator<Item = (FunctionIdx, &FunctionRegistration)> {
        self.registrations.iter_enumerated()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CompilerError {
    #[error(transparent)]
    Parse(#[from] ParseError),

    #[error(transparent)]
    Ty(#[from] TyError),
}

/// Contains all of the state required for a compiler pass.
#[derive(Default, Debug, Clone)]
pub struct Compiler {
    /// Symbols interned by the compiler.
    pub symbols: StringInterner<DefaultBackend>,

    /// All functions that have been registered with the compiler.
    pub function_manager: FunctionManager,
}

impl Compiler {
    /// From the provided source, compile and return the IR of each of the functions.
    pub fn compile(&mut self, source: impl AsRef<str>) -> Result<Vec<ir::Function>, CompilerError> {
        // Parse the source
        let program = stage::parse::parse(self, source.as_ref())?;

        // Perform type checking
        let program = program.ty_solve(self)?;

        // Lower into IR
        let ir = stage::lower_ir::lower(self, program);

        Ok(ir)
    }

    /// Intern the provided string, producing a [`Symbol`] that will represent it.
    pub fn intern_string(&mut self, string: impl AsRef<str>) -> Symbol {
        self.symbols.get_or_intern(string)
    }

    /// Resolve a [`Symbol`] into its underlying string.
    pub fn get_interned_string(&self, symbol: Symbol) -> Option<&str> {
        self.symbols.resolve(symbol)
    }

    /// Checks if a string has been interned.
    pub fn has_interned_string(&self, string: impl AsRef<str>) -> bool {
        self.symbols.get(string).is_some()
    }

    /// Register a new instance of a function.
    pub fn register_function(
        &mut self,
        symbol: Symbol,
        signature: FunctionSignature,
    ) -> FunctionIdx {
        self.function_manager.register(symbol, signature)
    }

    /// Get the [`FunctionIdx`] associated with a [`Symbol`], if one exists.
    pub fn get_function_idx(&mut self, symbol: Symbol) -> Option<FunctionIdx> {
        self.function_manager.get_idx(symbol)
    }

    /// Get a function registration for a given [`FunctionIdx`].
    pub fn get_function(&self, idx: FunctionIdx) -> Option<&FunctionRegistration> {
        self.function_manager.get_function(idx)
    }

    /// Get a mutable function registration for a given [`FunctionIdx`].
    pub fn get_function_mut(&mut self, idx: FunctionIdx) -> Option<&mut FunctionRegistration> {
        self.function_manager.get_function_mut(idx)
    }

    pub fn get_function_symbol(&self, idx: FunctionIdx) -> Option<Symbol> {
        self.function_manager.get_function_symbol(idx)
    }
}
