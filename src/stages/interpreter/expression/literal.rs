use crate::{
    core::ast::{BooleanLiteral, FunctionLiteral, IntegerLiteral, StringLiteral},
    runtime::{
        object::{BooleanObject, FunctionObject, IntegerObject, Object, StringObject},
        Environment,
    },
};

pub fn interpret_boolean(literal: BooleanLiteral) -> Object {
    Object::Boolean(BooleanObject {
        value: literal.value,
    })
}

pub fn interpret_integer(literal: IntegerLiteral) -> Object {
    Object::Integer(IntegerObject {
        value: literal.value,
    })
}

pub fn interpret_string(literal: StringLiteral) -> Object {
    Object::String(StringObject {
        value: literal.value,
    })
}

pub fn interpret_function(env: &Environment, literal: FunctionLiteral) -> Object {
    Object::Function(FunctionObject {
        parameters: literal.parameters,
        body: literal.body,

        // Capture the environment
        env: env.clone(),
    })
}
