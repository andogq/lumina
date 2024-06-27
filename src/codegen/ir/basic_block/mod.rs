mod statement;
mod terminator;

pub use statement::*;
pub use terminator::*;

use crate::util::index::Index;

use super::{value::RValue, Context};

#[derive(Clone)]
pub struct BasicBlockData {
    pub statements: Vec<Statement>,
    pub terminator: Terminator,
}

pub type BasicBlock = Index<BasicBlockData>;

pub struct BasicBlockBuilder {
    ctx: Context,
    statements: Vec<Statement>,
}

/// The value that is the result of some statement
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct StatementValue(pub usize);

impl BasicBlockBuilder {
    pub fn new(ctx: Context) -> Self {
        Self {
            ctx,
            statements: Vec::new(),
        }
    }
    pub fn statement(&mut self, statement: Statement) -> StatementValue {
        let value = StatementValue(self.statements.len());
        self.statements.push(statement);
        value
    }

    /// Terminate this basic block with a `return` statement. This will replace the underlying
    /// basic block builder with a new empty one.
    pub fn t_return(&mut self, value: RValue) -> BasicBlock {
        self.ctx.0.borrow_mut().basic_blocks.push(BasicBlockData {
            statements: std::mem::take(&mut self.statements),
            terminator: Terminator::Return(value),
        })
    }
}
