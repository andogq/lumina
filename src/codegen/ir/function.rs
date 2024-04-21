use crate::core::symbol::Symbol;

use super::BasicBlock;

pub struct Function {
    /// Entry point for this function
    pub entry: BasicBlock,

    /// The name of this function
    pub name: Symbol,
}
