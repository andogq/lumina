use crate::token::{
    AsteriskToken, BangToken, EqToken, IdentToken, IntToken, LeftAngleToken, LetToken, MinusToken,
    NotEqToken, PlusToken, ReturnToken, RightAngleToken, SlashToken, Token,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Statement {
    Let(LetStatement),
    Return(ReturnStatement),
    Expression(Expression),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LetStatement {
    pub let_token: LetToken,
    pub name: Identifier,
    pub value: Expression,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReturnStatement {
    pub return_token: ReturnToken,
    pub value: Expression,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Identifier {
    pub ident_token: IdentToken,
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IntegerLiteral {
    pub token: IntToken,
    pub value: i64,
}

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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expression {
    Identifier(Identifier),
    Integer(IntegerLiteral),
    Prefix(PrefixExpression),
    Infix(InfixExpression),
}
