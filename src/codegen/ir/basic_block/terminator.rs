use crate::codegen::ir::value::RValue;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Terminator {
    /// Return from the function.
    Return(RValue),
}
