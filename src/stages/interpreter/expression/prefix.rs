use crate::{
    core::ast::{PrefixExpression, PrefixToken},
    return_value,
    runtime::{object::Object, Environment},
    stages::interpreter::runtime::{error::Error, return_value::Return},
};

use super::interpret_expression;

pub fn interpret_prefix(env: &mut Environment, prefix: PrefixExpression) -> Return<Object> {
    let right = return_value!(interpret_expression(env, *prefix.right));

    match (&prefix.prefix_token, right) {
        (PrefixToken::Plus(_), Object::Integer(int)) => {
            Return::Implicit(Object::integer(int.value))
        }
        (PrefixToken::Minus(_), Object::Integer(int)) => {
            Return::Implicit(Object::integer(-int.value))
        }
        (PrefixToken::Bang(_), Object::Boolean(bool)) => {
            Return::Implicit(Object::boolean(!bool.value))
        }
        (PrefixToken::Bang(_), Object::Null(_)) => Return::Implicit(Object::boolean(true)),
        _ => Error::throw("prefix operation not supported"),
    }
}
