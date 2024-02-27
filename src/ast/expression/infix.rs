use std::fmt::Display;

use crate::{
    ast::{AstNode, Expression},
    code::Instruction,
    interpreter::{environment::Environment, error::Error, return_value::Return},
    lexer::Lexer,
    object::{BooleanObject, IntegerObject, NullObject, Object, StringObject},
    parser::Precedence,
    return_value,
    token::{
        AsteriskToken, EqToken, LeftAngleToken, MinusToken, NotEqToken, PlusToken, RightAngleToken,
        SlashToken, Token,
    },
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

impl AstNode for InfixExpression {
    fn evaluate(&self, env: Environment) -> Return<Object> {
        use InfixOperatorToken::*;

        let left = return_value!(self.left.evaluate(env.clone()));
        let right = return_value!(self.right.evaluate(env));

        Return::Implicit(match (&self.operator_token, left, right) {
            (token, Object::Integer(left), Object::Integer(right)) => {
                let left = left.value;
                let right = right.value;

                match token {
                    Plus(_) | Minus(_) | Asterisk(_) | Slash(_) => Object::Integer(IntegerObject {
                        value: match token {
                            Plus(_) => left + right,
                            Minus(_) => left - right,
                            Asterisk(_) => left * right,
                            Slash(_) => left / right,
                            _ => unreachable!(),
                        },
                    }),
                    LeftAngle(_) | RightAngle(_) | Eq(_) | NotEq(_) => {
                        Object::Boolean(BooleanObject {
                            value: match token {
                                LeftAngle(_) => left < right,
                                RightAngle(_) => left > right,
                                Eq(_) => left == right,
                                NotEq(_) => left != right,
                                _ => unreachable!(),
                            },
                        })
                    }
                }
            }
            (token, Object::Boolean(left), Object::Boolean(right)) => {
                let left = left.value;
                let right = right.value;

                Object::Boolean(BooleanObject {
                    value: match token {
                        LeftAngle(_) => left < right,
                        RightAngle(_) => left > right,
                        Eq(_) => left == right,
                        NotEq(_) => left != right,
                        _ => return Return::Implicit(Object::Null(NullObject)),
                    },
                })
            }
            (Plus(_), Object::String(left), Object::String(right)) => {
                Object::String(StringObject {
                    value: left.value + &right.value,
                })
            }

            // If hasn't already been evaluated, left and right aren't equal
            (Eq(_), _, _) => Object::Boolean(BooleanObject { value: false }),
            (NotEq(_), _, _) => Object::Boolean(BooleanObject { value: true }),

            _ => return Error::throw("insupported infix operation"),
        })
    }

    fn compile(
        &self,
        register_constant: &mut impl FnMut(Object) -> u32,
    ) -> Result<Vec<Instruction>, String> {
        let mut instructions = self.left.compile(register_constant)?;
        instructions.append(&mut self.right.compile(register_constant)?);
        instructions.push(match self.operator_token {
            InfixOperatorToken::Plus(_) => Instruction::Add,
            InfixOperatorToken::Minus(_) => todo!(),
            InfixOperatorToken::Asterisk(_) => todo!(),
            InfixOperatorToken::Slash(_) => todo!(),
            InfixOperatorToken::LeftAngle(_) => todo!(),
            InfixOperatorToken::RightAngle(_) => todo!(),
            InfixOperatorToken::Eq(_) => todo!(),
            InfixOperatorToken::NotEq(_) => todo!(),
        });
        Ok(instructions)
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
