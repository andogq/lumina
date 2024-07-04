use index_vec::IndexVec;

use super::{Triple, TripleIdx};

#[derive(Default, Clone, Debug)]
pub struct BasicBlock {
    pub triples: IndexVec<TripleIdx, Triple>,
}
