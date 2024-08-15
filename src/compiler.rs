use string_interner::{DefaultBackend, DefaultSymbol, StringInterner};

use crate::compile_pass::CompilePass;

/// A symbol represents an interned string.
pub type Symbol = DefaultSymbol;

/// Contains all of the state required for a compiler pass.
#[derive(Default, Clone)]
pub struct Compiler {
    /// Symbols interned by the compiler.
    symbols: StringInterner<DefaultBackend>,
}

impl Compiler {
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
}

impl From<Compiler> for CompilePass {
    fn from(compiler: Compiler) -> Self {
        let mut ctx = Self::default();

        ctx.compiler = compiler;

        ctx
    }
}
