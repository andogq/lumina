use index_vec::IndexVec;

use super::{terminator::Terminator, Triple, TripleIdx};

#[derive(Clone, Debug)]
pub struct BasicBlock {
    pub triples: IndexVec<TripleIdx, Triple>,
    pub terminator: Terminator,
}
