use super::*;

pub fn parse_block(compiler: &mut Compiler, lexer: &mut Lexer<'_>) -> Result<Block, ParseError> {
    let span_start = match lexer.next_spanned().unwrap() {
        (Token::LeftBrace, span) => span.start,
        (token, _) => {
            return Err(ParseError::ExpectedToken {
                expected: Box::new(Token::LeftBrace),
                found: Box::new(token),
                reason: "block must begin with an opening brace".to_string(),
            });
        }
    };

    let statements = std::iter::from_fn(|| match lexer.peek_token().unwrap() {
        Token::RightBrace => None,
        _ => Some(parse_statement(compiler, lexer)),
    })
    .collect::<Result<Vec<_>, _>>()?;

    // Consume the right brace that just stopped us
    let span_end = match lexer.next_spanned().unwrap() {
        (Token::RightBrace, span) => span.end,
        (token, _) => {
            return Err(ParseError::ExpectedToken {
                expected: Box::new(Token::RightBrace),
                found: Box::new(token),
                reason: "block must end with a closing brace".to_string(),
            });
        }
    };

    let span = span_start..span_end;

    Ok(Block::new(statements, span, Default::default()))
}
