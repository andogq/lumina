mod infix;
mod literal;
mod prefix;

use crate::{
    code::Instruction,
    core::ast::{Expression, IfExpression},
};

use super::Compiler;

impl Compiler {
    pub(super) fn compile_expression(&mut self, e: Expression) -> Result<(), String> {
        match e {
            Expression::Identifier(ident) => self.compile_ident(ident),
            Expression::Integer(integer) => self.compile_integer(integer),
            Expression::String(_) => todo!(),
            Expression::Boolean(boolean) => self.compile_boolean(boolean),
            Expression::Prefix(prefix) => self.compile_prefix(prefix),
            Expression::Infix(infix) => self.compile_infix(infix),
            Expression::If(if_statement) => self.compile_if_expression(*if_statement),
            Expression::Block(_) => todo!(),
            Expression::Function(_) => todo!(),
            Expression::Call(_) => todo!(),
        }
    }

    pub(super) fn compile_if_expression(&mut self, e: IfExpression) -> Result<(), String> {
        // Compile the condition
        self.compile_expression(e.condition)?;

        // Add a placehodler jump instruction to alternate branch
        let consequence_jump_instruction = self.instructions.len();
        self.push(Instruction::JumpNotTrue(0));

        // Compile the consequence
        let consequence_start = self.instructions.len();
        self.compile_block_statement(e.consequence)?;

        let consequence_end = if let Some(else_branch) = e.else_branch {
            // Add an additional instruction to skip past the else body
            let alternate_jump_instruction = self.instructions.len();
            self.push(Instruction::Jump(0));

            // Shift consequence end to include the jump
            let consequence_end = self.instructions.len();

            self.compile_block_statement(else_branch.statement)?;
            let else_end = self.instructions.len();

            // Calculate instruction offset to skip else block
            let offset = else_end - consequence_end;
            self.replace(alternate_jump_instruction, Instruction::Jump(offset as i16));

            consequence_end
        } else {
            // Always jump 1 byte, to skip over the null instruction
            self.push(Instruction::Jump(1));

            let consequence_end = self.instructions.len();

            self.push(Instruction::Null);

            consequence_end
        };

        // Calculate the number of instructions to jump past
        let offset = consequence_end - consequence_start;
        self.replace(
            consequence_jump_instruction,
            Instruction::JumpNotTrue(offset as i16),
        );

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::core::{
        ast::{Block, BooleanLiteral, ElseBranch, IntegerLiteral, Statement},
        lexer::{ElseToken, IfToken},
    };

    use super::*;

    #[test]
    fn simple_if() {
        let mut compiler = Compiler::default();
        compiler
            .compile_if_expression(IfExpression {
                if_token: IfToken::default(),
                condition: Expression::Boolean(BooleanLiteral::new(true)),
                consequence: Block {
                    statements: vec![Statement::Expression {
                        expression: Expression::Integer(IntegerLiteral::new(10)),
                        semicolon: true,
                    }],
                },
                else_branch: None,
            })
            .unwrap();

        assert_eq!(
            compiler.instructions,
            [
                Instruction::True.encode(),
                Instruction::JumpNotTrue(8).encode(),
                Instruction::Constant(0).encode(),
                Instruction::Jump(1).encode(),
                Instruction::Null.encode(),
            ]
            .concat()
        );
    }

    #[test]
    fn simple_if_else() {
        let mut compiler = Compiler::default();
        compiler
            .compile_if_expression(IfExpression {
                if_token: IfToken::default(),
                condition: Expression::Boolean(BooleanLiteral::new(true)),
                consequence: Block {
                    statements: vec![Statement::Expression {
                        expression: Expression::Integer(IntegerLiteral::new(10)),
                        semicolon: true,
                    }],
                },
                else_branch: Some(ElseBranch {
                    else_token: ElseToken::default(),
                    statement: Block {
                        statements: vec![Statement::Expression {
                            expression: Expression::Integer(IntegerLiteral::new(99)),
                            semicolon: true,
                        }],
                    },
                }),
            })
            .unwrap();

        assert_eq!(
            compiler.instructions,
            [
                Instruction::True.encode(),
                Instruction::JumpNotTrue(8).encode(),
                Instruction::Constant(0).encode(),
                Instruction::Jump(5).encode(),
                Instruction::Constant(1).encode(),
            ]
            .concat()
        );
    }
}
