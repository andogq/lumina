mod expression;
mod function;
mod program;
mod statement;

use std::collections::HashMap;

use super::symbol::Symbol;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ty {
    Int,
    Boolean,
    Unit,
}

#[derive(Debug, thiserror::Error)]
pub enum TyError {
    #[error("mismatched types: {0:?} and {1:?}")]
    Mismatch(Ty, Ty),

    #[error("invalid return type, expected {expected:?} but found {found:?}")]
    Return {
        expected: Option<Ty>,
        found: Option<Ty>,
    },

    #[error("symbol not found: {0}")]
    SymbolNotFound(Symbol),
}

trait InferTy {
    fn infer(&self, symbols: &mut HashMap<Symbol, Ty>) -> Result<Ty, TyError>;
    fn return_ty(&self, symbols: &mut HashMap<Symbol, Ty>) -> Result<Option<Ty>, TyError>;
}
