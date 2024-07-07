mod block;
mod expression;
mod function;
mod statement;

use std::collections::HashMap;

use crate::{
    core::{
        ctx::Ctx,
        lexer::{token::*, Lexer},
        ty::Ty,
    },
    util::source::*,
};

use self::block::*;
use self::expression::*;
use self::function::*;
use self::statement::*;

pub mod ast {
    use crate::{core::ty::Ty, generate_ast};

    generate_ast!(Option<Ty>);
}

use self::ast::*;

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
    ctx: Ctx,
}

impl ParseCtx {
    pub fn new(ctx: Ctx, lexer: Lexer) -> Self {
        Self { lexer, ctx }
    }
}

pub fn parse(ctx: Ctx, lexer: Lexer) -> Result<(Program, Ctx), ParseError> {
    let mut ctx = ParseCtx::new(ctx, lexer);
    // WARN: wacky af
    let main = ctx.ctx.symbols.get_or_intern_static("main");

    // Parse each expression which should be followed by a semicolon
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

    let program = Program::new(
        functions.into_values().collect(),
        main,
        // TODO: This should just reference the global ctx
        ctx.ctx.symbols.clone(),
        // WARN: Really should be something better
        Span::default(),
    );

    Ok((program, ctx.ctx))
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
