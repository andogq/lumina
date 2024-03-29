use int_enum::IntEnum;

use crate::runtime::object::Object;

pub struct Bytecode {
    pub instructions: Vec<u8>,
    pub constants: Vec<Object>,
}

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
    Negate = 11,
    Bang = 12,
    JumpNotTrue = 13,
    Jump = 14,
    Null = 15,
    GetGlobal = 16,
    SetGlobal = 17,
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
    Negate,
    Bang,
    JumpNotTrue(i16),
    Jump(i16),
    Null,
    GetGlobal(u16),
    SetGlobal(u16),
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
            Instruction::Negate => vec![Opcode::Negate as u8],
            Instruction::Bang => vec![Opcode::Bang as u8],
            Instruction::JumpNotTrue(offset) => {
                let mut bytes = Vec::with_capacity(3);
                bytes.push(Opcode::JumpNotTrue as u8);
                bytes.extend_from_slice(&offset.to_be_bytes());
                bytes
            }
            Instruction::Jump(offset) => {
                let mut bytes = Vec::with_capacity(3);
                bytes.push(Opcode::Jump as u8);
                bytes.extend_from_slice(&offset.to_be_bytes());
                bytes
            }
            Instruction::Null => vec![Opcode::Null as u8],
            Instruction::GetGlobal(identifier) => {
                let mut bytes = Vec::with_capacity(3);
                bytes.push(Opcode::GetGlobal as u8);
                bytes.extend_from_slice(&identifier.to_be_bytes());
                bytes
            }
            Instruction::SetGlobal(identifier) => {
                let mut bytes = Vec::with_capacity(3);
                bytes.push(Opcode::SetGlobal as u8);
                bytes.extend_from_slice(&identifier.to_be_bytes());
                bytes
            }
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
            Opcode::Negate => Ok(Self::Negate),
            Opcode::Bang => Ok(Self::Bang),
            Opcode::JumpNotTrue => Ok(Self::JumpNotTrue(i16::from_be_bytes([
                next_byte(),
                next_byte(),
            ]))),
            Opcode::Jump => Ok(Self::Jump(i16::from_be_bytes([next_byte(), next_byte()]))),
            Opcode::Null => Ok(Self::Null),
            Opcode::GetGlobal => Ok(Self::GetGlobal(u16::from_be_bytes([
                next_byte(),
                next_byte(),
            ]))),
            Opcode::SetGlobal => Ok(Self::SetGlobal(u16::from_be_bytes([
                next_byte(),
                next_byte(),
            ]))),
        }
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
