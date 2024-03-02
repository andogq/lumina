use crate::{
    core::ast::{CallExpression, CallableFunction, Expression, Identifier, IfExpression},
    return_value,
    runtime::{
        object::{BooleanObject, Object},
        Environment,
    },
};

use self::{
    infix::interpret_infix,
    literal::{interpret_boolean, interpret_function, interpret_integer, interpret_string},
    prefix::interpret_prefix,
};

use super::{
    runtime::{error::Error, return_value::Return},
    statement::interpret_block_statement,
};

mod infix;
mod literal;
mod prefix;

pub fn interpret_expression(env: &mut Environment, expression: Expression) -> Return<Object> {
    match expression {
        Expression::Identifier(identifier) => interpret_identifier(env, identifier),
        Expression::Integer(integer) => Return::Implicit(interpret_integer(integer)),
        Expression::Boolean(boolean) => Return::Implicit(interpret_boolean(boolean)),
        Expression::String(string) => Return::Implicit(interpret_string(string)),
        Expression::Prefix(prefix) => interpret_prefix(env, prefix),
        Expression::Infix(infix) => interpret_infix(env, infix),
        Expression::If(if_expression) => interpret_if_expression(env, *if_expression),
        Expression::Function(function) => Return::Implicit(interpret_function(env, function)),
        Expression::Call(call) => interpret_call(env, call),
    }
}

pub fn interpret_identifier(env: &Environment, identifier: Identifier) -> Return<Object> {
    env.get(&identifier.value)
        .map(|value| Return::Implicit(value))
        .unwrap_or_else(|| Error::throw(format!("identifier not found: \"{}\"", identifier.value)))
}

pub fn interpret_call(env: &mut Environment, call: CallExpression) -> Return<Object> {
    let Object::Function(function) = (match call.function {
        CallableFunction::Identifier(ident) => return_value!(interpret_identifier(env, ident)),
        CallableFunction::FunctionLiteral(lit) => interpret_function(env, lit),
    }) else {
        return Error::throw("value is not of type function");
    };

    let mut function_env = env.nest();

    // Evaluate all arguments and set them in the environment
    for (arg, param) in call
        .arguments
        .into_iter()
        .map(|arg| interpret_expression(env, arg))
        .zip(function.parameters)
    {
        match arg {
            Return::Implicit(value) | Return::Explicit(value) => {
                function_env.set(param.value, value)
            }
            Return::Error(err) => return Return::Error(err),
        }
    }

    match interpret_block_statement(&mut function_env, function.body) {
        Return::Explicit(value) | Return::Implicit(value) => Return::Implicit(value),
        Return::Error(err) => Return::Error(err),
    }
}

pub fn interpret_if_expression(
    env: &mut Environment,
    if_expression: IfExpression,
) -> Return<Object> {
    let condition = return_value!(interpret_expression(env, if_expression.condition));

    match (condition, if_expression.else_branch) {
        (Object::Boolean(BooleanObject { value: true }), _) => {
            interpret_block_statement(env, if_expression.consequence)
        }
        (Object::Boolean(BooleanObject { value: false }), Some(alternative)) => {
            interpret_block_statement(env, alternative.statement)
        }
        _ => Return::Implicit(Object::null()),
    }
}
