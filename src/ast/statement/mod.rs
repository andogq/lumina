mod s_block;
mod s_let;
mod s_return;

use std::{fmt::Display, iter::Peekable};

pub use s_block::*;
pub use s_let::*;
pub use s_return::*;

use crate::{
    ast::Expression,
    interpreter::{environment::Environment, object::Object, return_value::Return},
    token::Token,
};

use super::{AstNode, ParseNode};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Statement {
    Let(LetStatement),
    Return(ReturnStatement),
    Expression(Expression),
}

impl AstNode for Statement {
    fn evaluate(&self, env: &mut Environment) -> Return<Object> {
        match self {
            Statement::Let(let_statement) => let_statement.evaluate(env),
            Statement::Return(return_statement) => return_statement.evaluate(env),
            Statement::Expression(expression_statement) => expression_statement.evaluate(env),
        }
    }
}

impl ParseNode for Statement {
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

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::Let(let_statement) => let_statement.fmt(f),
            Statement::Return(return_statement) => return_statement.fmt(f),
            Statement::Expression(expression) => expression.fmt(f),
        }
    }
}
