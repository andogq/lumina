use std::iter::Peekable;

use crate::{
    ast::Expression,
    parser::{Node, Precedence},
    token::{
        AsteriskToken, EqToken, LeftAngleToken, MinusToken, NotEqToken, PlusToken, RightAngleToken,
        SlashToken, Token,
    },
};

use super::parse_expression;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InfixOperatorToken {
    Plus(PlusToken),
    Minus(MinusToken),
    Asterisk(AsteriskToken),
    Slash(SlashToken),
    LeftAngle(LeftAngleToken),
    RightAngle(RightAngleToken),
    Eq(EqToken),
    NotEq(NotEqToken),
}

impl TryFrom<Token> for InfixOperatorToken {
    type Error = String;

    fn try_from(token: Token) -> Result<Self, Self::Error> {
        match token {
            Token::Plus(token) => Ok(InfixOperatorToken::Plus(token)),
            Token::Minus(token) => Ok(InfixOperatorToken::Minus(token)),
            Token::Asterisk(token) => Ok(InfixOperatorToken::Asterisk(token)),
            Token::Slash(token) => Ok(InfixOperatorToken::Slash(token)),
            Token::LeftAngle(token) => Ok(InfixOperatorToken::LeftAngle(token)),
            Token::RightAngle(token) => Ok(InfixOperatorToken::RightAngle(token)),
            Token::Eq(token) => Ok(InfixOperatorToken::Eq(token)),
            Token::NotEq(token) => Ok(InfixOperatorToken::NotEq(token)),
            token => Err(format!("unknown infix operator: {token:?}")),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InfixExpression {
    pub operator_token: InfixOperatorToken,
    pub operator: String,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

impl InfixExpression {
    pub fn parse_with_left(
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
}
