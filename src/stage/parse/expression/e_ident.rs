use super::*;

pub fn parse_ident(compiler: &mut Compiler, tokens: &mut Lexer<'_>) -> Result<Ident, ParseError> {
    match tokens.next_spanned().unwrap() {
        (Token::Ident(ident), span) => Ok(Ident::new(
            compiler.symbols.get_or_intern(ident),
            span,
            Default::default(),
        )),
        (token, _) => Err(ParseError::ExpectedToken {
            expected: Box::new(Token::Ident(String::new())),
            found: Box::new(token),
            reason: "expected ident".to_string(),
        }),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use rstest::rstest;

    #[test]
    fn success() {
        let mut tokens = "someident".into();

        let mut compiler = Compiler::default();

        let ident = parse_ident(&mut compiler, &mut tokens).unwrap();

        assert_eq!(
            compiler.symbols.resolve(ident.binding).unwrap(),
            "someident"
        );
    }

    #[test]
    fn fail() {
        assert!(parse_ident(&mut Compiler::default(), &mut "1".into(),).is_err());
    }

    #[rstest]
    #[case::success("someident;")]
    #[case::fail("1;")]
    fn single_token(#[case] source: &str) {
        let mut tokens = source.into();
        let _ = parse_ident(&mut Compiler::default(), &mut tokens);

        assert_eq!(tokens.count(), 1);
    }
}
