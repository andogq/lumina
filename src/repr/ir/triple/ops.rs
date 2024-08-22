use crate::repr::ast::typed as ast;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BinaryOp {
    Add,
    Sub,
    Eq,
    NotEq,
    Greater,
    Less,
    GreaterEq,
    LessEq,
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
            ast::InfixOperation::Greater => Self::Greater,
            ast::InfixOperation::Less => Self::Less,
            ast::InfixOperation::GreaterEq => Self::GreaterEq,
            ast::InfixOperation::LessEq => Self::LessEq,
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
