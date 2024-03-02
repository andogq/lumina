use std::fmt::{Display, Formatter};

use crate::{
    ast::{AstNode, Expression, ParseNode},
    interpreter::{environment::Environment, error::Error, return_value::Return},
    lexer::Lexer,
    object::{BooleanObject, Object},
    parser::Precedence,
    return_value,
    token::{BangToken, MinusToken, PlusToken, Token},
};

use super::parse_expression;

#[derive(Clone, Debug, PartialEq)]
pub enum PrefixToken {
    Plus(PlusToken),
    Minus(MinusToken),
    Bang(BangToken),
}

#[derive(Clone, Debug, PartialEq)]
pub struct PrefixExpression {
    pub prefix_token: PrefixToken,
    pub operator: String,
    pub right: Box<Expression>,
}

impl AstNode for PrefixExpression {
    fn evaluate(&self, env: Environment) -> Return<Object> {
        let right = return_value!(self.right.evaluate(env));

        match (&self.prefix_token, right) {
            (PrefixToken::Plus(_), Object::Integer(int)) => Return::Implicit(Object::Integer(int)),
            (PrefixToken::Minus(_), Object::Integer(mut int)) => {
                Return::Implicit(Object::Integer({
                    int.value = -int.value;
                    int
                }))
            }
            (PrefixToken::Bang(_), Object::Boolean(bool)) => {
                Return::Implicit(Object::Boolean(BooleanObject { value: !bool.value }))
            }
            (PrefixToken::Bang(_), Object::Null(_)) => {
                Return::Implicit(Object::Boolean(BooleanObject { value: true }))
            }
            _ => Error::throw("prefix operation not supported"),
        }
    }
}

impl<S> ParseNode<S> for PrefixExpression
where
    S: Iterator<Item = char>,
{
    fn parse(lexer: &mut Lexer<S>) -> Result<Self, String> {
        let (prefix_token, operator) = match lexer.next() {
            Token::Plus(token) => Ok((PrefixToken::Plus(token), "+".to_string())),
            Token::Minus(token) => Ok((PrefixToken::Minus(token), "-".to_string())),
            Token::Bang(token) => Ok((PrefixToken::Bang(token), "!".to_string())),
            token => Err(format!("unknown prefix operator: {token:?}")),
        }?;

        let right = parse_expression(lexer, Precedence::Prefix)?;

        Ok(PrefixExpression {
            prefix_token,
            operator,
            right: Box::new(right),
        })
    }
}

impl Display for PrefixExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.operator, self.right.to_string())
    }
}
