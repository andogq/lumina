mod expression;

use crate::core::lexer::{token::Token, Lexer};

use self::expression::{parse_expression, Precedence};

use super::node::Program;

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("unexpected token encountered")]
    UnexpectedToken(Token),

    #[error("invalid literal, expected `{expected}`")]
    InvalidLiteral { expected: String },
}

pub fn parse<S>(mut lexer: Lexer<S>) -> Result<Program, ParseError>
where
    S: Iterator<Item = char>,
{
    Ok(Program {
        // Parse each expression which should be followed by a semi colon
        expressions: std::iter::from_fn(|| {
            (!matches!(lexer.peek(), Token::EOF(_))).then(|| {
                parse_expression(&mut lexer, Precedence::Lowest).and_then(|expression| match lexer
                    .next()
                {
                    Token::Semicolon(_) => Ok(expression),
                    token => Err(ParseError::UnexpectedToken(token)),
                })
            })
        })
        .collect::<Result<_, _>>()?,
    })
}
