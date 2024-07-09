use crate::{ast_node, repr::token::Token, util::source::Span};

use super::Expression;

#[derive(Debug, Clone)]
pub enum InfixOperation {
    Plus(Span),
    Eq(Span),
    NotEq(Span),
}

impl InfixOperation {
    pub fn plus() -> Self {
        Self::Plus(Span::default())
    }
}

impl TryFrom<Token> for InfixOperation {
    type Error = ();

    fn try_from(token: Token) -> Result<Self, Self::Error> {
        match token {
            Token::Plus(token) => Ok(InfixOperation::Plus(token.span)),
            Token::Eq(token) => Ok(InfixOperation::Eq(token.span)),
            Token::NotEq(token) => Ok(InfixOperation::NotEq(token.span)),
            _ => Err(()),
        }
    }
}

ast_node! {
    struct Infix<TyInfo> {
        left: Box<Expression<TyInfo>>,
        operation: InfixOperation,
        right: Box<Expression<TyInfo>>,
    }
}
