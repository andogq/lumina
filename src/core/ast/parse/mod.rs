mod expression;
mod function;

use std::collections::HashMap;

use crate::core::lexer::{token::Token, Lexer};

use self::function::parse_function;

use super::node::Program;

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("unexpected token encountered")]
    UnexpectedToken(Token),

    #[error("expected token {0}")]
    ExpectedToken(String),

    #[error("invalid literal, expected `{expected}`")]
    InvalidLiteral { expected: String },

    #[error("the main function is missing and must be present")]
    MissingMain,
}

pub fn parse<S>(mut lexer: Lexer<S>) -> Result<Program, ParseError>
where
    S: Iterator<Item = char>,
{
    // Parse each expression which should be followed by a semi colon
    let mut functions = std::iter::from_fn(|| {
        (!matches!(lexer.peek(), Token::EOF(_))).then(|| {
            let function = parse_function(&mut lexer)?;
            Ok((function.name.clone(), function))
        })
    })
    .collect::<Result<HashMap<_, _>, _>>()?;

    let Some(main) = functions.remove("main") else {
        return Err(ParseError::MissingMain);
    };

    Ok(Program {
        functions: functions.into_values().collect(),
        main,
    })
}
