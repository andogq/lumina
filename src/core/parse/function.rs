use crate::core::{
    ast::Function,
    lexer::{
        token::{FnToken, IdentToken, LeftParenToken, RightParenToken, ThinArrowToken, Token},
        Lexer,
    },
    symbol::SymbolMap,
    ty::Ty,
};

use super::{block::parse_block, ParseError};

pub fn parse_function(lexer: &mut Lexer, symbols: &mut SymbolMap) -> Result<Function, ParseError> {
    // `fn` keyword
    let fn_token = match lexer.next() {
        Token::Fn(fn_token) => fn_token,
        token => {
            return Err(ParseError::ExpectedToken {
                expected: Token::Fn(FnToken::default()),
                found: token,
                reason: "function declaration must begin with keyword".to_string(),
            });
        }
    };

    // function name
    let fn_name = match lexer.next() {
        Token::Ident(fn_name) => fn_name,
        token => {
            return Err(ParseError::ExpectedToken {
                expected: Token::Ident(IdentToken::default()),
                found: token,
                reason: "function declaration requires identifier".to_string(),
            });
        }
    };

    // opening paren for argument list
    match lexer.next() {
        Token::LeftParen(_) => (),
        token => {
            return Err(ParseError::ExpectedToken {
                expected: Token::LeftParen(LeftParenToken::default()),
                found: token,
                reason: "argument list must begin with opening parenthesis".to_string(),
            });
        }
    }

    // TODO: this
    let parameters = Vec::new();

    // closing paren for argument list
    match lexer.next() {
        Token::RightParen(_) => (),
        token => {
            return Err(ParseError::ExpectedToken {
                expected: Token::RightParen(RightParenToken::default()),
                found: token,
                reason: "argument list must end with closing parenthesis".to_string(),
            });
        }
    }

    // arrow for return type
    match lexer.next() {
        Token::ThinArrow(_) => (),
        token => {
            return Err(ParseError::ExpectedToken {
                expected: Token::ThinArrow(ThinArrowToken::default()),
                found: token,
                reason: "thin arrow must preceed return type".to_string(),
            });
        }
    }

    // return type (can currently only be `int`)
    let return_ty = match lexer.next() {
        Token::Ident(ident) => match ident.literal.as_str() {
            "int" => Ty::Int,
            _ => {
                panic!("only int can be returned from a function")
            }
        },
        token => {
            return Err(ParseError::ExpectedToken {
                expected: Token::Ident(IdentToken::default()),
                found: token,
                reason: "return type must follow thin arrow".to_string(),
            });
        }
    };

    let body = parse_block(lexer, symbols)?;

    Ok(Function {
        span: fn_token.span.to(&body),
        name: symbols.get(fn_name.literal),
        parameters,
        return_ty,
        body,
    })
}
