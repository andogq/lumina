use std::{
    fmt::{Display, Formatter},
    iter::Peekable,
};

use crate::{
    ast::{AstNode, ParseNode},
    interpreter::{
        environment::Environment,
        object::{NullObject, Object},
        return_value::Return,
    },
    return_value,
    token::Token,
};

use super::Statement;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BlockStatement {
    statements: Vec<Statement>,
}

impl AstNode for BlockStatement {
    fn evaluate(&self, env: &mut Environment) -> Return<Object> {
        let mut result = Object::Null(NullObject);

        for statement in &self.statements {
            result = return_value!(statement.evaluate(env));
        }

        Return::Implicit(result)
    }
}

impl ParseNode for BlockStatement {
    fn parse(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<Self, String> {
        let _l_brace = tokens
            .next()
            .and_then(|token| {
                if let Token::LeftBrace(token) = token {
                    Some(token)
                } else {
                    None
                }
            })
            .ok_or_else(|| "expected opening brace".to_string())?;

        let mut statements = Vec::new();

        while tokens
            .peek()
            .map(|token| !matches!(token, Token::RightBrace(_) | Token::EOF(_)))
            .unwrap_or(false)
        {
            statements.push(Statement::parse(tokens)?);
        }

        let _r_brace = tokens
            .next()
            .and_then(|token| {
                if let Token::RightBrace(token) = token {
                    Some(token)
                } else {
                    None
                }
            })
            .ok_or_else(|| "expected closing brace".to_string())?;

        Ok(Self { statements })
    }
}

impl Display for BlockStatement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ {} }}",
            self.statements
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}
