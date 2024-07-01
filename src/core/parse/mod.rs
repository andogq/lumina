mod block;
mod expression;
mod function;
mod statement;

use std::collections::HashMap;

use crate::core::lexer::{token::Token, Lexer};

use self::function::parse_function;

use super::{ast::Program, lexer::token::IntegerToken, symbol::SymbolMap};

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("unexpected token encountered: '{0}")]
    UnexpectedToken(Token),

    #[error("expected token '{expected}' but found '{found}': {reason}")]
    ExpectedToken {
        expected: Box<Token>,
        found: Box<Token>,
        reason: String,
    },

    #[error("invalid literal, expected `{expected}`")]
    InvalidLiteral { expected: String },

    #[error("the main function is missing and must be present")]
    MissingMain,

    #[error("the function must have a return statement")]
    MissingReturn,
}

pub fn parse(mut lexer: Lexer) -> Result<Program, ParseError> {
    let mut symbol_map = SymbolMap::new();
    let main = symbol_map.get("main");

    // Parse each expression which should be followed by a semi colon
    let mut functions = std::iter::from_fn(|| {
        lexer.peek().is_some().then(|| {
            let function = parse_function(&mut lexer, &mut symbol_map)?;
            Ok((function.name, function))
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

impl Lexer {
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
}
