use crate::{
    core::ast::{BooleanLiteral, FunctionLiteral, IntegerLiteral, StringLiteral},
    runtime::{object::Object, Environment},
};

pub fn interpret_boolean(literal: BooleanLiteral) -> Object {
    Object::boolean(literal.value)
}

pub fn interpret_integer(literal: IntegerLiteral) -> Object {
    Object::integer(literal.value)
}

pub fn interpret_string(literal: StringLiteral) -> Object {
    Object::string(literal.value)
}

pub fn interpret_function(env: &Environment, literal: FunctionLiteral) -> Object {
    Object::function(env, literal.parameters, literal.body)
}
