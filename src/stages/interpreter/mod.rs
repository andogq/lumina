mod expression;
pub mod runtime;
mod statement;

use crate::{core::ast::Program, return_value, runtime::object::Object, runtime::Environment};

use self::{runtime::return_value::Return, statement::interpret_statement};

pub fn interpret(program: Program) -> Return<Object> {
    interpret_with_env(&mut Environment::new(), program)
}

pub fn interpret_with_env(env: &mut Environment, program: Program) -> Return<Object> {
    let mut result = Object::null();

    for statement in program.statements {
        result = return_value!(interpret_statement(env, statement));
    }

    Return::Implicit(result)
}
