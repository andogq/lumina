use std::{
    fmt::{Display, Formatter},
    iter::Peekable,
};

use crate::{
    ast::{AstNode, ParseNode, Return},
    object::Object,
    token::Token,
};

use super::Statement;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BlockStatement {
    statements: Vec<Statement>,
}

impl AstNode for BlockStatement {
    fn evaluate(&self) -> Return<Object> {
        let mut result = None;

        for statement in &self.statements {
            result = Some(statement.evaluate());
        }

        // WARN: bad
        result.unwrap()
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
