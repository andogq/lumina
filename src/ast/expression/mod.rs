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

        left = Expression::Infix(parse_infix_expression(tokens, left)?);
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
            Ok(Expression::Prefix(parse_prefix_expression(tokens)?))
        }
        Token::LeftParen(_) => Ok(parse_grouped_expression(tokens)?),
        token => Err(format!("no prefix parse function found for {token:?}")),
    }
}

fn parse_infix_expression(
    tokens: &mut Peekable<impl Iterator<Item = Token>>,
    left: Expression,
) -> Result<InfixExpression, String> {
    let (precedence, operator, operator_token) = {
        let token = tokens
            .next()
            .ok_or_else(|| "expected infix operator".to_string())?;

        (
            Precedence::of(&token),
            match &token {
                Token::Plus(_) => Ok("+".to_string()),
                Token::Minus(_) => Ok("-".to_string()),
                Token::Asterisk(_) => Ok("*".to_string()),
                Token::Slash(_) => Ok("/".to_string()),
                Token::LeftAngle(_) => Ok("<".to_string()),
                Token::RightAngle(_) => Ok(">".to_string()),
                Token::Eq(_) => Ok("==".to_string()),
                Token::NotEq(_) => Ok("!=".to_string()),
                token => Err(format!("unknown infix operator: {token:?}")),
            }?,
            InfixOperatorToken::try_from(token)?,
        )
    };

    let right = parse_expression(tokens, precedence)?;

    Ok(InfixExpression {
        operator_token,
        operator,
        left: Box::new(left),
        right: Box::new(right),
    })
}

fn parse_prefix_expression(
    tokens: &mut Peekable<impl Iterator<Item = Token>>,
) -> Result<PrefixExpression, String> {
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

fn parse_grouped_expression(
    tokens: &mut Peekable<impl Iterator<Item = Token>>,
) -> Result<Expression, String> {
    let left_paren_token = tokens
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

    let right_paren_token = tokens
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
