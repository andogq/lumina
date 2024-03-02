mod expression;
mod statement;

use crate::{
    ast::Program,
    code::{Bytecode, Instruction},
    runtime::object::Object,
};

#[derive(Default)]
pub struct Compiler {
    instructions: Vec<Instruction>,
    constants: Vec<Object>,
}

impl Compiler {
    fn register_constant(&mut self, c: Object) -> u32 {
        let id = self.constants.len() as u32;
        self.constants.push(c);
        id
    }

    /// Consume this compiler instance, producing bytecode.
    pub fn compile(program: Program) -> Result<Bytecode, String> {
        Ok(program
            .statements
            .into_iter()
            .try_fold(Compiler::default(), |mut compiler, statement| {
                compiler.compile_statement(statement)?;

                Ok::<_, String>(compiler)
            })?
            .into())
    }
}

impl Into<Bytecode> for Compiler {
    fn into(self) -> Bytecode {
        Bytecode {
            instructions: self
                .instructions
                .into_iter()
                .map(|i| i.encode())
                .flatten()
                .collect(),
            constants: self.constants,
        }
    }
}
