use std::iter::Peekable;

use crate::{
    ast::{Expression, Identifier},
    parser::Node,
    token::{LetToken, Token},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LetStatement {
    pub let_token: LetToken,
    pub name: Identifier,
    pub value: Expression,
}

impl Node for LetStatement {
    fn parse(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<Self, String> {
        let let_token = tokens
            .next()
            .and_then(|token| {
                if let Token::Let(let_token) = token {
                    Some(let_token)
                } else {
                    None
                }
            })
            .ok_or_else(|| "expected `let` token".to_string())?;

        let name = tokens
            .next()
            .and_then(|token| {
                if let Token::Ident(name_ident_token) = token {
                    Some(Identifier {
                        value: name_ident_token.literal.clone(),
                        ident_token: name_ident_token,
                    })
                } else {
                    None
                }
            })
            .ok_or_else(|| "expected `ident` token".to_string())?;

        let _assign_token = tokens
            .next()
            .and_then(|token| {
                if let Token::Assign(assign_token) = token {
                    Some(assign_token)
                } else {
                    None
                }
            })
            .ok_or_else(|| "expected `assign` token".to_string())?;

        // TODO: Read expression instead of skipping to semicolon
        while tokens
            .next_if(|token| !matches!(token, Token::Semicolon(_)))
            .is_some()
        {}

        let _semicolon_token = tokens
            .next()
            .and_then(|token| {
                if let Token::Semicolon(semicolon_token) = token {
                    Some(semicolon_token)
                } else {
                    None
                }
            })
            .ok_or_else(|| "expected `semicolon` token".to_string())?;

        Ok(LetStatement {
            let_token,
            name,
            value: todo!(),
        })
    }
}
