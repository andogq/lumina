use super::*;

pub fn parse_integer(
    _compiler: &mut Compiler,
    tokens: &mut Lexer<'_>,
) -> Result<Integer, ParseError> {
    match tokens.next_spanned().unwrap() {
        (Token::Integer(value), span) => Ok(Integer::new(value, span)),
        (token, _) => Err(ParseError::ExpectedToken {
            expected: Box::new(Token::Integer(0)),
            found: Box::new(token),
            reason: "integer token expected".to_string(),
        }),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use rstest::rstest;

    fn run(source: &str) -> Result<Integer, ParseError> {
        parse_integer(&mut Compiler::default(), &mut source.into())
    }

    #[rstest]
    #[case::single_digit(1)]
    #[case::multi_digit(123)]
    fn success(#[case] value: i64) {
        let integer = run(&value.to_string());
        assert_eq!(integer.unwrap().value, value);
    }

    #[test]
    fn fail() {
        let integer = run("someident");
        assert!(integer.is_err());
    }

    #[rstest]
    #[case::success("1;")]
    #[case::fail("someident;")]
    fn single_token(#[case] source: &str) {
        let mut tokens = source.into();
        let _ = parse_integer(&mut Compiler::default(), &mut tokens);
        assert_eq!(tokens.0.count(), 1);
    }
}
