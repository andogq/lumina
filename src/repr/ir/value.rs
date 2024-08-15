use super::TripleRef;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConstantValue {
    Integer(i64),
    Boolean(bool),
}

/// Corresponds to the 'address' portion of a three-address code. Intended to transparently
/// represent any possible source of a value.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Value {
    /// Constant value, potentially inserted from the compiler or originating from the source code.
    Constant(ConstantValue),
    /// Temporary value representing the result of some triple.
    Triple(TripleRef),
    /// A parameter passed to the current function.
    Parameter(usize),
    /// No value.
    Unit,
}

impl Value {
    pub fn integer(value: i64) -> Self {
        Self::Constant(ConstantValue::Integer(value))
    }

    pub fn boolean(value: bool) -> Self {
        Self::Constant(ConstantValue::Boolean(value))
    }
}
