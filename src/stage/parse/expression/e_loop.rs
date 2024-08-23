use crate::{
    repr::{ast::untyped::Loop, token::Token},
    stage::parse::parse_block,
};

use super::{Compiler, Lexer, ParseError};

pub fn parse_loop(compiler: &mut Compiler, lexer: &mut Lexer<'_>) -> Result<Loop, ParseError> {
    let span_start = match lexer.next_spanned().unwrap() {
        (Token::Loop, span) => span.start,
        (token, _) => {
            return Err(ParseError::ExpectedToken {
                expected: Box::new(Token::If),
                found: Box::new(token),
                reason: "loop must start with keyword".to_string(),
            });
        }
    };

    let body = parse_block(compiler, lexer)?;

    Ok(Loop {
        span: span_start..body.span.end,
        body,
        ty_info: None,
    })
}
