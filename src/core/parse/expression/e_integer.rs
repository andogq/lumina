use crate::core::{
    ast::*,
    parse::{ParseCtx, ParseError},
};

pub fn parse_integer(ctx: &mut ParseCtx) -> Result<Integer<()>, ParseError> {
    let token = ctx.lexer.integer("integer peeked")?;

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
    use crate::core::lexer::{token::Token, Lexer};

    use rstest::rstest;

    fn run(tokens: Vec<Token>) -> (ParseCtx, Result<Integer<()>, ParseError>) {
        let lexer = Lexer::with_tokens(tokens);
        let mut ctx = ParseCtx::new(lexer);
        let integer = parse_integer(&mut ctx);
        (ctx, integer)
    }

    #[rstest]
    #[case::single_digit(1)]
    #[case::multi_digit(123)]
    fn success(#[case] value: i64) {
        let (_, integer) = run(vec![Token::integer(&value.to_string())]);
        assert_eq!(integer.unwrap().value, value);
    }

    #[test]
    fn fail() {
        let (_, integer) = run(vec![Token::ident("someident")]);
        assert!(integer.is_err());
    }

    #[rstest]
    #[case::success(Token::integer("1"))]
    #[case::fail(Token::ident("someident"))]
    fn single_token(#[case] token: Token) {
        let (ctx, _) = run(vec![token, Token::semicolon()]);
        assert_eq!(ctx.lexer.into_iter().count(), 1);
    }
}
