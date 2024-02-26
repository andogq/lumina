use std::fmt::{Display, Formatter};

use crate::{
    ast::Statement,
    interpreter::{environment::Environment, return_value::Return},
    object::{NullObject, Object},
    return_value,
};

use super::AstNode;

#[derive(Clone, Debug, PartialEq)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl AstNode for Program {
    fn evaluate(&self, env: Environment) -> Return<Object> {
        let mut result = Object::Null(NullObject);

        for statement in &self.statements {
            result = return_value!(statement.evaluate(env.clone()));
        }

        Return::Implicit(result)
    }

    fn compile(&self, register_constant: impl FnMut(Object) -> u32) -> Result<Vec<u8>, String> {
        todo!()
    }
}

impl Display for Program {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.statements.iter().map(|s| write!(f, "{s}\n")).collect()
    }
}
