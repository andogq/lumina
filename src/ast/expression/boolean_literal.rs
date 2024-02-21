use crate::token::{FalseToken, TrueToken};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BooleanToken {
    True(TrueToken),
    False(FalseToken),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BooleanLiteral {
    pub token: BooleanToken,
    pub value: bool,
}
