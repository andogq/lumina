use std::fmt::{Display, Formatter};

use crate::{object::Object, vm::Stack};

#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Opcode {
    Constant,
    Add,
}

impl TryFrom<u8> for Opcode {
    type Error = ();

    fn try_from(opcode: u8) -> Result<Self, Self::Error> {
        use Opcode::*;

        Ok(match opcode {
            0 => Constant,
            1 => Add,
            _ => return Err(()),
        })
    }
}

impl Display for Opcode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use Opcode::*;
        match self {
            Constant => write!(f, "Constant"),
            Add => write!(f, "Other"),
        }
    }
}

pub fn decode_instruction(
    opcode: u8,
    next_byte: impl FnMut() -> u8,
) -> Result<Box<dyn Instruction>, String> {
    use Opcode::*;
    Ok(
        match Opcode::try_from(opcode).map_err(|_| "unknown opcode".to_string())? {
            Constant => Box::new(OpConstant::from_bytes(next_byte)?),
            Add => Box::new(OpAdd),
        },
    )
}

pub trait Instruction {
    /// Convert to bytes, including opcode
    fn bytes(&self) -> Vec<u8>;

    // Parse from bytes, excluding opcode
    fn from_bytes(next_byte: impl FnMut() -> u8) -> Result<Self, String>
    where
        Self: Sized;

    fn run(&self, stack: &mut Stack, constants: &[Object]);
}

pub struct OpAdd;
impl Instruction for OpAdd {
    fn bytes(&self) -> Vec<u8> {
        vec![Opcode::Add as u8]
    }

    fn from_bytes(next_byte: impl FnMut() -> u8) -> Result<Self, String> {
        todo!()
    }

    fn run(&self, stack: &mut Stack, constants: &[Object]) {
        todo!()
    }
}

pub struct OpConstant(pub u32);
impl Instruction for OpConstant {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(5);
        bytes.push(Opcode::Constant as u8);
        bytes.extend_from_slice(&self.0.to_be_bytes());

        bytes
    }

    fn from_bytes(mut next_byte: impl FnMut() -> u8) -> Result<Self, String> {
        Ok(Self(u32::from_be_bytes([
            next_byte(),
            next_byte(),
            next_byte(),
            next_byte(),
        ])))
    }

    fn run(&self, stack: &mut Stack, constants: &[Object]) {
        stack.push(constants[self.0 as usize].clone());
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn encode_op_constant() {
        assert_eq!(OpConstant(65534).bytes(), [0x00, 0xff, 0xff, 0xff, 0xfe]);
    }
}
