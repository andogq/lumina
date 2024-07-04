use crate::{
    core::lexer::token::Token,
    util::source::{Span, Spanned},
};

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

#[derive(Debug, Clone)]
pub struct Infix {
    pub span: Span,
    pub left: Box<Expression>,
    pub operation: InfixOperation,
    pub right: Box<Expression>,
}

impl Infix {
    pub fn new(left: Expression, operation: InfixOperation, right: Expression) -> Self {
        Self {
            span: Span::default(),
            left: Box::new(left),
            operation,
            right: Box::new(right),
        }
    }
}

impl Spanned for Infix {
    fn span(&self) -> &Span {
        &self.span
    }
}
