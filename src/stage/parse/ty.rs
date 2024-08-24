use crate::repr::ty::Ty;

use super::{Lexer, ParseError, Token};

pub fn parse_ty(lexer: &mut Lexer) -> Result<Ty, ParseError> {
    Ok(match lexer.next_token().unwrap() {
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
    })
}
