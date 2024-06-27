use crate::codegen::ir::value::{Local, RValue};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BinaryOperation {
    Plus,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Statement {
    Assign(Local, RValue),
    Infix {
        lhs: RValue,
        rhs: RValue,
        op: BinaryOperation,
    },
}
