use super::*;

pub fn parse_boolean(
    _ctx: &mut impl ParseCtx,
    tokens: &mut Lexer<'_>,
) -> Result<Boolean, ParseError> {
    match tokens.next_spanned().unwrap() {
        (Token::True, span) => Ok(Boolean::new(true, span)),
        (Token::False, span) => Ok(Boolean::new(false, span)),
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
    use ctx::MockParseCtx;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::t_true("true", true)]
    #[case::t_false("false", false)]
    fn success(#[case] source: &str, #[case] value: bool) {
        let boolean = parse_boolean(&mut MockParseCtx::new(), &mut source.into()).unwrap();
        assert_eq!(boolean.value, value);
    }

    #[test]
    fn fail() {
        assert!(parse_boolean(&mut MockParseCtx::new(), &mut "someident".into()).is_err());
    }

    #[rstest]
    #[case::success("true;")]
    #[case::fail("someident;")]
    fn single_token(#[case] source: &str) {
        let mut tokens = source.into();
        let _ = parse_boolean(&mut MockParseCtx::new(), &mut tokens);

        assert_eq!(tokens.0.count(), 1);
    }
}
