use super::*;

pub fn parse_statement(
    compiler: &mut Compiler,
    tokens: &mut Lexer<'_>,
) -> Result<Statement, ParseError> {
    let mut expecting_semicolon = true;

    let statement = match tokens.peek_token().unwrap() {
        Token::Return => {
            // Parse as return statement
            let (_, return_span) = tokens.next_spanned().unwrap();

            // Parse out the value
            let value = parse_expression(compiler, tokens, Precedence::Lowest)?;

            // Build the span
            let span = return_span.start..value.span().end;

            Statement::Return(ReturnStatement::new(value, span))
        }
        Token::Let => {
            // let token
            let (_, let_span) = tokens.next_spanned().unwrap();

            // variable binding
            let name = match tokens.next_token().unwrap() {
                Token::Ident(name) => name,
                token => {
                    return Err(ParseError::ExpectedToken {
                        found: Box::new(token),
                        expected: Box::new(Token::Ident(Default::default())),
                        reason: "ident must follow let binding".to_string(),
                    });
                }
            };

            // equals sign
            match tokens.next_token().unwrap() {
                Token::Eq => (),
                token => {
                    return Err(ParseError::ExpectedToken {
                        found: Box::new(token),
                        expected: Box::new(Token::Eq),
                        reason: "equals sign must follow ident".to_string(),
                    });
                }
            };

            // value
            let value = parse_expression(compiler, tokens, Precedence::Lowest)?;
            let span = let_span.start..value.span().end;

            Statement::Let(LetStatement::new(compiler.intern_string(name), value, span))
        }
        _ => {
            // Parse expression
            let expression = parse_expression(compiler, tokens, Precedence::Lowest)?;
            let span = expression.span().clone();

            Statement::Expression(ExpressionStatement::new(
                expression,
                if matches!(tokens.peek_token().unwrap(), Token::SemiColon) {
                    false
                } else {
                    expecting_semicolon = false;

                    true
                },
                span,
            ))
        }
    };

    if expecting_semicolon {
        match tokens.next_token().unwrap() {
            Token::SemiColon => (),
            token => {
                return Err(ParseError::ExpectedToken {
                    found: Box::new(token),
                    expected: Box::new(Token::Eq),
                    reason: "semicolon must follow statement".to_string(),
                });
            }
        };
    }

    Ok(statement)
}
