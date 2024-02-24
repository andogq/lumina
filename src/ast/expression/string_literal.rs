use std::{fmt::Display, iter::Peekable};

use crate::{
    ast::{AstNode, ParseNode},
    interpreter::{
        environment::Environment,
        object::{Object, StringObject},
        return_value::Return,
    },
    token::{StringToken, Token},
};

#[derive(Clone, Debug, PartialEq, Eq)]
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

impl ParseNode for StringLiteral {
    fn parse(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<Self, String> {
        let string_token = tokens
            .next()
            .and_then(|token| {
                if let Token::String(string_token) = token {
                    Some(string_token)
                } else {
                    None
                }
            })
            .ok_or_else(|| "expected string".to_string())?;

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
