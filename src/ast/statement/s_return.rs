use crate::{ast::Expression, token::ReturnToken};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReturnStatement {
    pub return_token: ReturnToken,
    pub value: Expression,
}
