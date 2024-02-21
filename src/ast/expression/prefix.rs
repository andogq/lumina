use std::iter::Peekable;

use crate::{
    ast::Expression,
    parser::{Node, Precedence},
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

impl Node for PrefixExpression {
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

impl ToString for PrefixExpression {
    fn to_string(&self) -> String {
        format!("{}{}", self.operator, self.right.to_string())
    }
}
