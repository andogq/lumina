use crate::{
    ast::{PrefixExpression, PrefixToken},
    return_value,
    runtime::{
        object::{BooleanObject, Object},
        Environment,
    },
    stages::interpreter::runtime::{error::Error, return_value::Return},
};

use super::interpret_expression;

pub fn interpret_prefix(env: &mut Environment, prefix: PrefixExpression) -> Return<Object> {
    let right = return_value!(interpret_expression(env, *prefix.right));

    match (&prefix.prefix_token, right) {
        (PrefixToken::Plus(_), Object::Integer(int)) => Return::Implicit(Object::Integer(int)),
        (PrefixToken::Minus(_), Object::Integer(mut int)) => Return::Implicit(Object::Integer({
            int.value = -int.value;
            int
        })),
        (PrefixToken::Bang(_), Object::Boolean(bool)) => {
            Return::Implicit(Object::Boolean(BooleanObject { value: !bool.value }))
        }
        (PrefixToken::Bang(_), Object::Null(_)) => {
            Return::Implicit(Object::Boolean(BooleanObject { value: true }))
        }
        _ => Error::throw("prefix operation not supported"),
    }
}
