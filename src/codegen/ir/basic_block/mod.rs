mod statement;
mod terminator;

pub use statement::*;
pub use terminator::*;

use super::{Context, Index};

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

impl BasicBlockBuilder {
    pub fn new(ctx: Context) -> Self {
        Self {
            ctx,
            statements: Vec::new(),
        }
    }
    pub fn statement(mut self, statement: Statement) -> Self {
        self.statements.push(statement);
        self
    }

    pub fn t_return(self) -> BasicBlock {
        self.ctx.0.borrow_mut().basic_blocks.push(BasicBlockData {
            statements: self.statements,
            terminator: Terminator::Return,
        })
    }
}
