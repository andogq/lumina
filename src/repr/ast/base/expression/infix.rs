use crate::{ast_node, repr::token::Token};

use super::Expression;

#[derive(Debug, Clone)]
pub enum InfixOperation {
    Minus,
    Plus,
    Eq,
    NotEq,
    And,
    Or,
}

impl InfixOperation {
    pub fn plus() -> Self {
        Self::Plus
    }

    pub fn minus() -> Self {
        Self::Minus
    }
}

impl TryFrom<Token> for InfixOperation {
    type Error = ();

    fn try_from(token: Token) -> Result<Self, Self::Error> {
        match token {
            Token::Plus => Ok(InfixOperation::Plus),
            Token::Minus => Ok(InfixOperation::Minus),
            Token::DoubleEq => Ok(InfixOperation::Eq),
            Token::NotEq => Ok(InfixOperation::NotEq),
            Token::And => Ok(InfixOperation::And),
            Token::Or => Ok(InfixOperation::Or),
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
