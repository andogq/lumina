use crate::{ast_node2, repr::token::Token};

use super::Expression;

#[derive(Debug, Clone)]
pub enum InfixOperation {
    Minus,
    Plus,
    Eq,
    NotEq,
    Greater,
    Less,
    GreaterEq,
    LessEq,
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
            Token::LeftAngle => Ok(InfixOperation::Less),
            Token::RightAngle => Ok(InfixOperation::Greater),
            Token::LeftAngleEq => Ok(InfixOperation::LessEq),
            Token::RightAngleEq => Ok(InfixOperation::GreaterEq),
            Token::And => Ok(InfixOperation::And),
            Token::Or => Ok(InfixOperation::Or),
            _ => Err(()),
        }
    }
}

ast_node2! {
    Infix<M> {
        left: Box<Expression<M>>,
        operation: InfixOperation,
        right: Box<Expression<M>>,
        span,
        ty_info,
    }
}
