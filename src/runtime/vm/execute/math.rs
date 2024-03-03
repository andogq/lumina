use crate::runtime::{
    object::{BooleanObject, IntegerObject, Object},
    vm::VM,
};

pub enum MathIntegerOperation {
    Add,
    Sub,
    Mul,
    Div,
}

impl VM {
    fn get_integer_pair(&mut self) -> Result<(i64, i64), String> {
        // Right integer is pushed after left integer
        let right = self.stack.pop_integer()?;
        let left = self.stack.pop_integer()?;

        Ok((left, right))
    }

    pub(super) fn math_integer_operation(
        &mut self,
        op: MathIntegerOperation,
    ) -> Result<(), String> {
        let (left, right) = self.get_integer_pair()?;

        self.stack.push(Object::integer(match op {
            MathIntegerOperation::Add => left + right,
            MathIntegerOperation::Sub => left - right,
            MathIntegerOperation::Mul => left * right,
            MathIntegerOperation::Div => left / right,
        }))?;

        Ok(())
    }

    pub(super) fn math_equality_operation(&mut self, equal: bool) -> Result<(), String> {
        // Pop two items off the stack, they should be the same type
        let right = self.stack.pop()?;
        let left = self.stack.pop()?;

        self.stack.push(Object::boolean(match (left, right) {
            (
                Object::Boolean(BooleanObject { value: left }),
                Object::Boolean(BooleanObject { value: right }),
            ) => (left == right) == equal,
            (
                Object::Integer(IntegerObject { value: left }),
                Object::Integer(IntegerObject { value: right }),
            ) => (left == right) == equal,
            // Any other combination of operands cannot be equal, since they're different types
            _ => false,
        }))?;

        Ok(())
    }

    pub(super) fn math_greater_than(&mut self) -> Result<(), String> {
        let (left, right) = self.get_integer_pair()?;

        self.stack.push(Object::boolean(left > right))?;

        Ok(())
    }

    pub(super) fn math_negate(&mut self) -> Result<(), String> {
        let result = -self.stack.pop_integer()?;

        self.stack.push(Object::integer(result))?;

        Ok(())
    }

    pub(super) fn math_invert(&mut self) -> Result<(), String> {
        let value = match self.stack.pop()? {
            Object::Boolean(BooleanObject { value }) => value,
            Object::Null(_) => false,
            _ => return Err("expected boolean or null".to_string()),
        };

        self.stack.push(Object::boolean(!value))?;

        Ok(())
    }
}
