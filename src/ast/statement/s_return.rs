use std::iter::Peekable;

use crate::{
    ast::Expression,
    parser::Node,
    token::{ReturnToken, Token},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReturnStatement {
    pub return_token: ReturnToken,
    pub value: Expression,
}

impl Node for ReturnStatement {
    fn parse(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<Self, String> {
        let return_token = tokens
            .next()
            .and_then(|token| {
                if let Token::Return(return_token) = token {
                    Some(return_token)
                } else {
                    None
                }
            })
            .ok_or_else(|| "expected `return` token".to_string())?;

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

        Ok(ReturnStatement {
            return_token,
            value: todo!(),
        })
    }
}
