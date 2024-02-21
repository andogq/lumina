mod boolean_literal;
mod identifier;
mod infix;
mod integer_literal;
mod prefix;

use std::iter::Peekable;

pub use boolean_literal::*;
pub use identifier::*;
pub use infix::*;
pub use integer_literal::*;
pub use prefix::*;

use crate::{
    parser::{Node, Precedence},
    token::Token,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expression {
    Identifier(Identifier),
    Integer(IntegerLiteral),
    Boolean(BooleanLiteral),
    Prefix(PrefixExpression),
    Infix(InfixExpression),
}

impl Node for Expression {
    fn parse(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<Self, String> {
        let expression = parse_expression(tokens, Precedence::Lowest)?;

        // Advance past semicolon, if present
        tokens.next_if(|token| matches!(token, Token::Semicolon(_)));

        Ok(expression)
    }
}

fn parse_expression(
    tokens: &mut Peekable<impl Iterator<Item = Token>>,
    precedence: Precedence,
) -> Result<Expression, String> {
    let mut left = parse_prefix(tokens)?;

    while tokens
        .peek()
        .map(|token| !matches!(token, Token::Semicolon(_)) && precedence < Precedence::of(token))
        .unwrap_or(false)
    {
        // Make sure next token is an infix operator
        if tokens
            .peek()
            .map(|token| InfixOperatorToken::try_from(token.clone()).is_err())
            .unwrap_or(true)
        {
            break;
        }

        left = Expression::Infix(InfixExpression::parse_with_left(tokens, left)?);
    }

    Ok(left)
}

fn parse_prefix(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<Expression, String> {
    // NOTE: Effectively the `nud` map
    match tokens
        .peek()
        .ok_or_else(|| "expected token for prefix expression".to_string())?
    {
        Token::Ident(_) => Ok(Expression::Identifier(Identifier::parse(tokens)?)),
        Token::Int(_) => Ok(Expression::Integer(IntegerLiteral::parse(tokens)?)),
        Token::True(_) | Token::False(_) => Ok(Expression::Boolean(BooleanLiteral::parse(tokens)?)),
        Token::Bang(_) | Token::Plus(_) | Token::Minus(_) => {
            Ok(Expression::Prefix(PrefixExpression::parse(tokens)?))
        }
        Token::LeftParen(_) => Ok(parse_grouped_expression(tokens)?),
        token => Err(format!("no prefix parse function found for {token:?}")),
    }
}

fn parse_grouped_expression(
    tokens: &mut Peekable<impl Iterator<Item = Token>>,
) -> Result<Expression, String> {
    let _left_paren_token = tokens
        .next()
        .and_then(|token| {
            if let Token::LeftParen(token) = token {
                Some(token)
            } else {
                None
            }
        })
        .ok_or_else(|| "expected left parenthesis".to_string())?;

    let expression = parse_expression(tokens, Precedence::Lowest)?;

    let _right_paren_token = tokens
        .next()
        .and_then(|token| {
            if let Token::RightParen(token) = token {
                Some(token)
            } else {
                None
            }
        })
        .ok_or_else(|| "expected right parenthesis".to_string())?;

    Ok(expression)
}
