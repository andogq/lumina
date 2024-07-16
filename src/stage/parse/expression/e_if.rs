use super::*;

pub fn parse_if(
    ctx: &mut impl ParseCtx,
    tokens: &mut impl TokenGenerator,
) -> Result<If, ParseError> {
    // Parse out the if keyword
    let token = tokens.t_if("if peeked")?;

    let mut span = token.span;

    let condition = parse_expression(ctx, tokens, Precedence::Lowest)?;

    let success = parse_block(ctx, tokens)?;
    span = span.to(&success);

    let otherwise = if matches!(tokens.peek_token(), Token::Else(_)) {
        tokens.next_token();

        let otherwise = parse_block(ctx, tokens)?;
        span = span.to(&otherwise);

        Some(otherwise)
    } else {
        None
    };

    Ok(If::new(Box::new(condition), success, otherwise, span))
}

#[cfg(test)]
mod test {
    use ctx::MockParseCtx;
    use rstest::rstest;
    use string_interner::Symbol;

    use super::*;

    fn build_if(condition: Token, body: Token, otherwise: Option<Token>) -> Vec<Token> {
        // Build up the if statement
        let mut tokens = vec![
            Token::t_if(),
            condition,
            Token::left_brace(),
            body,
            Token::right_brace(),
        ];

        // Build up the otherwise branch, if present
        if let Some(otherwise) = otherwise {
            tokens.extend([
                Token::t_else(),
                Token::left_brace(),
                otherwise,
                Token::right_brace(),
            ]);
        }

        tokens
    }

    #[test]
    fn integer_condition() {
        let tokens = build_if(Token::integer("123"), Token::integer("1"), None);

        let mut ctx = MockParseCtx::new();
        let e_if = parse_if(&mut ctx, &mut tokens.into_iter().peekable()).unwrap();

        assert!(matches!(
            *e_if.condition,
            Expression::Integer(Integer { value: 123, .. })
        ));
    }

    #[test]
    fn ident_condition() {
        let tokens = build_if(Token::ident("someident"), Token::integer("1"), None);

        let mut ctx = MockParseCtx::new();

        ctx.expect_intern()
            .once()
            .return_const::<crate::ctx::Symbol>(Symbol::try_from_usize(0).unwrap());

        let e_if = parse_if(&mut ctx, &mut tokens.into_iter().peekable()).unwrap();

        assert!(matches!(*e_if.condition, Expression::Ident(_)));
    }

    #[test]
    fn otherwise_branch() {
        let tokens = build_if(
            Token::ident("someident"),
            Token::integer("1"),
            Some(Token::integer("2")),
        );

        let mut ctx = MockParseCtx::new();

        ctx.expect_intern()
            .once()
            .return_const::<crate::ctx::Symbol>(Symbol::try_from_usize(0).unwrap());

        let e_if = parse_if(&mut ctx, &mut tokens.into_iter().peekable()).unwrap();

        assert!(e_if.otherwise.is_some());
    }

    #[rstest]
    #[case::multiple_condition_tokens(&[
        Token::t_if(),
        Token::integer("1"),
        Token::integer("2"),
    ])]
    #[case::malformed_otherwise_block(&[
        Token::t_if(),
        Token::integer("1"),
        Token::left_brace(),
        Token::integer("3"),
        Token::right_brace(),
        Token::t_else(),
        Token::t_else(), // Two else keywords
        Token::left_brace(),
        Token::integer("3"),
        Token::right_brace(),
    ])]
    fn fail(#[case] tokens: &[Token]) {
        let result = parse_if(
            &mut MockParseCtx::new(),
            &mut tokens.iter().cloned().peekable(),
        );

        assert!(result.is_err());
    }
}
