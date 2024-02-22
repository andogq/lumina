mod boolean_literal;
mod call_expression;
mod function_literal;
mod identifier;
mod if_expression;
mod infix;
mod integer_literal;
mod prefix;

use std::iter::Peekable;

pub use boolean_literal::*;
pub use call_expression::*;
pub use function_literal::*;
pub use identifier::*;
pub use if_expression::*;
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
    If(Box<IfExpression>),
    Function(FunctionLiteral),
    Call(CallExpression),
}

impl Node for Expression {
    fn parse(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<Self, String> {
        let expression = parse_expression(tokens, Precedence::Lowest)?;

        Ok(expression)
    }
}

impl ToString for Expression {
    fn to_string(&self) -> String {
        match self {
            Expression::Identifier(identifier) => identifier.to_string(),
            Expression::Integer(integer) => integer.to_string(),
            Expression::Boolean(boolean) => boolean.to_string(),
            Expression::Prefix(prefix) => prefix.to_string(),
            Expression::Infix(infix) => infix.to_string(),
            Expression::If(if_expression) => if_expression.to_string(),
            Expression::Function(function) => function.to_string(),
            Expression::Call(call) => call.to_string(),
        }
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
        // Depending on the following token, continue parsing in a different manner
        left = match tokens
            .peek()
            .ok_or_else(|| "expected token following in expression".to_string())?
        {
            // Opening bracket, potentially a function call
            Token::LeftParen(_) => Expression::Call(CallExpression::parse_with_left(left, tokens)?),

            // An infix operator
            token if InfixOperatorToken::try_from(token.clone()).is_ok() => {
                Expression::Infix(InfixExpression::parse_with_left(tokens, left)?)
            }

            // Some unknown token, potentially the end of the expression
            _ => break,
        };
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
        Token::If(_) => Ok(Expression::If(Box::new(IfExpression::parse(tokens)?))),
        Token::Function(_) => Ok(Expression::Function(FunctionLiteral::parse(tokens)?)),
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
