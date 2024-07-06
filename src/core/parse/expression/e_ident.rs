use crate::core::{
    ast::parse_ast::*,
    parse::{ParseCtx, ParseError},
};

pub fn parse_ident(ctx: &mut ParseCtx) -> Result<Ident, ParseError> {
    let token = ctx.lexer.ident("ident peeked")?;

    Ok(Ident::new(
        ctx.symbols.get_or_intern(token.literal),
        token.span,
    ))
}

#[cfg(test)]
mod test {
    use super::{parse_ident, *};
    use crate::core::lexer::{token::Token, Lexer};

    use rstest::rstest;

    #[test]
    fn success() {
        let mut ctx = ParseCtx::new(Lexer::with_tokens(vec![Token::ident("someident")]));

        let ident = parse_ident(&mut ctx).unwrap();
        assert_eq!(ctx.symbols.resolve(ident.name).unwrap(), "someident");
    }

    #[test]
    fn fail() {
        let mut ctx = ParseCtx::new(Lexer::with_tokens(vec![Token::integer("1")]));

        assert!(parse_ident(&mut ctx).is_err());
    }

    #[rstest]
    #[case::success(Token::ident("someident"))]
    #[case::fail(Token::integer("1"))]
    fn single_token(#[case] token: Token) {
        let mut ctx = ParseCtx::new(Lexer::with_tokens(vec![token, Token::semicolon()]));
        let _ = parse_ident(&mut ctx);

        assert_eq!(ctx.lexer.into_iter().count(), 1);
    }
}
