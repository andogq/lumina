use int_enum::IntEnum;

use crate::{
    object::{BooleanObject, IntegerObject, Object},
    vm::Stack,
};

#[derive(Debug, PartialEq, Eq, IntEnum)]
#[repr(u8)]
pub enum Opcode {
    Constant = 0,
    Add = 1,
    Sub = 2,
    Mul = 3,
    Div = 4,
    Pop = 5,
    True = 6,
    False = 7,
    Equal = 8,
    NotEqual = 9,
    GreaterThan = 10,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    Constant(u32),
    Add,
    Sub,
    Mul,
    Div,
    Pop,
    True,
    False,
    Equal,
    NotEqual,
    GreaterThan,
}

impl Instruction {
    /// Encode this instruction
    pub fn encode(&self) -> Vec<u8> {
        match self {
            Instruction::Constant(offset) => {
                let mut bytes = Vec::with_capacity(5);
                bytes.push(Opcode::Constant as u8);
                bytes.extend_from_slice(&offset.to_be_bytes());

                bytes
            }
            Instruction::Add => vec![Opcode::Add as u8],
            Instruction::Sub => vec![Opcode::Sub as u8],
            Instruction::Mul => vec![Opcode::Mul as u8],
            Instruction::Div => vec![Opcode::Div as u8],
            Instruction::Pop => vec![Opcode::Pop as u8],
            Instruction::True => vec![Opcode::True as u8],
            Instruction::False => vec![Opcode::False as u8],
            Instruction::Equal => vec![Opcode::Equal as u8],
            Instruction::NotEqual => vec![Opcode::NotEqual as u8],
            Instruction::GreaterThan => vec![Opcode::GreaterThan as u8],
        }
    }

    pub fn decode(mut next_byte: impl FnMut() -> u8) -> Result<Self, String> {
        match Opcode::try_from(next_byte()).map_err(|opcode| format!("unknown opcode {opcode}"))? {
            Opcode::Constant => Ok(Self::Constant(u32::from_be_bytes([
                next_byte(),
                next_byte(),
                next_byte(),
                next_byte(),
            ]))),
            Opcode::Add => Ok(Self::Add),
            Opcode::Sub => Ok(Self::Sub),
            Opcode::Mul => Ok(Self::Mul),
            Opcode::Div => Ok(Self::Div),
            Opcode::Pop => Ok(Self::Pop),
            Opcode::True => Ok(Self::True),
            Opcode::False => Ok(Self::False),
            Opcode::Equal => Ok(Self::Equal),
            Opcode::NotEqual => Ok(Self::NotEqual),
            Opcode::GreaterThan => Ok(Self::GreaterThan),
        }
    }

    pub fn run(&self, stack: &mut Stack, constants: &[Object]) -> Result<(), String> {
        match self {
            &Instruction::Constant(offset) => {
                stack.push(constants[offset as usize].clone())?;
            }
            instruction @ (Instruction::Add
            | Instruction::Sub
            | Instruction::Mul
            | Instruction::Div) => {
                let Object::Integer(IntegerObject { value: left }) = stack.pop()? else {
                    return Err("expected int on stack".to_string());
                };
                let Object::Integer(IntegerObject { value: right }) = stack.pop()? else {
                    return Err("expected int on stack".to_string());
                };

                let value = match instruction {
                    Instruction::Add => left + right,
                    Instruction::Sub => left - right,
                    Instruction::Mul => left * right,
                    Instruction::Div => left / right,
                    _ => unreachable!(),
                };

                stack.push(Object::Integer(IntegerObject { value }))?;
            }
            Instruction::Pop => {
                stack.pop()?;
            }
            instruction @ (Instruction::True | Instruction::False) => {
                stack.push(Object::Boolean(BooleanObject {
                    value: match instruction {
                        Instruction::True => true,
                        Instruction::False => false,
                        _ => unreachable!(),
                    },
                }))?;
            }
            instruction @ (Instruction::Equal
            | Instruction::NotEqual
            | Instruction::GreaterThan) => {
                let Object::Boolean(BooleanObject { value: left }) = stack.pop()? else {
                    return Err("expected boolean on stack".to_string());
                };
                let Object::Boolean(BooleanObject { value: right }) = stack.pop()? else {
                    return Err("expected boolean on stack".to_string());
                };

                let value = match instruction {
                    Instruction::Equal => left == right,
                    Instruction::NotEqual => left != right,
                    Instruction::GreaterThan => left > right,
                    _ => unreachable!(),
                };

                stack.push(Object::Boolean(BooleanObject { value }))?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn encode_op_constant() {
        assert_eq!(
            Instruction::Constant(65534).encode(),
            [0x00, 0x00, 0x00, 0xff, 0xfe]
        );
    }
}
