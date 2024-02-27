use crate::{code::Instruction, compiler::Bytecode, object::Object};

#[derive(Default)]
pub struct VM {
    constants: Vec<Object>,
    instructions: Vec<u8>,

    stack: Stack,
}

pub struct Stack {
    stack: [Option<Object>; 1024],
    pointer: usize,
}

const ARRAY_REPEAT_VALUE: Option<Object> = None;

impl Default for Stack {
    fn default() -> Self {
        Self {
            stack: [ARRAY_REPEAT_VALUE; 1024],
            pointer: 0,
        }
    }
}

impl Stack {
    pub fn push(&mut self, object: Object) -> Result<(), String> {
        if self.pointer < self.stack.len() {
            self.stack[self.pointer] = Some(object);
            self.pointer += 1;
            Ok(())
        } else {
            Err("stack overflow".to_string())
        }
    }

    pub fn pop(&mut self) -> Result<Object, String> {
        if self.pointer > 0 {
            self.pointer -= 1;
            std::mem::replace(&mut self.stack[self.pointer], None)
                .ok_or_else(|| "value not found on stack".to_string())
        } else {
            Err("stack underflow(?)".to_string())
        }
    }
}

impl VM {
    pub fn new(bytecode: Bytecode) -> Self {
        Self {
            constants: bytecode.constants,
            instructions: bytecode.instructions,

            stack: Stack::default(),
        }
    }

    pub fn run(&mut self) -> Result<(), String> {
        let mut i = 0;

        while i < self.instructions.len() {
            // Decode and run
            Instruction::decode(|| {
                // WARN: Should have bounds check
                let b = self.instructions[i];
                i += 1;
                b
            })?
            .run(&mut self.stack, &self.constants)?;
        }

        Ok(())
    }

    pub fn stack_top(&self) -> Option<&Object> {
        self.stack.stack[self.stack.pointer - 1].as_ref()
    }
}

#[cfg(test)]
mod test {
    use crate::{code::Instruction, object::IntegerObject};

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
        let vm = run_vm(
            &[Instruction::Constant(0)],
            &[Object::Integer(IntegerObject { value: 1234 })],
        )
        .unwrap();

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
            &[
                Object::Integer(IntegerObject { value: 5 }),
                Object::Integer(IntegerObject { value: 4 }),
            ],
        )
        .unwrap();

        assert!(matches!(
            vm.stack_top(),
            Some(Object::Integer(IntegerObject { value: 9 }))
        ));
    }
}
