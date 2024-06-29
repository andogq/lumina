use crate::core::symbol::Symbol;

use super::TripleRef;

/// Corresponds to the 'address' portion of a three-address code. Intended to transparently
/// represent any possible source of a value.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Value {
    /// Value derived from a name in the source code.
    Name(Symbol),
    /// Constant value, potentially inserted from the compiler or originating from the source code.
    Constant(i64),
    /// Temporary value representing the result of some triple.
    Triple(TripleRef),
}
