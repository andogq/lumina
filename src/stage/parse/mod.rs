mod block;
mod expression;
mod function;
mod statement;
mod token_generator;

use std::collections::HashMap;

use crate::ctx::SymbolMapTrait;
use crate::repr::token::*;
use crate::util::source::*;

use self::block::*;
use self::expression::*;
use self::function::*;
use self::statement::*;
pub use self::token_generator::TokenGenerator;

use crate::repr::ast::untyped::*;

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

pub fn parse(
    ctx: &mut impl SymbolMapTrait,
    tokens: &mut impl TokenGenerator,
) -> Result<Program, ParseError> {
    // WARN: wacky af
    let main = ctx.intern("main");

    // Parse each expression which should be followed by a semicolon
    let mut functions = std::iter::from_fn(|| match tokens.peek_token() {
        Token::EOF(_) => None,
        _ => Some(parse_function(ctx, tokens).map(|function| (function.name, function))),
    })
    .collect::<Result<HashMap<_, _>, _>>()?;

    let Some(main) = functions.remove(&main) else {
        return Err(ParseError::MissingMain);
    };

    let program = Program::new(
        functions.into_values().collect(),
        main,
        // TODO: This should just reference the global ctx
        ctx.dump_symbols(),
        // WARN: Really should be something better
        Span::default(),
    );

    Ok(program)
}
