use std::fmt::{Display, Formatter};

use crate::{
    ast::{Expression, ParseNode},
    lexer::Lexer,
    token::{ReturnToken, Token},
};

#[derive(Clone, Debug, PartialEq)]
pub struct ReturnStatement {
    pub return_token: ReturnToken,
    pub value: Expression,
}

impl<S> ParseNode<S> for ReturnStatement
where
    S: Iterator<Item = char>,
{
    fn parse(tokens: &mut Lexer<S>) -> Result<Self, String> {
        let Token::Return(return_token) = tokens.next() else {
            return Err("expected `return` token".to_string());
        };

        let value = Expression::parse(tokens)?;

        let Token::Semicolon(_semicolon_token) = tokens.next() else {
            return Err("expected `semicolon` token".to_string());
        };

        Ok(ReturnStatement {
            return_token,
            value,
        })
    }
}

impl Display for ReturnStatement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "return {};", self.value.to_string())
    }
}

#[cfg(test)]
mod test {
    use crate::token::{EOFToken, IdentToken, SemicolonToken};

    use super::*;

    #[test]
    fn simple_return() {
        let mut lexer = Lexer::from_tokens([
            Token::Return(ReturnToken::default()),
            Token::Ident(IdentToken {
                literal: "a".to_string(),
                ..Default::default()
            }),
            Token::Semicolon(SemicolonToken::default()),
            Token::EOF(EOFToken::default()),
        ]);

        let result = ReturnStatement::parse(&mut lexer);

        assert!(matches!(result, Ok(ReturnStatement { .. })));
        if let Ok(return_statement) = result {
            assert!(matches!(return_statement.value, Expression::Identifier(_)));

            if let Expression::Identifier(ident) = return_statement.value {
                assert_eq!(ident.value, "a");
            }
        }

        assert!(matches!(lexer.next(), Token::EOF(_)));
    }

    #[test]
    fn reject_no_return() {
        let mut lexer = Lexer::from_tokens([
            Token::Ident(IdentToken {
                literal: "a".to_string(),
                ..Default::default()
            }),
            Token::Semicolon(SemicolonToken::default()),
        ]);

        let result = ReturnStatement::parse(&mut lexer);

        assert!(result.is_err());
    }

    #[test]
    fn reject_no_value() {
        let mut lexer = Lexer::from_tokens([
            Token::Return(ReturnToken::default()),
            Token::Semicolon(SemicolonToken::default()),
        ]);

        let result = ReturnStatement::parse(&mut lexer);

        assert!(result.is_err());
    }

    #[test]
    fn reject_no_semicolon() {
        let mut lexer = Lexer::from_tokens([
            Token::Return(ReturnToken::default()),
            Token::Ident(IdentToken {
                literal: "a".to_string(),
                ..Default::default()
            }),
        ]);

        let result = ReturnStatement::parse(&mut lexer);

        assert!(result.is_err());
    }
}
