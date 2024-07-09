mod block;
mod expression;
mod function;
mod statement;

use std::{collections::HashMap, iter::Peekable};

use crate::{
    core::{lexer::token::*, ty::Ty},
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

use super::{
    ctx::{Symbol, SymbolMap, SymbolMapTrait},
    lexer::Lexer,
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

pub trait ParseCtxTrait: TokenGenerator + SymbolMapTrait {}

/// Most basic form of a ctx required for parsing
struct SimpleParseCtx {
    tokens: Peekable<std::vec::IntoIter<Token>>,
    symbols: SymbolMap,
}

impl From<&[Token]> for SimpleParseCtx {
    fn from(tokens: &[Token]) -> Self {
        Self {
            tokens: tokens.to_vec().into_iter().peekable(),
            symbols: Default::default(),
        }
    }
}

impl<I, S> From<(I, S)> for SimpleParseCtx
where
    I: AsRef<[Token]>,
    S: AsRef<[&'static str]>,
{
    fn from((tokens, symbols): (I, S)) -> Self {
        Self {
            tokens: tokens.as_ref().to_vec().into_iter().peekable(),
            symbols: SymbolMap::from_iter(symbols.as_ref().iter()),
        }
    }
}

impl TokenGenerator for SimpleParseCtx {
    fn next_token(&mut self) -> Token {
        self.tokens.next().unwrap_or(Token::EOF(Default::default()))
    }

    fn peek_token(&mut self) -> Token {
        self.tokens
            .peek()
            .cloned()
            .unwrap_or(Token::EOF(Default::default()))
    }
}

impl SymbolMapTrait for SimpleParseCtx {
    fn intern(&mut self, s: impl AsRef<str>) -> super::ctx::Symbol {
        self.symbols.get_or_intern(s)
    }

    fn get(&self, s: Symbol) -> String {
        self.symbols.resolve(s).unwrap().to_string()
    }

    fn dump_symbols(&self) -> SymbolMap {
        SymbolMapTrait::dump_symbols(&self.symbols)
    }
}

impl ParseCtxTrait for SimpleParseCtx {}

pub fn parse(ctx: &mut impl ParseCtxTrait) -> Result<Program, ParseError> {
    // WARN: wacky af
    let main = ctx.intern("main");

    // Parse each expression which should be followed by a semicolon
    let mut functions = std::iter::from_fn(|| match ctx.peek_token() {
        Token::EOF(_) => None,
        _ => Some(parse_function(ctx).map(|function| (function.name, function))),
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

enum BooleanToken {
    True(TrueToken),
    False(FalseToken),
}

pub trait TokenGenerator {
    fn peek_token(&mut self) -> Token;
    fn next_token(&mut self) -> Token;

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

impl TokenGenerator for Lexer {
    fn peek_token(&mut self) -> Token {
        self.peek_token()
    }

    fn next_token(&mut self) -> Token {
        self.next_token()
    }
}
