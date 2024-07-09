use super::*;

pub fn parse_boolean(ctx: &mut impl ParseCtxTrait) -> Result<Boolean, ParseError> {
    let token = ctx.boolean("boolean peeked")?;

    let (span, value) = match token {
        BooleanToken::True(token) => (token.span, true),
        BooleanToken::False(token) => (token.span, false),
    };

    Ok(Boolean::new(value, span))
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use crate::core::lexer::token::Token;

    use super::*;

    #[rstest]
    #[case::t_true(Token::t_true(), true)]
    #[case::t_false(Token::t_false(), false)]
    fn success(#[case] token: Token, #[case] value: bool) {
        let mut ctx = SimpleParseCtx::from([token].as_slice());
        let boolean = parse_boolean(&mut ctx).unwrap();
        assert_eq!(boolean.value, value);
    }

    #[test]
    fn fail() {
        let mut ctx = SimpleParseCtx::from([Token::ident("someident")].as_slice());
        assert!(parse_boolean(&mut ctx).is_err());
    }

    #[rstest]
    #[case::success(Token::t_true())]
    #[case::fail(Token::ident("someident"))]
    fn single_token(#[case] token: Token) {
        let mut ctx = SimpleParseCtx::from([token, Token::semicolon()].as_slice());
        let _ = parse_boolean(&mut ctx);

        assert_eq!(ctx.tokens.len(), 1);
    }
}
