use std::collections::HashSet;

use index_vec::IndexVec;

use crate::{ctx::Symbol, repr::ast::typed as ast, stage::type_check::FunctionSignature};

use super::BasicBlock;

index_vec::define_index_type! {
    pub struct BasicBlockIdx = usize;
}

#[derive(Debug, Clone)]
pub struct Function {
    pub symbol: Symbol,
    pub signature: FunctionSignature,
    pub basic_blocks: IndexVec<BasicBlockIdx, BasicBlock>,
    pub scope: HashSet<Symbol>,
}

impl Function {
    pub fn new(function: &ast::Function) -> Self {
        Self {
            symbol: function.name,
            signature: FunctionSignature::from(function),
            basic_blocks: IndexVec::new(),
            scope: HashSet::new(),
        }
    }
}
