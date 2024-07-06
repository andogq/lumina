use super::*;
use crate::core::{ast::*, parse::ParseError};

pub fn parse_if(ctx: &mut ParseCtx) -> Result<If<()>, ParseError> {
    // Parse out the if keyword
    let token = ctx.lexer.t_if("if peeked")?;

    let mut span = token.span;

    let condition = parse_expression(ctx, Precedence::Lowest)?;

    let success = parse_block(ctx)?;
    span = span.to(&success);

    let otherwise = if matches!(ctx.lexer.peek_token(), Token::Else(_)) {
        ctx.lexer.next_token();

        let otherwise = parse_block(ctx)?;
        span = span.to(&otherwise);

        Some(otherwise)
    } else {
        None
    };

    Ok(If::new(Box::new(condition), success, otherwise, span))
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use super::*;
    use crate::core::lexer::{token::Token, Lexer};

    fn build_if(
        condition: Token,
        body: Token,
        otherwise: Option<Token>,
    ) -> (ParseCtx, Result<If<()>, ParseError>) {
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

        let lexer = Lexer::with_tokens(tokens);
        let mut ctx = ParseCtx::new(lexer);
        let e_if = parse_if(&mut ctx);

        (ctx, e_if)
    }

    #[test]
    fn integer_condition() {
        let (_, e_if) = build_if(Token::integer("123"), Token::integer("1"), None);
        let e_if = e_if.unwrap();

        assert!(matches!(
            *e_if.condition,
            Expression::Integer(Integer { value: 123, .. })
        ));
    }

    #[test]
    fn ident_condition() {
        let (_, e_if) = build_if(Token::ident("someident"), Token::integer("1"), None);
        let e_if = e_if.unwrap();

        assert!(matches!(*e_if.condition, Expression::Ident(_)));
    }

    #[test]
    fn otherwise_branch() {
        let (_, e_if) = build_if(
            Token::ident("someident"),
            Token::integer("1"),
            Some(Token::integer("2")),
        );
        let e_if = e_if.unwrap();

        assert!(e_if.otherwise.is_some());
    }

    #[rstest]
    #[case::multiple_condition_tokens(vec![
        Token::t_if(),
        Token::integer("1"),
        Token::integer("2"),
    ])]
    #[case::malformed_otherwise_block(vec![
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
    fn fail(#[case] tokens: Vec<Token>) {
        let lexer = Lexer::with_tokens(tokens);
        let result = parse_if(&mut ParseCtx::new(lexer));

        assert!(result.is_err());
    }
}
