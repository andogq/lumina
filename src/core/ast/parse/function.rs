use crate::core::{
    ast::Function,
    lexer::{token::Token, Lexer},
};

use super::{block::parse_block, ParseError};

pub fn parse_function<S>(lexer: &mut Lexer<S>) -> Result<Function, ParseError>
where
    S: Iterator<Item = char>,
{
    // `fn` keyword
    if !matches!(lexer.next(), Token::Fn(_)) {
        return Err(ParseError::ExpectedToken("fn".to_string()));
    }

    // function name
    let Token::Ident(fn_name) = lexer.next() else {
        return Err(ParseError::ExpectedToken("ident".to_string()));
    };

    // opening and closing paren for argument list
    if !matches!(lexer.next(), Token::LeftParen(_)) {
        return Err(ParseError::ExpectedToken("(".to_string()));
    }
    if !matches!(lexer.next(), Token::RightParen(_)) {
        return Err(ParseError::ExpectedToken(")".to_string()));
    }

    // arrow for return type
    if !matches!(lexer.next(), Token::ThinArrow(_)) {
        return Err(ParseError::ExpectedToken("->".to_string()));
    }

    // return type (can currently only be `int`)
    match lexer.next() {
        Token::Ident(ident) => {
            if ident.literal != "int" {
                return Err(ParseError::ExpectedToken(
                    "int type is only supported type".to_string(),
                ));
            }
        }
        _ => return Err(ParseError::ExpectedToken("return type".to_string())),
    }

    Ok(Function {
        name: fn_name.literal,
        body: parse_block(lexer)?,
    })
}
