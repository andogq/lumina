use crate::{
    ast::{Expression, Identifier},
    token::LetToken,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LetStatement {
    pub let_token: LetToken,
    pub name: Identifier,
    pub value: Expression,
}
