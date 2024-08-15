use std::iter;

use crate::repr::ty::Ty;

use super::*;

pub fn parse_function(c: &mut Compiler, tokens: &mut Lexer<'_>) -> Result<Function, ParseError> {
    // `fn` keyword
    let span_start = match tokens.next_spanned().unwrap() {
        // `fn` keyword
        (Token::Fn, span) => span.start,
        // Some other token
        (token, _) => {
            return Err(ParseError::ExpectedToken {
                expected: Box::new(Token::Fn),
                found: Box::new(token),
                reason: "function declaration must begin with keyword".to_string(),
            });
        }
    };

    // function name
    let fn_name = match tokens.next_token().unwrap() {
        Token::Ident(fn_name) => fn_name,
        token => {
            return Err(ParseError::ExpectedToken {
                expected: Box::new(Token::Ident(String::new())),
                found: Box::new(token),
                reason: "function declaration requires identifier".to_string(),
            });
        }
    };

    // opening paren for argument list
    match tokens.next_token().unwrap() {
        Token::LeftParen => (),
        token => {
            return Err(ParseError::ExpectedToken {
                expected: Box::new(Token::LeftParen),
                found: Box::new(token),
                reason: "argument list must begin with opening parenthesis".to_string(),
            });
        }
    }

    enum ParseState {
        /// Waiting for an item or closing bracket
        Item,
        /// Waiting for a comma
        Comma,
    }
    let mut parse_state = ParseState::Item;

    let parameters = iter::from_fn(|| {
        loop {
            match (&parse_state, tokens.next_token().unwrap()) {
                // Parameter list finished
                (_, Token::RightParen) => {
                    return None;
                }
                // Comma encountered when expected
                (ParseState::Comma, Token::Comma) => {
                    parse_state = ParseState::Item;
                }
                (ParseState::Comma, token) => {
                    return Some(Err(ParseError::ExpectedToken {
                        expected: Box::new(Token::Comma),
                        found: Box::new(token),
                        reason: "function arguments must be separated by a comma".to_string(),
                    }))
                }
                // Parameter item encountered
                (ParseState::Item, Token::Ident(ident)) => {
                    // Intern the parameter identifier
                    let ident = c.intern_string(ident);

                    // Ensure a colon follows it
                    match tokens.next_token().unwrap() {
                        Token::Colon => (),
                        token => {
                            return Some(Err(ParseError::ExpectedToken {
                                expected: Box::new(Token::Colon),
                                found: Box::new(token),
                                reason: "param name and type must be separated by a colon"
                                    .to_string(),
                            }));
                        }
                    }

                    // Extract the type
                    let ty = match tokens.next_token().unwrap() {
                        Token::Int => Ty::Int,
                        Token::Bool => Ty::Boolean,
                        token => {
                            return Some(Err(ParseError::ExpectedToken {
                                // WARN: This should be any of the primitive type tokens
                                expected: Box::new(Token::Int),
                                found: Box::new(token),
                                reason: "parameter must have a type".to_string(),
                            }));
                        }
                    };

                    parse_state = ParseState::Comma;

                    return Some(Ok((ident, ty)));
                }
                (ParseState::Item, token) => {
                    return Some(Err(ParseError::ExpectedToken {
                        expected: Box::new(Token::Ident(String::new())),
                        found: Box::new(token),
                        reason: "parameter must have identifier".to_string(),
                    }))
                }
            }
        }
    })
    .collect::<Result<Vec<_>, _>>()?;

    // arrow for return type
    match tokens.next_token().unwrap() {
        Token::ThinArrow => (),
        token => {
            return Err(ParseError::ExpectedToken {
                expected: Box::new(Token::ThinArrow),
                found: Box::new(token),
                reason: "thin arrow must preceed return type".to_string(),
            });
        }
    }

    // return type
    let return_ty = match tokens.next_token().unwrap() {
        Token::Int => Ty::Int,
        Token::Bool => Ty::Boolean,
        token => {
            return Err(ParseError::ExpectedToken {
                // WARN: This should be any of the primitive type tokens
                expected: Box::new(Token::Int),
                found: Box::new(token),
                reason: "return type must follow thin arrow".to_string(),
            });
        }
    };

    // Parse out the body
    let body = parse_block(c, tokens)?;

    // Construct the function span to the end of the body
    let span = span_start..body.span.end;

    Ok(Function::new(
        c.intern_string(fn_name),
        parameters,
        return_ty,
        body,
        span,
    ))
}
