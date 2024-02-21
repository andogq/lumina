mod s_let;
mod s_return;

use std::iter::Peekable;

pub use s_let::*;
pub use s_return::*;

use crate::{ast::Expression, parser::Node, token::Token};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Statement {
    Let(LetStatement),
    Return(ReturnStatement),
    Expression(Expression),
}

impl Node for Statement {
    fn parse(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<Self, String> {
        match tokens
            .peek()
            .ok_or_else(|| "expected statement to follow".to_string())?
        {
            Token::Let(_) => Ok(Statement::Let(LetStatement::parse(tokens)?)),
            Token::Return(_) => Ok(Statement::Return(ReturnStatement::parse(tokens)?)),
            _ => Ok(Statement::Expression(Expression::parse(tokens)?)),
        }
    }
}
