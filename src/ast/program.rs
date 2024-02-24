use std::fmt::{Display, Formatter};

use crate::{
    ast::Statement,
    interpreter::{
        object::{NullObject, Object},
        return_value::Return,
    },
    return_value,
};

use super::AstNode;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl AstNode for Program {
    fn evaluate(&self) -> Return<Object> {
        let mut result = Object::Null(NullObject);

        for statement in &self.statements {
            result = return_value!(statement.evaluate());
        }

        Return::Implicit(result)
    }
}

impl Display for Program {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.statements.iter().map(|s| write!(f, "{s}\n")).collect()
    }
}
