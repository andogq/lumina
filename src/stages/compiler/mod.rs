mod expression;
mod statement;

use std::collections::HashMap;

use crate::{
    code::{Bytecode, Instruction},
    core::ast::Program,
    runtime::object::Object,
};

#[derive(Default)]
pub struct SymbolTable {
    table: HashMap<String, u16>,
    next_id: u16,
}

impl SymbolTable {
    pub fn resolve(&mut self, ident: &str) -> u16 {
        *self.table.entry(ident.to_string()).or_insert_with(|| {
            let id = self.next_id;
            self.next_id += 1;
            id
        })
    }
}

#[derive(Default)]
pub struct Compiler {
    instructions: Vec<u8>,
    constants: Vec<Object>,
    symbol_table: SymbolTable,
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
