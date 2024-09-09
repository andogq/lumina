use crate::compiler::Symbol;

use super::*;

#[derive(Debug, thiserror::Error)]
pub enum TyError {
    #[error("mismatched types: {0:?} and {1:?}")]
    Mismatch(Ty, Ty),

    #[error("invalid return type, expected {expected:?} but found {found:?}")]
    Return {
        expected: Option<Ty>,
        found: Option<Ty>,
    },

    #[error("cannot cast {0:?} to {1:?}")]
    Cast(Ty, Ty),

    #[error("cannot perform index on {0:?}")]
    Index(Ty),

    #[error("symbol not found: {0:?}")]
    SymbolNotFound(Symbol),
}
