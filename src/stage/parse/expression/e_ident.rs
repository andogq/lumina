use super::*;

pub fn parse_ident(ctx: &mut impl ParseCtxTrait) -> Result<Ident, ParseError> {
    let token = ctx.ident("ident peeked")?;

    Ok(Ident::new(ctx.intern(token.literal), token.span))
}

#[cfg(test)]
mod test {
    use super::{parse_ident, *};

    use rstest::rstest;

    #[test]
    fn success() {
        let mut ctx = SimpleParseCtx::from([Token::ident("someident")].as_slice());

        let ident = parse_ident(&mut ctx).unwrap();
        assert_eq!(ctx.get(ident.name), "someident");
    }

    #[test]
    fn fail() {
        let mut ctx = SimpleParseCtx::from([Token::integer("1")].as_slice());

        assert!(parse_ident(&mut ctx).is_err());
    }

    #[rstest]
    #[case::success(Token::ident("someident"))]
    #[case::fail(Token::integer("1"))]
    fn single_token(#[case] token: Token) {
        let mut ctx = SimpleParseCtx::from([token, Token::semicolon()].as_slice());
        let _ = parse_ident(&mut ctx);

        assert_eq!(ctx.tokens.len(), 1);
    }
}
