use super::*;

pub fn parse_array(compiler: &mut Compiler, lexer: &mut Lexer) -> Result<Array, ParseError> {
    // Parse opening square bracket
    let span_start = match lexer.next_spanned().unwrap() {
        (Token::LeftSquare, span) => span.start,
        (token, _) => {
            return Err(ParseError::ExpectedToken {
                expected: Box::new(Token::LeftSquare),
                found: Box::new(token),
                reason: "array literal must start with square brace".to_string(),
            });
        }
    };

    // Parse each of the items, deliminated by a comma
    let mut init = Vec::new();
    let mut expect_item = true;
    let span_end = loop {
        match (lexer.peek_token().unwrap(), expect_item) {
            (Token::Comma, false) => {
                expect_item = true;
                lexer.next_token();
            }
            (Token::RightSquare, _) => {
                break lexer.next_spanned().unwrap().1.end;
            }
            (_, true) => {
                init.push(parse_expression(compiler, lexer, Precedence::Lowest)?);
                expect_item = false;
            }
            (token, _) => {
                return Err(ParseError::ExpectedToken {
                    expected: Box::new(Token::RightSquare),
                    found: Box::new(token.clone()),
                    reason: "expected a comma or closing brace".to_string(),
                });
            }
        }
    };

    Ok(Array {
        init,
        span: span_start..span_end,
        ty_info: None,
    })
}
