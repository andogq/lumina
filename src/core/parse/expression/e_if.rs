use super::*;
use crate::core::{ast::*, lexer::Lexer, parse::ParseError, symbol::SymbolMap};

pub fn parse_if(lexer: &mut Lexer, symbols: &mut SymbolMap) -> Result<If, ParseError> {
    // Parse out the if keyword
    let token = lexer.t_if("if peeked")?;

    let mut span = token.span;

    let condition = parse_expression(lexer, Precedence::Lowest, symbols)?;

    let success = parse_block(lexer, symbols)?;
    span = span.to(&success);

    let otherwise = if matches!(lexer.peek_token(), Token::Else(_)) {
        lexer.next_token();

        let otherwise = parse_block(lexer, symbols)?;
        span = span.to(&otherwise);

        Some(otherwise)
    } else {
        None
    };

    Ok(If {
        condition: Box::new(condition),
        success,
        otherwise,
        span,
    })
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use super::*;
    use crate::core::lexer::token::Token;

    fn build_if(
        condition: Token,
        body: Token,
        otherwise: Option<Token>,
    ) -> (Lexer, SymbolMap, Result<If, ParseError>) {
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

        let mut lexer = Lexer::with_tokens(tokens);
        let mut symbols = SymbolMap::new();
        let e_if = parse_if(&mut lexer, &mut symbols);

        (lexer, symbols, e_if)
    }

    #[test]
    fn integer_condition() {
        let (_lexer, _symbols, e_if) = build_if(Token::integer("123"), Token::integer("1"), None);
        let e_if = e_if.unwrap();

        assert!(matches!(
            *e_if.condition,
            Expression::Integer(Integer { value: 123, .. })
        ));
    }

    #[test]
    fn ident_condition() {
        let (_lexer, _symbols, e_if) =
            build_if(Token::ident("someident"), Token::integer("1"), None);
        let e_if = e_if.unwrap();

        assert!(matches!(*e_if.condition, Expression::Ident(_)));
    }

    #[test]
    fn otherwise_branch() {
        let (_lexer, _symbols, e_if) = build_if(
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
        let mut lexer = Lexer::with_tokens(tokens);
        let result = parse_if(&mut lexer, &mut SymbolMap::new());

        assert!(result.is_err());
    }
}
