use std::fmt::Display;

use crate::{
    ast::{AstNode, ParseNode},
    interpreter::{environment::Environment, return_value::Return},
    lexer::Lexer,
    object::{Object, StringObject},
    token::{StringToken, Token},
};

#[derive(Clone, Debug, PartialEq)]
pub struct StringLiteral {
    pub token: StringToken,
    pub value: String,
}

impl AstNode for StringLiteral {
    fn evaluate(&self, _env: Environment) -> Return<Object> {
        Return::Implicit(Object::String(StringObject {
            value: self.value.clone(),
        }))
    }
}

impl<S> ParseNode<S> for StringLiteral
where
    S: Iterator<Item = char>,
{
    fn parse(lexer: &mut Lexer<S>) -> Result<Self, String> {
        let Token::String(string_token) = lexer.next() else {
            return Err("expected string".to_string());
        };

        Ok(StringLiteral {
            value: string_token.literal.clone(),
            token: string_token,
        })
    }
}

impl Display for StringLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, r#""{}""#, self.value)
    }
}
