use crate::core::{
    ast::*,
    lexer::Lexer,
    parse::{BooleanToken, ParseError},
};

pub fn parse_boolean(lexer: &mut Lexer) -> Result<Boolean, ParseError> {
    let token = lexer.boolean("boolean peeked")?;

    let (span, value) = match token {
        BooleanToken::True(token) => (token.span, true),
        BooleanToken::False(token) => (token.span, false),
    };

    Ok(Boolean { span, value })
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
        let mut lexer = Lexer::with_tokens(vec![token]);

        let boolean = parse_boolean(&mut lexer).unwrap();
        assert_eq!(boolean.value, value);
    }

    #[test]
    fn fail() {
        let mut lexer = Lexer::with_tokens(vec![Token::ident("someident")]);
        assert!(parse_boolean(&mut lexer).is_err());
    }

    #[rstest]
    #[case::success(Token::t_true())]
    #[case::fail(Token::ident("someident"))]
    fn single_token(#[case] token: Token) {
        let mut lexer = Lexer::with_tokens(vec![token, Token::semicolon()]);
        let _ = parse_boolean(&mut lexer);

        assert_eq!(lexer.into_iter().count(), 1);
    }
}
