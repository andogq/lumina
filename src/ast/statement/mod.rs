mod s_block;
mod s_let;
mod s_return;

use std::iter::Peekable;

pub use s_block::*;
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
            _ => {
                let expression = Expression::parse(tokens)?;

                // Expression statement may end in semicolon, or be ommitted for implicit returns
                // TODO: Should semicolon checks be done for all statements at this level?
                tokens.next_if(|token| matches!(token, Token::Semicolon(_)));

                Ok(Statement::Expression(expression))
            }
        }
    }
}

impl ToString for Statement {
    fn to_string(&self) -> String {
        match self {
            Statement::Let(let_statement) => let_statement.to_string(),
            Statement::Return(return_statement) => return_statement.to_string(),
            Statement::Expression(expression) => expression.to_string(),
        }
    }
}
