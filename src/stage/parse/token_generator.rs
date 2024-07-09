use std::iter::Peekable;

use crate::{repr::token::*, stage::parse::ParseError};

pub trait TokenGenerator {
    fn peek_token(&mut self) -> Token;
    fn next_token(&mut self) -> Token;

    fn integer(&mut self, reason: impl ToString) -> Result<IntegerToken, ParseError> {
        match self.next_token() {
            Token::Integer(token) => Ok(token),
            token => Err(ParseError::ExpectedToken {
                expected: Box::new(Token::Integer(Default::default())),
                found: Box::new(token),
                reason: reason.to_string(),
            }),
        }
    }

    fn ident(&mut self, reason: impl ToString) -> Result<IdentToken, ParseError> {
        match self.next_token() {
            Token::Ident(token) => Ok(token),
            token => Err(ParseError::ExpectedToken {
                expected: Box::new(Token::Ident(Default::default())),
                found: Box::new(token),
                reason: reason.to_string(),
            }),
        }
    }

    fn boolean(&mut self, reason: impl ToString) -> Result<BooleanToken, ParseError> {
        match self.next_token() {
            Token::True(token) => Ok(BooleanToken::True(token)),
            Token::False(token) => Ok(BooleanToken::False(token)),
            token => Err(ParseError::ExpectedToken {
                // BUG: This should be a true or false token
                expected: Box::new(Token::t_true()),
                found: Box::new(token),
                reason: reason.to_string(),
            }),
        }
    }

    fn t_if(&mut self, reason: impl ToString) -> Result<IfToken, ParseError> {
        match self.next_token() {
            Token::If(token) => Ok(token),
            token => Err(ParseError::ExpectedToken {
                expected: Box::new(Token::t_if()),
                found: Box::new(token),
                reason: reason.to_string(),
            }),
        }
    }
}

impl<I> TokenGenerator for Peekable<I>
where
    I: Iterator<Item = Token>,
{
    fn peek_token(&mut self) -> Token {
        self.peek()
            .cloned()
            .unwrap_or(Token::EOF(Default::default()))
    }

    fn next_token(&mut self) -> Token {
        self.next().unwrap_or(Token::EOF(Default::default()))
    }
}
