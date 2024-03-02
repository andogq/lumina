use std::fmt::Display;

use crate::core::{
    ast::Expression,
    lexer::{
        AsteriskToken, EqToken, LeftAngleToken, Lexer, MinusToken, NotEqToken, PlusToken,
        RightAngleToken, SlashToken, Token,
    },
    parser::Precedence,
};

use super::parse_expression;

#[derive(Clone, Debug, PartialEq)]
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

#[derive(Clone, Debug, PartialEq)]
pub struct InfixExpression {
    pub operator_token: InfixOperatorToken,
    pub operator: String,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

impl InfixExpression {
    pub fn parse_with_left<S>(
        lexer: &mut Lexer<S>,
        left: Expression,
    ) -> Result<InfixExpression, String>
    where
        S: Iterator<Item = char>,
    {
        let (precedence, operator, operator_token) = {
            let token = lexer.next();

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

        let right = parse_expression(lexer, precedence)?;

        Ok(InfixExpression {
            operator_token,
            operator,
            left: Box::new(left),
            right: Box::new(right),
        })
    }
}

impl Display for InfixExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({} {} {})",
            self.left.to_string(),
            self.operator,
            self.right.to_string()
        )
    }
}
