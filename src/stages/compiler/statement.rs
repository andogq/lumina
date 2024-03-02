use crate::{
    ast::{Expression, LetStatement, ReturnStatement, Statement},
    code::Instruction,
};

use super::Compiler;

impl Compiler {
    pub(super) fn compile_statement(&mut self, statement: Statement) -> Result<(), String> {
        match statement {
            Statement::Let(statement) => self.compile_let(statement),
            Statement::Return(statement) => self.compile_return(statement),
            Statement::Expression(statement) => self.compile_expression_statement(statement),
        }
    }

    pub(super) fn compile_let(&mut self, _statement: LetStatement) -> Result<(), String> {
        todo!()
    }

    pub(super) fn compile_return(&mut self, _statement: ReturnStatement) -> Result<(), String> {
        todo!()
    }

    pub(super) fn compile_expression_statement(
        &mut self,
        statement: Expression,
    ) -> Result<(), String> {
        self.compile_expression(statement)?;

        // End every expression statement with a pop instruction, to prevent the stack from
        // continually growing.
        self.instructions.push(Instruction::Pop);

        Ok(())
    }
}
