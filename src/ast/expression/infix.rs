use crate::{
    ast::Expression,
    token::{
        AsteriskToken, EqToken, LeftAngleToken, MinusToken, NotEqToken, PlusToken, RightAngleToken,
        SlashToken, Token,
    },
};

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
