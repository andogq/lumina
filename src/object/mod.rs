use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Object {
    Integer(IntegerObject),
    Boolean(BooleanObject),
    Null(NullObject),
}

impl Display for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Integer(integer) => integer.fmt(f),
            Object::Boolean(boolean) => boolean.fmt(f),
            Object::Null(null) => null.fmt(f),
        }
    }
}

#[derive(Debug)]
pub struct IntegerObject {
    pub value: i64,
}

impl Display for IntegerObject {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}

#[derive(Debug)]
pub struct BooleanObject {
    pub value: bool,
}

impl Display for BooleanObject {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}

#[derive(Debug)]
pub struct NullObject;

impl Display for NullObject {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "null")
    }
}
