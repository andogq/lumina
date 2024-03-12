use crate::{
    code::Instruction,
    core::ast::{BlockStatement, Expression, LetStatement, ReturnStatement, Statement},
};

use super::Compiler;

impl Compiler {
    pub(super) fn compile_statement(
        &mut self,
        statement: Statement,
        skip_pop: bool,
    ) -> Result<(), String> {
        match statement {
            Statement::Let(statement) => self.compile_let(statement),
            Statement::Return(statement) => self.compile_return(statement),
            Statement::Expression { expression, .. } => {
                self.compile_expression_statement(expression, skip_pop)
            }
        }
    }

    pub(super) fn compile_let(&mut self, statement: LetStatement) -> Result<(), String> {
        // Compile the RHS
        self.compile_expression(statement.value)?;

        // Resolve the ident
        let ident = self.symbol_table.resolve(&statement.name.value);

        // Save the resulting value on the stack
        self.push(Instruction::SetGlobal(ident));

        Ok(())
    }

    pub(super) fn compile_return(&mut self, _statement: ReturnStatement) -> Result<(), String> {
        todo!()
    }

    pub(super) fn compile_expression_statement(
        &mut self,
        statement: Expression,
        skip_pop: bool,
    ) -> Result<(), String> {
        self.compile_expression(statement)?;

        if !skip_pop {
            // End every expression statement with a pop instruction, to prevent the stack from
            // continually growing.
            self.push(Instruction::Pop);
        }

        Ok(())
    }

    pub(super) fn compile_block_statement(&mut self, block: BlockStatement) -> Result<(), String> {
        let last_statment = block.statements.len() - 1;

        block
            .statements
            .into_iter()
            .enumerate()
            // Leave the last statement value on the stack, so it can be used in expressions
            .map(|(i, statement)| self.compile_statement(statement, i == last_statment))
            .collect()
    }
}
