use crate::{
    ast::{BooleanLiteral, IntegerLiteral},
    code::Instruction,
    runtime::object::{IntegerObject, Object},
};

use super::Compiler;

impl Compiler {
    pub(super) fn compile_boolean(&mut self, literal: BooleanLiteral) -> Result<(), String> {
        self.instructions.push(match literal.value {
            true => Instruction::True,
            false => Instruction::False,
        });

        Ok(())
    }

    pub(super) fn compile_integer(&mut self, literal: IntegerLiteral) -> Result<(), String> {
        let id = self.register_constant(Object::Integer(IntegerObject {
            value: literal.value,
        }));

        self.instructions.push(Instruction::Constant(id));

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn int_compile() {
        let mut compiler = Compiler::default();
        compiler.compile_integer(IntegerLiteral::new(5)).unwrap();

        assert_eq!(compiler.constants.len(), 1);

        // Make sure the stored constants value is correct
        assert!(matches!(compiler.constants[0], Object::Integer(_)));
        if let Object::Integer(int) = &compiler.constants[0] {
            assert_eq!(int.value, 5);
        }

        assert_eq!(compiler.instructions, [Instruction::Constant(0),]);
    }
}
