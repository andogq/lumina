mod function;
pub mod parser;

use std::collections::HashMap;
use std::iter::Peekable;
use std::ops::Deref;
use std::ops::DerefMut;

use logos::Logos;
use parser::Parser;

use crate::compiler::Compiler;
use crate::hir::{Expression, Parsable, Statement};
use crate::repr::token::*;
use crate::ty::TySpanned;
use crate::util::span::*;

use self::function::*;

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

    #[error("invalid infix left hand side: {reason} ({found:?})")]
    InvalidInfixLhs {
        found: Box<Expression<UntypedAstMetadata>>,
        reason: String,
    },

    #[error("invalid literal, expected `{expected}`")]
    InvalidLiteral { expected: String },

    #[error("the main function is missing and must be present")]
    MissingMain,

    #[error("the function must have a return statement")]
    MissingReturn,

    #[error("expected to parse a block")]
    ExpectedBlock,

    #[error("unexpectedly encountered end of file")]
    UnexpectedEOF,

    #[error("no parsers registered for type: {0}")]
    NoRegisteredParsers(String),
}

#[derive(Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    #[default]
    Lowest,
    Assign,
    Binary,
    Equality,
    Sum,
    Multiply,
    Cast,
    Call,
}

impl Precedence {
    pub fn of(token: &Token) -> Self {
        match token {
            Token::Minus | Token::Plus => Precedence::Sum,
            Token::Asterix | Token::ForwardSlash => Precedence::Multiply,
            Token::And | Token::Or => Precedence::Binary,
            Token::DoubleEq
            | Token::NotEq
            | Token::LeftAngle
            | Token::RightAngle
            | Token::LeftAngleEq
            | Token::RightAngleEq => Precedence::Equality,
            Token::LeftParen | Token::LeftSquare => Precedence::Call,
            Token::Eq
            | Token::AddAssign
            | Token::MinusAssign
            | Token::DivAssign
            | Token::MulAssign => Precedence::Assign,
            Token::As => Precedence::Cast,
            _ => Precedence::Lowest,
        }
    }
}

impl From<InfixOperation> for Precedence {
    fn from(op: InfixOperation) -> Self {
        use InfixOperation::*;

        match op {
            Minus | Plus => Precedence::Sum,
            Multiply | Divide => Precedence::Multiply,
            And | Or => Precedence::Binary,
            Eq | NotEq | Greater | Less | GreaterEq | LessEq => Precedence::Equality,
        }
    }
}

impl From<Token> for Precedence {
    fn from(token: Token) -> Self {
        Self::of(&token)
    }
}

pub fn parse(compiler: &mut Compiler, source: &str) -> Result<Program, ParseError> {
    let mut lexer: Lexer = source.into();

    let parser = {
        let mut parser = Parser::new();

        TySpanned::register(&mut parser);
        Statement::<UntypedAstMetadata>::register(&mut parser);
        Expression::<UntypedAstMetadata>::register(&mut parser);

        parser
    };

    // WARN: wacky af
    let main = compiler.symbols.get_or_intern("main");

    // Parse each expression which should be followed by a semicolon
    let mut functions = std::iter::from_fn(|| {
        Some(match lexer.peek_token()? {
            Token::Fn => parse_function(&parser, compiler, &mut lexer)
                .map(|function| (function.name, function)),
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

pub struct Lexer<'source> {
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

    pub fn next_token(&mut self) -> Option<Token> {
        self.next_spanned().map(|(token, _)| token)
    }

    pub fn peek_token(&mut self) -> Option<&Token> {
        self.peek_spanned().map(|(token, _)| token)
    }

    pub fn next_spanned(&mut self) -> Option<(Token, Span)> {
        self.next.take().or_else(|| self.next())
    }

    pub fn peek_spanned(&mut self) -> Option<(&Token, &Span)> {
        self.next
            .as_ref()
            .map(|(token, span)| (token, span))
            .or_else(|| {
                self.lexer
                    .peek()
                    .map(|(result, span)| (result.as_ref().unwrap(), span))
            })
    }

    pub fn double_peek_token(&mut self) -> Option<&Token> {
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
