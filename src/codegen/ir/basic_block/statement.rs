use crate::codegen::ir::value::{Local, RValue};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Statement {
    Assign(Local, RValue),
}
