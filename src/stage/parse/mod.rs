mod block;
mod expression;
mod function;
mod statement;

use std::collections::HashMap;
use std::iter::Peekable;
use std::ops::Deref;
use std::ops::DerefMut;

use logos::Logos;

use crate::compiler::Compiler;
use crate::repr::token::*;
use crate::util::span::*;

use self::block::*;
use self::expression::*;
use self::function::*;
use self::statement::*;

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

pub fn parse(c: &mut Compiler, source: &str) -> Result<Program, ParseError> {
    let mut tokens: Lexer = source.into();

    // WARN: wacky af
    let main = c.intern_string("main");

    // Parse each expression which should be followed by a semicolon
    let mut functions = std::iter::from_fn(|| {
        Some(match tokens.peek_token()? {
            Token::Fn => parse_function(c, &mut tokens).map(|function| (function.name, function)),
            token => Err(ParseError::ExpectedToken {
                expected: Box::new(Token::Fn),
                found: Box::new(token.clone()),
                reason: "only functions can be declared at top level".to_string(),
            }),
        })
    })
    .collect::<Result<HashMap<_, _>, _>>()?;

    let Some(main) = functions.remove(&main) else {
        return Err(ParseError::MissingMain);
    };

    let program = Program::new(
        functions.into_values().collect(),
        main,
        // WARN: Really should be something better
        Span::default(),
    );

    Ok(program)
}

struct Lexer<'source>(Peekable<logos::SpannedIter<'source, Token>>);

impl<'source> Lexer<'source> {
    fn next_token(&mut self) -> Option<Token> {
        self.next_spanned().map(|(token, _)| token)
    }

    fn peek_token(&mut self) -> Option<&Token> {
        self.peek_spanned().map(|(token, _)| token)
    }

    fn next_spanned(&mut self) -> Option<(Token, Span)> {
        self.0.next().map(|(result, span)| (result.unwrap(), span))
    }

    fn peek_spanned(&mut self) -> Option<(&Token, &Span)> {
        self.0
            .peek()
            .map(|(result, span)| (result.as_ref().unwrap(), span))
    }
}

impl<'source> From<&'source str> for Lexer<'source> {
    fn from(source: &'source str) -> Self {
        Token::lexer(source).into()
    }
}

impl<'source> From<logos::Lexer<'source, Token>> for Lexer<'source> {
    fn from(lexer: logos::Lexer<'source, Token>) -> Self {
        Self(lexer.spanned().peekable())
    }
}

impl<'source> Deref for Lexer<'source> {
    type Target = Peekable<logos::SpannedIter<'source, Token>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'source> DerefMut for Lexer<'source> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
