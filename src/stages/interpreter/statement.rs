use crate::{
    core::ast::{BlockStatement, LetStatement, ReturnStatement, Statement},
    return_value,
    runtime::{
        object::{NullObject, Object},
        Environment,
    },
};

use super::{expression::interpret_expression, runtime::return_value::Return};

pub fn interpret_statement(env: &mut Environment, statement: Statement) -> Return<Object> {
    match statement {
        Statement::Let(let_statement) => interpret_let_statement(env, let_statement),
        Statement::Return(return_statement) => interpret_return_statement(env, return_statement),
        Statement::Expression(expression_statement) => {
            interpret_expression(env, expression_statement)
        }
    }
}

pub fn interpret_block_statement(env: &mut Environment, block: BlockStatement) -> Return<Object> {
    let mut result = Object::Null(NullObject);

    for statement in block.statements {
        result = return_value!(interpret_statement(env, statement));
    }

    Return::Implicit(result)
}

pub fn interpret_let_statement(
    env: &mut Environment,
    let_statement: LetStatement,
) -> Return<Object> {
    let result = return_value!(interpret_expression(env, let_statement.value));
    env.set(&let_statement.name.value, result);

    Return::Implicit(Object::Null(NullObject))
}

pub fn interpret_return_statement(
    env: &mut Environment,
    return_statement: ReturnStatement,
) -> Return<Object> {
    match interpret_expression(env, return_statement.value) {
        Return::Explicit(value) | Return::Implicit(value) => Return::Explicit(value),
        Return::Error(err) => Return::Error(err),
    }
}
