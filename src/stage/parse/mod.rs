mod block;
mod expression;
mod function;
mod statement;
mod ty;

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

pub fn parse(compiler: &mut Compiler, source: &str) -> Result<Program, ParseError> {
    let mut tokens: Lexer = source.into();

    // WARN: wacky af
    let main = compiler.symbols.get_or_intern("main");

    // Parse each expression which should be followed by a semicolon
    let mut functions = std::iter::from_fn(|| {
        Some(match tokens.peek_token()? {
            Token::Fn => {
                parse_function(compiler, &mut tokens).map(|function| (function.name, function))
            }
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

struct Lexer<'source> {
    next: Option<(Token, Span)>,
    lexer: Peekable<logos::SpannedIter<'source, Token>>,
}

impl<'source> Lexer<'source> {
    fn new(lexer: logos::Lexer<'source, Token>) -> Self {
        Self {
            lexer: lexer.spanned().peekable(),
            next: None,
        }
    }

    fn next_token(&mut self) -> Option<Token> {
        self.next_spanned().map(|(token, _)| token)
    }

    fn peek_token(&mut self) -> Option<&Token> {
        self.peek_spanned().map(|(token, _)| token)
    }

    fn next_spanned(&mut self) -> Option<(Token, Span)> {
        self.next.take().or_else(|| self.next())
    }

    fn peek_spanned(&mut self) -> Option<(&Token, &Span)> {
        self.next
            .as_ref()
            .map(|(token, span)| (token, span))
            .or_else(|| {
                self.lexer
                    .peek()
                    .map(|(result, span)| (result.as_ref().unwrap(), span))
            })
    }

    fn double_peek_token(&mut self) -> Option<&Token> {
        if self.next.is_none() {
            self.next = self.next();
        }

        self.lexer
            .peek()
            .map(|(result, _)| result.as_ref().unwrap())
    }

    fn next(&mut self) -> Option<(Token, Span)> {
        self.lexer
            .next()
            .map(|(result, span)| (result.unwrap(), span))
    }

    #[allow(dead_code)]
    fn count(self) -> usize {
        self.lexer.count() + self.next.map(|_| 1).unwrap_or(0)
    }
}

impl<'source> From<&'source str> for Lexer<'source> {
    fn from(source: &'source str) -> Self {
        Token::lexer(source).into()
    }
}

impl<'source> From<logos::Lexer<'source, Token>> for Lexer<'source> {
    fn from(lexer: logos::Lexer<'source, Token>) -> Self {
        Self::new(lexer)
    }
}

impl<'source> Deref for Lexer<'source> {
    type Target = Peekable<logos::SpannedIter<'source, Token>>;

    fn deref(&self) -> &Self::Target {
        &self.lexer
    }
}

impl<'source> DerefMut for Lexer<'source> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.lexer
    }
}
