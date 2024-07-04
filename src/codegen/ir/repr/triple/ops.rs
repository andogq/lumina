use crate::core::ast;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BinaryOp {
    Add,
    Sub,
}

impl From<&ast::InfixOperation> for BinaryOp {
    fn from(value: &ast::InfixOperation) -> Self {
        match value {
            ast::InfixOperation::Plus(_) => Self::Add,
        }
    }
}

impl From<ast::InfixOperation> for BinaryOp {
    fn from(value: ast::InfixOperation) -> Self {
        Self::from(&value)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UnaryOp {
    Minus,
    Not,
}
