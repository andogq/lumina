use super::*;

#[derive(Clone, Debug)]
pub enum Terminator {
    /// Jump to the corresponding basic block.
    Jump(BasicBlockIdx),
    /// Return with the provided value.
    Return(Value),
    Switch {
        value: Value,
        default: BasicBlockIdx,
        branches: Vec<(Value, BasicBlockIdx)>,
    },
}
