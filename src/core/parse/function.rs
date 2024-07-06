use super::*;

pub fn parse_function(ctx: &mut ParseCtx) -> Result<Function, ParseError> {
    // `fn` keyword
    let fn_token = match ctx.lexer.next_token() {
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
    let fn_name = match ctx.lexer.next_token() {
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
    match ctx.lexer.next_token() {
        Token::LeftParen(_) => (),
        token => {
            return Err(ParseError::ExpectedToken {
                expected: Box::new(Token::LeftParen(LeftParenToken::default())),
                found: Box::new(token),
                reason: "argument list must begin with opening parenthesis".to_string(),
            });
        }
    }

    // TODO: this
    let parameters = Vec::new();

    // closing paren for argument list
    match ctx.lexer.next_token() {
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
    match ctx.lexer.next_token() {
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
    let return_ty = match ctx.lexer.next_token() {
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

    let body = parse_block(ctx)?;

    let span = fn_token.span.to(&body);
    Ok(Function::new(
        ctx.symbols.get_or_intern(fn_name.literal),
        parameters,
        return_ty,
        body,
        span,
    ))
}
