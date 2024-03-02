mod expression;
pub mod runtime;
mod statement;

use crate::{
    core::ast::Program,
    return_value,
    runtime::object::{NullObject, Object},
    runtime::Environment,
};

use self::{runtime::return_value::Return, statement::interpret_statement};

pub fn interpret(program: Program) -> Return<Object> {
    let mut env = Environment::new();
    let mut result = Object::Null(NullObject);

    for statement in program.statements {
        result = return_value!(interpret_statement(&mut env, statement));
    }

    Return::Implicit(result)
}
