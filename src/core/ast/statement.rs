use crate::core::symbol::Symbol;

use super::Expression;

#[derive(Debug)]
pub enum Statement {
    Return(ReturnStatement),
    Let(LetStatement),
    Expression(ExpressionStatement),
}

#[derive(Debug)]
pub struct ReturnStatement {
    pub value: Expression,
}

#[derive(Debug)]
pub struct LetStatement {
    pub name: Symbol,
    pub value: Expression,
}

#[derive(Debug)]
pub struct ExpressionStatement {
    pub expression: Expression,
    pub implicit_return: bool,
}
