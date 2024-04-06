use crate::{core::lexer::token::Token, util::source::Span};

use super::Expression;

#[derive(Debug)]
pub enum InfixOperation {
    Plus(Span),
}

impl TryFrom<Token> for InfixOperation {
    type Error = ();

    fn try_from(token: Token) -> Result<Self, Self::Error> {
        match token {
            Token::Plus(token) => Ok(InfixOperation::Plus(token.span)),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub struct Infix {
    pub left: Box<Expression>,
    pub operation: InfixOperation,
    pub right: Box<Expression>,
}
