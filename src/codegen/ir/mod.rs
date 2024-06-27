mod basic_block;
mod function;
pub mod lowering;
pub mod value;

use std::{cell::RefCell, rc::Rc};

use crate::{core::symbol::SymbolMap, util::index::IndexVec};

pub use basic_block::*;
pub use function::Function;
pub use value::RETURN_LOCAL;

#[derive(Default)]
pub struct ContextInner {
    pub symbols: SymbolMap,

    /// All of the basic blocks created in this pass.
    pub basic_blocks: IndexVec<BasicBlockData>,
}

/// The context tracks any objects created in this compile pass. It is actually a wrapper type
/// around a shared pointer to the actual data structures.
#[derive(Clone)]
pub struct Context(Rc<RefCell<ContextInner>>);

impl Context {
    /// Create a new instance of the context.
    pub fn new() -> Self {
        Self(Rc::new(RefCell::new(Default::default())))
    }

    /// Utility method to create a new basic block builder.
    fn basic_block(&self) -> BasicBlockBuilder {
        BasicBlockBuilder::new(self.clone())
    }

    pub fn into_inner(self) -> ContextInner {
        self.0.take()
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}
