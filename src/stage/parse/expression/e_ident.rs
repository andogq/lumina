use super::*;

pub fn parse_ident(
    ctx: &mut impl SymbolMapTrait,
    tokens: &mut impl TokenGenerator,
) -> Result<Ident, ParseError> {
    let token = tokens.ident("ident peeked")?;

    Ok(Ident::new(ctx.intern(token.literal), token.span))
}

#[cfg(test)]
mod test {
    use crate::util::symbol_map::SymbolMap;

    use super::{parse_ident, *};

    use rstest::rstest;

    #[test]
    fn success() {
        let tokens = [Token::ident("someident")];
        let mut ctx = SymbolMap::default();

        let ident = parse_ident(&mut ctx, &mut tokens.into_iter().peekable()).unwrap();
        assert_eq!(SymbolMapTrait::get(&ctx, ident.name), "someident");
    }

    #[test]
    fn fail() {
        let mut ctx = SymbolMap::default();

        assert!(parse_ident(&mut ctx, &mut [Token::integer("1")].into_iter().peekable()).is_err());
    }

    #[rstest]
    #[case::success(Token::ident("someident"))]
    #[case::fail(Token::integer("1"))]
    fn single_token(#[case] token: Token) {
        let mut tokens = [token, Token::semicolon()].into_iter().peekable();
        let mut ctx = SymbolMap::default();
        let _ = parse_ident(&mut ctx, &mut tokens);

        assert_eq!(tokens.len(), 1);
    }
}
