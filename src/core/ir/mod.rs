mod basic_block;
mod function;
mod index;
mod lowering;
mod value;

use std::{cell::RefCell, rc::Rc};

use self::{basic_block::*, function::*, index::*, value::*};

use super::ast;

#[derive(Default)]
struct ContextInner {
    /// All of the basic blocks created in this pass.
    basic_blocks: IndexVec<BasicBlockData>,
}

/// The context tracks any objects created in this compile pass. It is actually a wrapper type
/// around a shared pointer to the actual data structures.
#[derive(Clone)]
struct Context(Rc<RefCell<ContextInner>>);

impl Context {
    /// Create a new instance of the context.
    pub fn new() -> Self {
        Self(Rc::new(RefCell::new(Default::default())))
    }

    /// Utility method to create a new basic block builder.
    fn basic_block(&self) -> BasicBlockBuilder {
        BasicBlockBuilder::new(self.clone())
    }
}
