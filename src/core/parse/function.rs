use crate::core::{
    ast::Function,
    lexer::{token::Token, Lexer},
    symbol::SymbolMap,
    ty::Ty,
};

use super::{block::parse_block, ParseError};

pub fn parse_function<S>(
    lexer: &mut Lexer<S>,
    symbols: &mut SymbolMap,
) -> Result<Function, ParseError>
where
    S: Iterator<Item = char>,
{
    // `fn` keyword
    let Token::Fn(fn_token) = lexer.next() else {
        return Err(ParseError::ExpectedToken("fn".to_string()));
    };
    let span = fn_token.span;

    // function name
    let Token::Ident(fn_name) = lexer.next() else {
        return Err(ParseError::ExpectedToken("ident".to_string()));
    };

    // opening and closing paren for argument list
    if !matches!(lexer.next(), Token::LeftParen(_)) {
        return Err(ParseError::ExpectedToken("(".to_string()));
    }

    // TODO: this
    let parameters = Vec::new();

    if !matches!(lexer.next(), Token::RightParen(_)) {
        return Err(ParseError::ExpectedToken(")".to_string()));
    }

    // arrow for return type
    if !matches!(lexer.next(), Token::ThinArrow(_)) {
        return Err(ParseError::ExpectedToken("->".to_string()));
    }

    // return type (can currently only be `int`)
    let return_ty = match lexer.next() {
        Token::Ident(ident) => match ident.literal.as_str() {
            "int" => Some(Ty::Int),
            _ => {
                return Err(ParseError::ExpectedToken(
                    "int type is only supported type".to_string(),
                ));
            }
        },
        _ => return Err(ParseError::ExpectedToken("return type".to_string())),
    };

    let body = parse_block(lexer, symbols)?;

    Ok(Function {
        span: span.to(&body),
        name: symbols.get(fn_name.literal),
        parameters,
        return_ty,
        body,
    })
}
