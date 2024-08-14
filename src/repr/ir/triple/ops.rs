use crate::repr::ast::typed as ast;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BinaryOp {
    Add,
    Sub,
    Eq,
    NotEq,
    And,
    Or,
}

impl From<&ast::InfixOperation> for BinaryOp {
    fn from(value: &ast::InfixOperation) -> Self {
        match value {
            ast::InfixOperation::Plus => Self::Add,
            ast::InfixOperation::Minus => Self::Sub,
            ast::InfixOperation::Eq => Self::Eq,
            ast::InfixOperation::NotEq => Self::NotEq,
            ast::InfixOperation::And => Self::And,
            ast::InfixOperation::Or => Self::Or,
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
