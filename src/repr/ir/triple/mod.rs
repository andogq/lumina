use index_vec::define_index_type;

use crate::repr::identifier::{FunctionIdx, ScopedBinding};

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
    /// Call the corresponding function.
    Call(FunctionIdx, Vec<Value>),
    /// Assign some symbol to some value.
    Assign(ScopedBinding, Value),
    /// Loads a value from a scoped binding.
    Load(ScopedBinding),
    /// Allocate an array of the provded size.
    AllocArray(u32),
    /// Index into the value.
    Index { value: ScopedBinding, index: Value },
    /// TODO: Must be removed
    SetIndex {
        array_ptr: TripleRef,
        index: Value,
        value: Value,
    },
    /// Merge the listed values from their basic blocks into a single value.
    Phi(Vec<(Value, BasicBlockIdx)>),
}

define_index_type! {
    /// Identifier for a triple within some basic block.
    pub struct TripleIdx = usize;
}

/// A reference to a specific triple.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct TripleRef {
    pub basic_block: BasicBlockIdx,
    pub triple: TripleIdx,
}

impl TripleRef {
    pub fn new(basic_block: BasicBlockIdx, triple: TripleIdx) -> Self {
        Self {
            basic_block,
            triple,
        }
    }
}
