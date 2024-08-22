use super::*;

pub fn parse_if(compiler: &mut Compiler, tokens: &mut Lexer<'_>) -> Result<If, ParseError> {
    let span_start = match tokens.next_spanned().unwrap() {
        (Token::If, span) => span.start,
        (token, _) => {
            return Err(ParseError::ExpectedToken {
                expected: Box::new(Token::If),
                found: Box::new(token),
                reason: "if statement expected".to_string(),
            });
        }
    };

    let condition = parse_expression(compiler, tokens, Precedence::Lowest)?;

    let success = parse_block(compiler, tokens)?;

    let otherwise = if matches!(tokens.peek_token(), Some(Token::Else)) {
        tokens.next_token().unwrap();

        let otherwise = parse_block(compiler, tokens)?;

        Some(otherwise)
    } else {
        None
    };

    let span_end = otherwise
        .as_ref()
        .map(|otherwise| otherwise.span.end)
        .unwrap_or(success.span.end);

    Ok(If::new(
        Box::new(condition),
        success,
        otherwise,
        span_start..span_end,
        Default::default(),
    ))
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use super::*;

    #[test]
    fn integer_condition() {
        let mut tokens = "if 123 { 1 }".into();

        let e_if = parse_if(&mut Compiler::default(), &mut tokens).unwrap();

        assert!(matches!(
            *e_if.condition,
            Expression::Integer(Integer { value: 123, .. })
        ));
    }

    #[test]
    fn ident_condition() {
        let mut tokens = "if someident { 1 }".into();

        let e_if = parse_if(&mut Compiler::default(), &mut tokens).unwrap();

        assert!(matches!(*e_if.condition, Expression::Ident(_)));
    }

    #[test]
    fn otherwise_branch() {
        let mut tokens = "if someident { 1 } else { 2 }".into();

        let e_if = parse_if(&mut Compiler::default(), &mut tokens).unwrap();

        assert!(e_if.otherwise.is_some());
    }

    #[rstest]
    #[case::multiple_condition_tokens("if 1 2")]
    #[case::malformed_otherwise_block("if 1 { 3 } else else { 3 }")]
    fn fail(#[case] source: &str) {
        let result = parse_if(&mut Compiler::default(), &mut source.into());

        assert!(result.is_err());
    }
}
