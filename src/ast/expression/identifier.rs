use crate::token::IdentToken;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Identifier {
    pub ident_token: IdentToken,
    pub value: String,
}
