mod jump;
mod math;
mod stack;

use crate::code::Instruction;

use self::math::MathIntegerOperation;

use super::VM;

impl VM {
    pub(super) fn execute(&mut self, instruction: Instruction) -> Result<(), String> {
        match instruction {
            // Stack operations
            Instruction::Constant(offset) => self.stack_push_constant(offset)?,
            Instruction::Pop => self.stack_pop()?,
            Instruction::True => self.stack_push_boolean(true)?,
            Instruction::False => self.stack_push_boolean(false)?,

            // Math operations
            Instruction::Add => self.math_integer_operation(MathIntegerOperation::Add)?,
            Instruction::Sub => self.math_integer_operation(MathIntegerOperation::Sub)?,
            Instruction::Mul => self.math_integer_operation(MathIntegerOperation::Mul)?,
            Instruction::Div => self.math_integer_operation(MathIntegerOperation::Div)?,

            // Equality
            Instruction::Equal => self.math_equality_operation(true)?,
            Instruction::NotEqual => self.math_equality_operation(false)?,

            // Comparison
            Instruction::GreaterThan => self.math_greater_than()?,

            // Unary operations
            Instruction::Negate => self.math_negate()?,
            Instruction::Bang => self.math_invert()?,

            // Jump instructions
            Instruction::JumpNotTrue(offset) => self.jump_not_true(offset)?,
            Instruction::Jump(offset) => self.jump(offset)?,
        }

        Ok(())
    }
}
