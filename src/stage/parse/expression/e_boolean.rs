use super::*;

pub fn parse_boolean(
    _ctx: &mut impl SymbolMapTrait,
    tokens: &mut impl TokenGenerator,
) -> Result<Boolean, ParseError> {
    let token = tokens.boolean("boolean peeked")?;

    let (span, value) = match token {
        BooleanToken::True(token) => (token.span, true),
        BooleanToken::False(token) => (token.span, false),
    };

    Ok(Boolean::new(value, span))
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use crate::util::symbol_map::SymbolMap;

    use super::*;

    #[rstest]
    #[case::t_true(Token::t_true(), true)]
    #[case::t_false(Token::t_false(), false)]
    fn success(#[case] token: Token, #[case] value: bool) {
        let mut ctx = SymbolMap::default();
        let boolean = parse_boolean(&mut ctx, &mut [token].into_iter().peekable()).unwrap();
        assert_eq!(boolean.value, value);
    }

    #[test]
    fn fail() {
        let mut ctx = SymbolMap::default();
        assert!(parse_boolean(
            &mut ctx,
            &mut [Token::ident("someident")].into_iter().peekable()
        )
        .is_err());
    }

    #[rstest]
    #[case::success(Token::t_true())]
    #[case::fail(Token::ident("someident"))]
    fn single_token(#[case] token: Token) {
        let mut tokens = [token, Token::semicolon()].into_iter().peekable();
        let mut ctx = SymbolMap::default();
        let _ = parse_boolean(&mut ctx, &mut tokens);

        assert_eq!(tokens.len(), 1);
    }
}
