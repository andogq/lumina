mod boolean_literal;
mod call_expression;
mod function_literal;
mod identifier;
mod if_expression;
mod infix;
mod integer_literal;
mod prefix;
mod string_literal;

use std::fmt::Display;

pub use boolean_literal::*;
pub use call_expression::*;
pub use function_literal::*;
pub use identifier::*;
pub use if_expression::*;
pub use infix::*;
pub use integer_literal::*;
pub use prefix::*;
pub use string_literal::*;

use crate::core::{
    lexer::{Lexer, Token},
    parser::Precedence,
};

use super::ParseNode;

#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    Identifier(Identifier),
    Integer(IntegerLiteral),
    String(StringLiteral),
    Boolean(BooleanLiteral),
    Prefix(PrefixExpression),
    Infix(InfixExpression),
    If(Box<IfExpression>),
    Function(FunctionLiteral),
    Call(CallExpression),
}

impl<S> ParseNode<S> for Expression
where
    S: Iterator<Item = char>,
{
    fn parse(lexer: &mut Lexer<S>) -> Result<Self, String> {
        let expression = parse_expression(lexer, Precedence::Lowest)?;

        Ok(expression)
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Identifier(identifier) => identifier.fmt(f),
            Expression::Integer(integer) => integer.fmt(f),
            Expression::Boolean(boolean) => boolean.fmt(f),
            Expression::String(string) => string.fmt(f),
            Expression::Prefix(prefix) => prefix.fmt(f),
            Expression::Infix(infix) => infix.fmt(f),
            Expression::If(if_expression) => if_expression.fmt(f),
            Expression::Function(function) => function.fmt(f),
            Expression::Call(call) => call.fmt(f),
        }
    }
}

fn parse_expression<S>(lexer: &mut Lexer<S>, precedence: Precedence) -> Result<Expression, String>
where
    S: Iterator<Item = char>,
{
    let mut left = parse_prefix(lexer)?;

    while !matches!(lexer.peek(), Token::Semicolon(_)) && precedence < Precedence::of(&lexer.peek())
    {
        // Depending on the following token, continue parsing in a different manner
        left = match lexer.peek() {
            // Opening bracket, potentially a function call
            Token::LeftParen(_) => Expression::Call(CallExpression::parse_with_left(left, lexer)?),

            // An infix operator
            token if InfixOperatorToken::try_from(token.clone()).is_ok() => {
                Expression::Infix(InfixExpression::parse_with_left(lexer, left)?)
            }

            // Some unknown token, potentially the end of the expression
            _ => break,
        };
    }

    Ok(left)
}

fn parse_prefix<S>(lexer: &mut Lexer<S>) -> Result<Expression, String>
where
    S: Iterator<Item = char>,
{
    // NOTE: Effectively the `nud` map
    match lexer.peek() {
        Token::Ident(_) => Ok(Expression::Identifier(Identifier::parse(lexer)?)),
        Token::Int(_) => Ok(Expression::Integer(IntegerLiteral::parse(lexer)?)),
        Token::String(_) => Ok(Expression::String(StringLiteral::parse(lexer)?)),
        Token::True(_) | Token::False(_) => Ok(Expression::Boolean(BooleanLiteral::parse(lexer)?)),
        Token::Bang(_) | Token::Plus(_) | Token::Minus(_) => {
            Ok(Expression::Prefix(PrefixExpression::parse(lexer)?))
        }
        Token::LeftParen(_) => Ok(parse_grouped_expression(lexer)?),
        Token::If(_) => Ok(Expression::If(Box::new(IfExpression::parse(lexer)?))),
        Token::Function(_) => Ok(Expression::Function(FunctionLiteral::parse(lexer)?)),
        token => Err(format!("no prefix parse function found for {token:?}")),
    }
}

fn parse_grouped_expression<S>(lexer: &mut Lexer<S>) -> Result<Expression, String>
where
    S: Iterator<Item = char>,
{
    let Token::LeftParen(_left_paren_token) = lexer.next() else {
        return Err("expected left parenthesis".to_string());
    };

    let expression = parse_expression(lexer, Precedence::Lowest)?;

    let Token::RightParen(_right_paren_token) = lexer.next() else {
        return Err("expected right parenthesis".to_string());
    };

    Ok(expression)
}
