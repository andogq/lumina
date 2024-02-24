use std::fmt::{Display, Formatter};

use crate::ast::{BlockStatement, Identifier};

use super::environment::Environment;

#[derive(Clone, Debug)]
pub enum Object {
    Integer(IntegerObject),
    Boolean(BooleanObject),
    Null(NullObject),
    Function(FunctionObject),
}

impl Display for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Integer(integer) => integer.fmt(f),
            Object::Boolean(boolean) => boolean.fmt(f),
            Object::Null(null) => null.fmt(f),
            Object::Function(function) => function.fmt(f),
        }
    }
}

#[derive(Clone, Debug)]
pub struct IntegerObject {
    pub value: i64,
}

impl Display for IntegerObject {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}

#[derive(Clone, Debug)]
pub struct BooleanObject {
    pub value: bool,
}

impl Display for BooleanObject {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}

#[derive(Clone, Debug)]
pub struct NullObject;

impl Display for NullObject {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "null")
    }
}

#[derive(Clone, Debug)]
pub struct FunctionObject {
    pub parameters: Vec<Identifier>,
    pub body: BlockStatement,
    pub env: Environment,
}

impl Display for FunctionObject {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "fn({}) {}",
            self.parameters
                .iter()
                .map(|param| param.to_string())
                .collect::<Vec<_>>()
                .join(", "),
            self.body.to_string()
        )
    }
}
