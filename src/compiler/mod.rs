mod function_manager;

use string_interner::{DefaultBackend, DefaultSymbol, StringInterner};

use self::function_manager::*;

use crate::{
    hir::SolveType,
    repr::ir,
    stage::{self, parse::ParseError},
    ty::TyError,
};

/// A symbol represents an interned string.
pub type Symbol = DefaultSymbol;

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
    pub functions: FunctionManager,
}

impl Compiler {
    /// From the provided source, compile and return the IR of each of the functions.
    pub fn compile(&mut self, source: impl AsRef<str>) -> Result<Vec<ir::Function>, CompilerError> {
        // Parse the source
        let program = stage::parse::parse(self, source.as_ref())?;

        // Perform type checking
        let program = program.solve(self, &mut ())?;

        // Lower into IR
        let ir = stage::lower_ir::lower(self, program);

        Ok(ir)
    }
}
