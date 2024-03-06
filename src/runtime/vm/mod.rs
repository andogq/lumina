mod execute;
mod stack;

use std::collections::HashMap;

use crate::{
    code::{Bytecode, Instruction},
    runtime::object::Object,
};

use self::stack::Stack;

#[derive(Default)]
pub struct VM {
    constants: Vec<Object>,
    instructions: Vec<u8>,

    pc: usize,

    stack: Stack,

    symbols: HashMap<u16, Object>,
}

impl VM {
    pub fn new(bytecode: Bytecode) -> Self {
        Self {
            constants: bytecode.constants,
            instructions: bytecode.instructions,

            pc: 0,

            stack: Stack::default(),

            symbols: HashMap::new(),
        }
    }

    pub fn run(&mut self) -> Result<(), String> {
        while self.pc < self.instructions.len() {
            let instruction = Instruction::decode(|| {
                // WARN: Should have bounds check
                let b = self.instructions[self.pc];
                self.pc += 1;
                b
            })?;

            self.execute(instruction)?;
        }

        Ok(())
    }

    pub fn stack_top(&self) -> Option<&Object> {
        self.stack.top()
    }

    pub fn last_pop(&self) -> Option<&Object> {
        self.stack.last_popped.as_ref()
    }
}

#[cfg(test)]
mod test {
    use crate::{code::Instruction, runtime::object::IntegerObject};

    use super::*;

    fn run_vm(instructions: &[Instruction], constants: &[Object]) -> Result<VM, String> {
        let mut vm = VM::default();

        vm.instructions = instructions
            .into_iter()
            .map(|i| i.encode())
            .flatten()
            .collect();
        vm.constants = Vec::from_iter(constants.into_iter().cloned());

        vm.run()?;

        Ok(vm)
    }

    #[test]
    fn int_constant() {
        let vm = run_vm(&[Instruction::Constant(0)], &[Object::integer(1234)]).unwrap();

        assert!(matches!(
            vm.stack_top(),
            Some(Object::Integer(IntegerObject { value: 1234 }))
        ));
    }

    #[test]
    fn add_ints() {
        let vm = run_vm(
            &[
                Instruction::Constant(0),
                Instruction::Constant(1),
                Instruction::Add,
            ],
            &[Object::integer(5), Object::integer(4)],
        )
        .unwrap();

        assert!(matches!(
            vm.stack_top(),
            Some(Object::Integer(IntegerObject { value: 9 }))
        ));
    }
}
