use crate::repr::ty::Ty;

use super::{Lexer, ParseError, Span, Token};

pub fn parse_ty(lexer: &mut Lexer) -> Result<(Ty, Span), ParseError> {
    let (token, span) = lexer.next_spanned().ok_or(ParseError::UnexpectedEOF)?;

    Ok((
        match token {
            Token::Int => Ty::Int,
            Token::Uint => Ty::Uint,
            Token::Bool => Ty::Boolean,
            token => {
                return Err(ParseError::ExpectedToken {
                    // WARN: This should be any of the primitive type tokens
                    expected: Box::new(Token::Int),
                    found: Box::new(token),
                    reason: "expected valid type".to_string(),
                });
            }
        },
        span,
    ))
}
