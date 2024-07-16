use super::*;

pub fn parse_integer(
    _ctx: &mut impl ParseCtx,
    tokens: &mut impl TokenGenerator,
) -> Result<Integer, ParseError> {
    let token = tokens.integer("integer peeked")?;

    Ok(Integer::new(
        token
            .literal
            .parse()
            .map_err(|_| ParseError::InvalidLiteral {
                expected: "integer".to_string(),
            })?,
        token.span,
    ))
}

#[cfg(test)]
mod test {
    use super::*;

    use ctx::MockParseCtx;
    use rstest::rstest;

    fn run(tokens: &[Token]) -> Result<Integer, ParseError> {
        let integer = parse_integer(
            &mut MockParseCtx::new(),
            &mut tokens.iter().cloned().peekable(),
        );
        integer
    }

    #[rstest]
    #[case::single_digit(1)]
    #[case::multi_digit(123)]
    fn success(#[case] value: i64) {
        let integer = run(&[Token::integer(&value.to_string())]);
        assert_eq!(integer.unwrap().value, value);
    }

    #[test]
    fn fail() {
        let integer = run(&[Token::ident("someident")]);
        assert!(integer.is_err());
    }

    #[rstest]
    #[case::success(Token::integer("1"))]
    #[case::fail(Token::ident("someident"))]
    fn single_token(#[case] token: Token) {
        let mut tokens = [token, Token::semicolon()].into_iter().peekable();
        let _ = parse_integer(&mut MockParseCtx::new(), &mut tokens);
        assert_eq!(tokens.len(), 1);
    }
}
