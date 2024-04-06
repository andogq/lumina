use crate::core::ast::Statement;

#[derive(Debug)]
pub struct Block {
    pub statements: Vec<Statement>,
}
