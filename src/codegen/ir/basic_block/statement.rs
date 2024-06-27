use crate::codegen::ir::value::{Local, RValue};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BinaryOperation {
    Plus,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Statement {
    Assign(Local, RValue),
    Load {
        result: Local,
        target: Local,
    },
    Infix {
        lhs: Local,
        rhs: Local,
        op: BinaryOperation,
        target: Local,
    },
}
