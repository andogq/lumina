use std::fmt::{Display, Formatter};

use crate::{ast::Statement, object::Object};

use super::{AstNode, Return};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl AstNode for Program {
    fn evaluate(&self) -> Return<Object> {
        let mut result = None;

        for statement in &self.statements {
            result = Some(statement.evaluate());
        }

        // TODO: Dunno what to do here
        result.unwrap()
    }
}

impl Display for Program {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.statements.iter().map(|s| write!(f, "{s}\n")).collect()
    }
}
