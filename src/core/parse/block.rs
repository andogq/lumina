use super::*;

pub fn parse_block(ctx: &mut impl ParseCtxTrait) -> Result<Block, ParseError> {
    let open_brace = match ctx.next_token() {
        Token::LeftBrace(ident) => ident,
        token => {
            return Err(ParseError::ExpectedToken {
                expected: Box::new(Token::LeftBrace(LeftBraceToken::default())),
                found: Box::new(token),
                reason: "block must begin with an opening brace".to_string(),
            });
        }
    };

    let statements = std::iter::from_fn(|| {
        if !matches!(ctx.peek_token(), Token::RightBrace(_)) {
            Some(parse_statement(ctx))
        } else {
            None
        }
    })
    .collect::<Result<Vec<_>, _>>()?;

    // Consume the right brace that just stopped us
    let close_brace = match ctx.next_token() {
        Token::RightBrace(ident) => ident,
        token => {
            return Err(ParseError::ExpectedToken {
                expected: Box::new(Token::RightBrace(RightBraceToken::default())),
                found: Box::new(token),
                reason: "block must end with a closing brace".to_string(),
            });
        }
    };

    Ok(Block::new(statements, open_brace.span().to(&close_brace)))
}
