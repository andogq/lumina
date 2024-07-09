use super::*;

pub fn parse_integer(ctx: &mut impl ParseCtxTrait) -> Result<Integer, ParseError> {
    let token = ctx.integer("integer peeked")?;

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

    use rstest::rstest;

    fn run(tokens: &[Token]) -> (SimpleParseCtx, Result<Integer, ParseError>) {
        let mut ctx = SimpleParseCtx::from(tokens);
        let integer = parse_integer(&mut ctx);
        (ctx, integer)
    }

    #[rstest]
    #[case::single_digit(1)]
    #[case::multi_digit(123)]
    fn success(#[case] value: i64) {
        let (_, integer) = run(&[Token::integer(&value.to_string())]);
        assert_eq!(integer.unwrap().value, value);
    }

    #[test]
    fn fail() {
        let (_, integer) = run(&[Token::ident("someident")]);
        assert!(integer.is_err());
    }

    #[rstest]
    #[case::success(Token::integer("1"))]
    #[case::fail(Token::ident("someident"))]
    fn single_token(#[case] token: Token) {
        let (ctx, _) = run(&[token, Token::semicolon()]);
        assert_eq!(ctx.tokens.len(), 1);
    }
}
