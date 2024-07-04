use crate::core::symbol::Symbol;

use super::Value;

pub use self::ops::*;

mod ops;

/// Each possible operation of the IR. The results of these operations (if applicable) can be
/// referenced using the ID of the triple.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Triple {
    /// Standard binary operation.
    BinaryOp {
        lhs: Value,
        rhs: Value,
        op: BinaryOp,
    },
    /// Standard unary operation.
    UnaryOp { rhs: Value, op: UnaryOp },
    /// Copy the provided value.
    Copy(Value),
    /// Jump to the corresponding basic block.
    Jump(usize),
    /// Call the corresponding function.
    Call(Symbol),
    /// Return with the provided value.
    Return(Value),
    /// Assign some symbol to some value.
    Assign(Symbol, Value),
    Switch {
        value: Value,
        default: (usize, Value),
        branches: Vec<(Value, usize, Value)>,
    },
}

/// A reference to a specific triple.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TripleRef {
    pub basic_block: usize,
    pub triple: usize,
}

impl TripleRef {
    pub fn new(basic_block: usize, triple: usize) -> Self {
        Self {
            basic_block,
            triple,
        }
    }
}
