use crate::token::{IdentToken, IntToken, LetToken, ReturnToken};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Statement {
    Let(LetStatement),
    Return(ReturnStatement),
    Expression(Expression),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LetStatement {
    pub let_token: LetToken,
    pub name: Identifier,
    pub value: Expression,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReturnStatement {
    pub return_token: ReturnToken,
    pub value: Expression,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Identifier {
    pub ident_token: IdentToken,
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IntegerLiteral {
    pub token: IntToken,
    pub value: i64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expression {
    Identifier(Identifier),
    Integer(IntegerLiteral),
}
