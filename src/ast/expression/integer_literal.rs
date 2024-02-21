use crate::token::IntToken;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IntegerLiteral {
    pub token: IntToken,
    pub value: i64,
}
