use crate::core::ir::{Local, RValue};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Statement {
    Assign(Local, RValue),
}
