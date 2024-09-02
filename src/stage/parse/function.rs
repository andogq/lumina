use std::iter;

use ty::parse_ty;

use super::*;

pub fn parse_function(
    parser: &Parser,
    compiler: &mut Compiler,
    lexer: &mut Lexer<'_>,
) -> Result<Function, ParseError> {
    // `fn` keyword
    let span_start = match lexer.next_spanned().unwrap() {
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
    let fn_name = match lexer.next_token().unwrap() {
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
    match lexer.next_token().unwrap() {
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
            match (&parse_state, lexer.next_token().unwrap()) {
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
                    let ident = compiler.symbols.get_or_intern(ident);

                    // Ensure a colon follows it
                    match lexer.next_token().unwrap() {
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
                    let (ty, _) = match parse_ty(lexer) {
                        Ok(ty) => ty,
                        Err(e) => {
                            return Some(Err(e));
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
    match lexer.next_token().unwrap() {
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
    let (return_ty, _) = parse_ty(lexer)?;

    // Parse out the body
    let Expression::<UntypedAstMetadata>::Block(body) =
        parser.parse(compiler, lexer, Precedence::Lowest)?
    else {
        return Err(ParseError::ExpectedBlock);
    };

    // Construct the function span to the end of the body
    let span = span_start..body.span.end;

    Ok(Function::new(
        compiler.symbols.get_or_intern(fn_name),
        parameters,
        return_ty,
        body,
        span,
    ))
}
