use crate::token::{IdentToken, LetToken};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Statement {
    Let(LetStatement),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LetStatement {
    pub let_token: LetToken,
    pub name: Identifier,
    pub value: Expression,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Identifier {
    pub ident_token: IdentToken,
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Expression;
