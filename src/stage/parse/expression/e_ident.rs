use super::*;

pub fn parse_ident(ctx: &mut impl ParseCtx, tokens: &mut Lexer<'_>) -> Result<Ident, ParseError> {
    match tokens.next_spanned().unwrap() {
        (Token::Ident(ident), span) => Ok(Ident::new(ctx.intern(ident), span)),
        (token, _) => Err(ParseError::ExpectedToken {
            expected: Box::new(Token::Ident(String::new())),
            found: Box::new(token),
            reason: "expected ident".to_string(),
        }),
    }
}

#[cfg(test)]
mod test {
    use crate::util::symbol_map::interner_symbol_map::Symbol;

    use super::{parse_ident, *};

    use ctx::MockParseCtx;
    use rstest::rstest;
    use string_interner::Symbol as _;

    #[test]
    fn success() {
        let mut tokens = "someident".into();

        let mocked_symbol = Symbol::try_from_usize(0).unwrap();

        let mut ctx = MockParseCtx::new();
        ctx.expect_intern()
            .once()
            .withf(|s| s.as_ref() == "someident")
            .return_const(mocked_symbol);

        let ident = parse_ident(&mut ctx, &mut tokens).unwrap();
        assert_eq!(ident.binding, mocked_symbol);
    }

    #[test]
    fn fail() {
        assert!(parse_ident(&mut MockParseCtx::new(), &mut "1".into(),).is_err());
    }

    #[rstest]
    #[case::success("someident;")]
    #[case::fail("1;")]
    fn single_token(#[case] source: &str) {
        let mut ctx = MockParseCtx::new();
        ctx.expect_intern()
            .times(0..=1)
            .withf(|s| s.as_ref() == "someident")
            .return_const(Symbol::try_from_usize(0).unwrap());

        let mut tokens = source.into();
        let _ = parse_ident(&mut ctx, &mut tokens);

        assert_eq!(tokens.0.count(), 1);
    }
}
