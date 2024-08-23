use super::*;

pub fn parse_boolean(
    _compiler: &mut Compiler,
    lexer: &mut Lexer<'_>,
) -> Result<Boolean, ParseError> {
    match lexer.next_spanned().unwrap() {
        (Token::True, span) => Ok(Boolean::new(true, span, Default::default())),
        (Token::False, span) => Ok(Boolean::new(false, span, Default::default())),
        (token, _) => {
            Err(ParseError::ExpectedToken {
                // WARN: Should be true or false
                expected: Box::new(Token::True),
                found: Box::new(token),
                reason: "expected boolean token".to_string(),
            })
        }
    }
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::t_true("true", true)]
    #[case::t_false("false", false)]
    fn success(#[case] source: &str, #[case] value: bool) {
        let boolean = parse_boolean(&mut Compiler::default(), &mut source.into()).unwrap();
        assert_eq!(boolean.value, value);
    }

    #[test]
    fn fail() {
        assert!(parse_boolean(&mut Compiler::default(), &mut "someident".into()).is_err());
    }

    #[rstest]
    #[case::success("true;")]
    #[case::fail("someident;")]
    fn single_token(#[case] source: &str) {
        let mut tokens = source.into();
        let _ = parse_boolean(&mut Compiler::default(), &mut tokens);

        assert_eq!(tokens.count(), 1);
    }
}
