mod block;
mod expression;
mod function;
mod statement;

use std::collections::HashMap;

use crate::core::lexer::{token::Token, Lexer};

use self::function::parse_function;

use super::{
    ast::Program,
    lexer::token::{FalseToken, IdentToken, IfToken, IntegerToken, TrueToken},
    symbol::SymbolMap,
};

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

struct ParseCtx {
    lexer: Lexer,
    symbols: SymbolMap,
}

impl ParseCtx {
    pub fn new(lexer: Lexer) -> Self {
        Self {
            lexer,
            symbols: SymbolMap::new(),
        }
    }
}

pub fn parse(lexer: Lexer) -> Result<Program, ParseError> {
    let mut ctx = ParseCtx::new(lexer);
    let main = ctx.symbols.get("main");

    // Parse each expression which should be followed by a semi colon
    let mut functions = std::iter::from_fn(|| {
        ctx.lexer.peek().is_some().then(|| {
            let function = parse_function(&mut ctx)?;
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
        symbols: ctx.symbols,
    })
}

enum BooleanToken {
    True(TrueToken),
    False(FalseToken),
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
