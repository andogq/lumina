use super::*;

pub fn parse_ident(
    ctx: &mut impl ParseCtx,
    tokens: &mut impl TokenGenerator,
) -> Result<Ident, ParseError> {
    let token = tokens.ident("ident peeked")?;

    Ok(Ident::new(ctx.intern(token.literal), token.span))
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
        let tokens = [Token::ident("someident")];

        let mocked_symbol = Symbol::try_from_usize(0).unwrap();

        let mut ctx = MockParseCtx::new();
        ctx.expect_intern()
            .once()
            .withf(|s| s.as_ref() == "someident")
            .return_const(mocked_symbol);

        let ident = parse_ident(&mut ctx, &mut tokens.into_iter().peekable()).unwrap();
        assert_eq!(ident.name, mocked_symbol);
    }

    #[test]
    fn fail() {
        assert!(parse_ident(
            &mut MockParseCtx::new(),
            &mut [Token::integer("1")].into_iter().peekable()
        )
        .is_err());
    }

    #[rstest]
    #[case::success(Token::ident("someident"))]
    #[case::fail(Token::integer("1"))]
    fn single_token(#[case] token: Token) {
        let mut ctx = MockParseCtx::new();
        ctx.expect_intern()
            .times(0..=1)
            .withf(|s| s.as_ref() == "someident")
            .return_const(Symbol::try_from_usize(0).unwrap());

        let mut tokens = [token, Token::semicolon()].into_iter().peekable();
        let _ = parse_ident(&mut ctx, &mut tokens);

        assert_eq!(tokens.len(), 1);
    }
}
