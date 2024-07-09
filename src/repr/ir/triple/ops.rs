use crate::repr::ast::typed as ast;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BinaryOp {
    Add,
    Sub,
    Eq,
    NotEq,
}

impl From<&ast::InfixOperation> for BinaryOp {
    fn from(value: &ast::InfixOperation) -> Self {
        match value {
            ast::InfixOperation::Plus(_) => Self::Add,
            ast::InfixOperation::Eq(_) => Self::Eq,
            ast::InfixOperation::NotEq(_) => Self::NotEq,
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
