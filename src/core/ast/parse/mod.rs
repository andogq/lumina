mod block;
mod expression;
mod function;
mod statement;

use std::collections::HashMap;

use crate::core::lexer::{token::Token, Lexer};

use self::function::parse_function;

use super::{source::Program, symbol::SymbolMap};

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

    #[error("the function must have a return statement")]
    MissingReturn,
}

pub fn parse<S>(mut lexer: Lexer<S>) -> Result<Program, ParseError>
where
    S: Iterator<Item = char>,
{
    let mut symbol_map = SymbolMap::new();
    let main = symbol_map.get("main");

    // Parse each expression which should be followed by a semi colon
    let mut functions = std::iter::from_fn(|| {
        (!matches!(lexer.peek(), Token::EOF(_))).then(|| {
            let function = parse_function(&mut lexer, &mut symbol_map)?;
            Ok((function.name.clone(), function))
        })
    })
    .collect::<Result<HashMap<_, _>, _>>()?;

    let Some(main) = functions.remove(&main) else {
        return Err(ParseError::MissingMain);
    };

    Ok(Program {
        functions: functions.into_values().collect(),
        main,
        symbol_map,
    })
}
