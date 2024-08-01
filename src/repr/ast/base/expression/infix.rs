use crate::{ast_node, repr::token::Token, util::source::Span};

use super::Expression;

#[derive(Debug, Clone)]
pub enum InfixOperation {
    Minus(Span),
    Plus(Span),
    Eq(Span),
    NotEq(Span),
}

impl InfixOperation {
    pub fn plus() -> Self {
        Self::Plus(Span::default())
    }

    pub fn minus() -> Self {
        Self::Minus(Span::default())
    }
}

impl TryFrom<Token> for InfixOperation {
    type Error = ();

    fn try_from(token: Token) -> Result<Self, Self::Error> {
        match token {
            Token::Plus(token) => Ok(InfixOperation::Plus(token.span)),
            Token::Minus(token) => Ok(InfixOperation::Minus(token.span)),
            Token::Eq(token) => Ok(InfixOperation::Eq(token.span)),
            Token::NotEq(token) => Ok(InfixOperation::NotEq(token.span)),
            _ => Err(()),
        }
    }
}

ast_node! {
    typed struct Infix<TyInfo, FnIdentifier, IdentIdentifier> {
        left: Box<Expression<TyInfo, FnIdentifier, IdentIdentifier>>,
        operation: InfixOperation,
        right: Box<Expression<TyInfo, FnIdentifier, IdentIdentifier>>,
    }
}
