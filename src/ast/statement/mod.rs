mod s_block;
mod s_let;
mod s_return;

use std::fmt::Display;

pub use s_block::*;
pub use s_let::*;
pub use s_return::*;

use crate::{
    ast::Expression,
    interpreter::{environment::Environment, return_value::Return},
    lexer::Lexer,
    object::Object,
    token::Token,
};

use super::{AstNode, ParseNode};

#[derive(Clone, Debug, PartialEq)]
pub enum Statement {
    Let(LetStatement),
    Return(ReturnStatement),
    Expression(Expression),
}

impl AstNode for Statement {
    fn evaluate(&self, env: Environment) -> Return<Object> {
        match self {
            Statement::Let(let_statement) => let_statement.evaluate(env),
            Statement::Return(return_statement) => return_statement.evaluate(env),
            Statement::Expression(expression_statement) => expression_statement.evaluate(env),
        }
    }

    fn compile(&self, register_constant: impl FnMut(Object) -> u32) -> Result<Vec<u8>, String> {
        todo!()
    }
}

impl<S> ParseNode<S> for Statement
where
    S: Iterator<Item = char>,
{
    fn parse(lexer: &mut Lexer<S>) -> Result<Self, String> {
        match lexer.peek() {
            Token::Let(_) => Ok(Statement::Let(LetStatement::parse(lexer)?)),
            Token::Return(_) => Ok(Statement::Return(ReturnStatement::parse(lexer)?)),
            _ => {
                let expression = Expression::parse(lexer)?;

                // Expression statement may end in semicolon, or be ommitted for implicit returns
                // TODO: Should semicolon checks be done for all statements at this level?
                lexer.next_if(|token| matches!(token, Token::Semicolon(_)));

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
