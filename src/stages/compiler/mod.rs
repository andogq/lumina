mod expression;
mod statement;

use crate::{
    code::{Bytecode, Instruction},
    core::ast::Program,
    runtime::object::Object,
};

#[derive(Default)]
pub struct Compiler {
    instructions: Vec<u8>,
    constants: Vec<Object>,
}

impl Compiler {
    fn register_constant(&mut self, c: Object) -> u32 {
        let id = self.constants.len() as u32;
        self.constants.push(c);
        id
    }

    fn push(&mut self, instruction: Instruction) {
        self.instructions.extend_from_slice(&instruction.encode());
    }

    fn replace(&mut self, offset: usize, instruction: Instruction) {
        let encoded = instruction.encode();
        self.instructions[offset..offset + encoded.len()].clone_from_slice(&encoded);
    }

    /// Consume this compiler instance, producing bytecode.
    pub fn compile(program: Program) -> Result<Bytecode, String> {
        Ok(program
            .statements
            .into_iter()
            .try_fold(Compiler::default(), |mut compiler, statement| {
                compiler.compile_statement(statement, false)?;

                Ok::<_, String>(compiler)
            })?
            .into())
    }
}

impl Into<Bytecode> for Compiler {
    fn into(self) -> Bytecode {
        Bytecode {
            instructions: self.instructions,
            constants: self.constants,
        }
    }
}
