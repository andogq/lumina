use crate::{code::decode_instruction, object::Object};

#[derive(Default)]
pub struct VM {
    constants: Vec<Object>,
    instructions: Vec<u8>,

    stack: Stack,
}

#[derive(Default)]
pub struct Stack {
    stack: Vec<Object>,
}

// TODO: Should do bounds checks/proper error handling in struct
impl Stack {
    pub fn push(&mut self, object: Object) {
        self.stack.push(object);
    }

    pub fn pop(&mut self) -> Object {
        self.stack.pop().unwrap()
    }
}

impl VM {
    pub fn run(&mut self) -> Result<(), String> {
        let mut i = 0;

        while i < self.instructions.len() {
            let mut get_byte = || {
                // WARN: Should have bounds check
                let b = self.instructions[i];
                i += 1;
                b
            };

            let opcode = get_byte();

            // Attempt to decode opcode
            let instruction = decode_instruction(opcode, get_byte)?;

            instruction.run(&mut self.stack, &self.constants);
        }

        Ok(())
    }

    pub fn stack_top(&self) -> Option<&Object> {
        self.stack.stack.last()
    }
}

#[cfg(test)]
mod test {
    use crate::{
        code::{Instruction, OpConstant},
        object::IntegerObject,
    };

    use super::*;

    #[test]
    fn int_constant() {
        let mut vm = VM::default();

        // Setup the machine
        vm.instructions = OpConstant(0).bytes();
        vm.constants = vec![Object::Integer(IntegerObject { value: 1234 })];

        // Run the machine
        let _ = vm.run();

        assert!(matches!(
            vm.stack_top(),
            Some(Object::Integer(IntegerObject { value: 1234 }))
        ));
    }
}
