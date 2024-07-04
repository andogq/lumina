use crate::core::symbol::Symbol;

use super::{BasicBlockIdx, Value};

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
    Jump(BasicBlockIdx),
    /// Call the corresponding function.
    Call(Symbol),
    /// Return with the provided value.
    Return(Value),
    /// Assign some symbol to some value.
    Assign(Symbol, Value),
    Switch {
        value: Value,
        default: (BasicBlockIdx, Value),
        branches: Vec<(Value, BasicBlockIdx, Value)>,
    },
}

/// A reference to a specific triple.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TripleRef {
    pub basic_block: BasicBlockIdx,
    pub triple: usize,
}

impl TripleRef {
    pub fn new(basic_block: BasicBlockIdx, triple: usize) -> Self {
        Self {
            basic_block,
            triple,
        }
    }
}
