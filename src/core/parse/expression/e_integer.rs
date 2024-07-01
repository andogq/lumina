use crate::core::{ast::*, lexer::Lexer, parse::ParseError};

pub fn parse_integer(lexer: &mut Lexer) -> Result<Integer, ParseError> {
    let token = lexer.integer("integer peeked")?;

    Ok(Integer {
        span: token.span,
        value: token
            .literal
            .parse()
            .map_err(|_| ParseError::InvalidLiteral {
                expected: "integer".to_string(),
            })?,
    })
}

#[cfg(test)]
mod test {
    use super::{parse_integer, *};
    use crate::core::lexer::token::Token;

    use rstest::rstest;

    #[rstest]
    #[case::single_digit(1)]
    #[case::multi_digit(123)]
    fn success(#[case] value: i64) {
        let mut lexer = Lexer::with_tokens(vec![Token::integer(&value.to_string())]);

        let integer = parse_integer(&mut lexer).unwrap();
        assert_eq!(integer.value, value);
    }

    #[test]
    fn fail() {
        let mut lexer = Lexer::with_tokens(vec![Token::ident("someident")]);

        assert!(parse_integer(&mut lexer).is_err());
    }

    #[rstest]
    #[case::success(Token::integer("1"))]
    #[case::fail(Token::ident("someident"))]
    fn single_token(#[case] token: Token) {
        let mut lexer = Lexer::with_tokens(vec![token, Token::semicolon()]);
        let _ = parse_integer(&mut lexer);

        assert_eq!(lexer.into_iter().count(), 1);
    }
}
