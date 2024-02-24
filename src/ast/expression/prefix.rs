use std::{
    fmt::{Display, Formatter},
    iter::Peekable,
};

use crate::{
    ast::{AstNode, Expression, ParseNode, Return},
    object::{BooleanObject, NullObject, Object},
    parser::Precedence,
    token::{BangToken, MinusToken, PlusToken, Token},
};

use super::parse_expression;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PrefixToken {
    Plus(PlusToken),
    Minus(MinusToken),
    Bang(BangToken),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PrefixExpression {
    pub prefix_token: PrefixToken,
    pub operator: String,
    pub right: Box<Expression>,
}

impl AstNode for PrefixExpression {
    fn evaluate(&self) -> Return<Object> {
        let right = self.right.evaluate().value();

        Return::Implicit(match self.prefix_token {
            PrefixToken::Plus(_) => {
                if matches!(right, Object::Integer(_)) {
                    right
                } else {
                    Object::Null(NullObject)
                }
            }
            PrefixToken::Minus(_) => match right {
                Object::Integer(mut int) => {
                    int.value = -int.value;

                    Object::Integer(int)
                }
                _ => Object::Null(NullObject),
            },
            PrefixToken::Bang(_) => Object::Boolean(BooleanObject {
                value: match right {
                    Object::Boolean(BooleanObject { value }) => !value,
                    Object::Null(_) => true,
                    _ => false,
                },
            }),
        })
    }
}

impl ParseNode for PrefixExpression {
    fn parse(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<Self, String> {
        let (prefix_token, operator) = match tokens
            .next()
            .ok_or_else(|| "expected prefix operator".to_string())?
        {
            Token::Plus(token) => Ok((PrefixToken::Plus(token), "+".to_string())),
            Token::Minus(token) => Ok((PrefixToken::Minus(token), "-".to_string())),
            Token::Bang(token) => Ok((PrefixToken::Bang(token), "!".to_string())),
            token => Err(format!("unknown prefix operator: {token:?}")),
        }?;

        let right = parse_expression(tokens, Precedence::Prefix)?;

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
