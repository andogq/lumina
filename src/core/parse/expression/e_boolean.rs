use crate::core::{
    ast::*,
    parse::{BooleanToken, ParseCtx, ParseError},
};

pub fn parse_boolean(ctx: &mut ParseCtx) -> Result<Boolean, ParseError> {
    let token = ctx.lexer.boolean("boolean peeked")?;

    let (span, value) = match token {
        BooleanToken::True(token) => (token.span, true),
        BooleanToken::False(token) => (token.span, false),
    };

    Ok(Boolean { span, value })
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use crate::core::lexer::{token::Token, Lexer};

    use super::*;

    #[rstest]
    #[case::t_true(Token::t_true(), true)]
    #[case::t_false(Token::t_false(), false)]
    fn success(#[case] token: Token, #[case] value: bool) {
        use crate::core::lexer::Lexer;

        let mut ctx = ParseCtx::new(Lexer::with_tokens(vec![token]));

        let boolean = parse_boolean(&mut ctx).unwrap();
        assert_eq!(boolean.value, value);
    }

    #[test]
    fn fail() {
        let mut ctx = ParseCtx::new(Lexer::with_tokens(vec![Token::ident("someident")]));
        assert!(parse_boolean(&mut ctx).is_err());
    }

    #[rstest]
    #[case::success(Token::t_true())]
    #[case::fail(Token::ident("someident"))]
    fn single_token(#[case] token: Token) {
        let mut ctx = ParseCtx::new(Lexer::with_tokens(vec![token, Token::semicolon()]));
        let _ = parse_boolean(&mut ctx);

        assert_eq!(ctx.lexer.into_iter().count(), 1);
    }
}
