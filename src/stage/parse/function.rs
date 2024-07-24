use std::iter;

use crate::repr::ty::Ty;

use super::*;

pub fn parse_function(
    ctx: &mut impl ParseCtx,
    tokens: &mut impl TokenGenerator,
) -> Result<Function, ParseError> {
    // `fn` keyword
    let fn_token = match tokens.next_token() {
        Token::Fn(fn_token) => fn_token,
        token => {
            return Err(ParseError::ExpectedToken {
                expected: Box::new(Token::Fn(FnToken::default())),
                found: Box::new(token),
                reason: "function declaration must begin with keyword".to_string(),
            });
        }
    };

    // function name
    let fn_name = match tokens.next_token() {
        Token::Ident(fn_name) => fn_name,
        token => {
            return Err(ParseError::ExpectedToken {
                expected: Box::new(Token::Ident(IdentToken::default())),
                found: Box::new(token),
                reason: "function declaration requires identifier".to_string(),
            });
        }
    };

    // opening paren for argument list
    match tokens.peek_token() {
        Token::LeftParen(_) => (),
        token => {
            return Err(ParseError::ExpectedToken {
                expected: Box::new(Token::LeftParen(LeftParenToken::default())),
                found: Box::new(token),
                reason: "argument list must begin with opening parenthesis".to_string(),
            });
        }
    }

    let parameters = iter::from_fn(|| {
        match tokens.peek_token() {
            Token::RightParen(_) => None,
            Token::LeftParen(_) | Token::Comma(_) => {
                // Consume the opening paren or comma
                tokens.next_token();

                // If the closing parenthesis is encountered, stop parsing arguments
                if matches!(tokens.peek_token(), Token::RightParen(_)) {
                    return None;
                }

                // Parse the identifier
                let ident = ctx.intern(tokens.ident("param identifier").ok()?.literal);

                let colon = tokens.next_token();
                if !matches!(colon, Token::Colon(_)) {
                    return Some(Err(ParseError::ExpectedToken {
                        expected: Box::new(Token::colon()),
                        found: Box::new(colon),
                        reason: "param name and type must be separated by a colon".to_string(),
                    }));
                }

                let ty_token = tokens.ident("ty").ok()?;
                assert_eq!(ty_token.literal, "int", "only support int type");
                let ty = Ty::Int;

                Some(Ok((ident, ty)))
            }
            token => Some(Err(ParseError::ExpectedToken {
                expected: Box::new(Token::comma()),
                found: Box::new(token),
                reason: "function arguments must be separated by a comma".to_string(),
            })),
        }
    })
    .collect::<Result<Vec<_>, _>>()?;

    // closing paren for argument list
    match tokens.next_token() {
        Token::RightParen(_) => (),
        token => {
            return Err(ParseError::ExpectedToken {
                expected: Box::new(Token::RightParen(RightParenToken::default())),
                found: Box::new(token),
                reason: "argument list must end with closing parenthesis".to_string(),
            });
        }
    }

    // arrow for return type
    match tokens.next_token() {
        Token::ThinArrow(_) => (),
        token => {
            return Err(ParseError::ExpectedToken {
                expected: Box::new(Token::ThinArrow(ThinArrowToken::default())),
                found: Box::new(token),
                reason: "thin arrow must preceed return type".to_string(),
            });
        }
    }

    // return type (can currently only be `int`)
    let return_ty = match tokens.next_token() {
        Token::Ident(ident) => match ident.literal.as_str() {
            "int" => Ty::Int,
            _ => {
                panic!("only int can be returned from a function")
            }
        },
        token => {
            return Err(ParseError::ExpectedToken {
                expected: Box::new(Token::Ident(IdentToken::default())),
                found: Box::new(token),
                reason: "return type must follow thin arrow".to_string(),
            });
        }
    };

    let body = parse_block(ctx, tokens)?;

    let span = fn_token.span.to(&body);
    Ok(Function::new(
        ctx.intern(fn_name.literal),
        parameters,
        return_ty,
        body,
        span,
    ))
}
