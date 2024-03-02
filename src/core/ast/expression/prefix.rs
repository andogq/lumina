use std::fmt::{Display, Formatter};

use crate::core::{
    ast::{Expression, ParseNode},
    lexer::{BangToken, Lexer, MinusToken, PlusToken, Token},
    parser::Precedence,
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
