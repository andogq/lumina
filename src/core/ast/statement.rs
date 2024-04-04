use crate::core::symbol::Symbol;

use super::Expression;

pub enum Statement {
    Return(ReturnStatement),
    Let(LetStatement),
    Expression(ExpressionStatement),
}

pub struct ReturnStatement {
    pub value: Expression,
}

pub struct LetStatement {
    pub name: Symbol,
    pub value: Expression,
}

pub struct ExpressionStatement {
    pub expression: Expression,
}
