use std::fmt::Display;

use crate::{
    ast::{Expression, Identifier, ParseNode},
    lexer::Lexer,
    token::{LetToken, Token},
};

#[derive(Clone, Debug, PartialEq)]
pub struct LetStatement {
    pub let_token: LetToken,
    pub name: Identifier,
    pub value: Expression,
}

impl<S> ParseNode<S> for LetStatement
where
    S: Iterator<Item = char>,
{
    fn parse(tokens: &mut Lexer<S>) -> Result<Self, String> {
        let Token::Let(let_token) = tokens.next() else {
            return Err("expected `let` token".to_string());
        };

        let name = {
            let Token::Ident(name_ident_token) = tokens.next() else {
                return Err("expected `ident` token".to_string());
            };
            Identifier {
                value: name_ident_token.literal.clone(),
                ident_token: name_ident_token,
            }
        };

        let Token::Assign(_assign_token) = tokens.next() else {
            return Err("expected `assign` token".to_string());
        };

        let value = Expression::parse(tokens)?;

        let Token::Semicolon(_semicolon_token) = tokens.next() else {
            return Err("expected `semicolon` token".to_string());
        };

        Ok(LetStatement {
            let_token,
            name,
            value,
        })
    }
}

impl Display for LetStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "let {} = {};",
            self.name.to_string(),
            self.value.to_string()
        )
    }
}

#[cfg(test)]
mod test {
    use crate::token::{AssignToken, EOFToken, IdentToken, IntToken, SemicolonToken};

    use super::*;

    #[test]
    fn simple_literal() {
        let mut lexer = Lexer::from_tokens([
            Token::Let(LetToken::default()),
            Token::Ident(IdentToken {
                literal: "a".to_string(),
                ..Default::default()
            }),
            Token::Assign(AssignToken::default()),
            Token::Int(IntToken {
                literal: "5".to_string(),
                ..Default::default()
            }),
            Token::Semicolon(SemicolonToken::default()),
            Token::EOF(EOFToken::default()),
        ]);

        let result = LetStatement::parse(&mut lexer);
        assert!(matches!(result, Ok(LetStatement { .. })));

        if let Ok(let_statement) = result {
            assert_eq!(let_statement.name.value, "a");
            assert!(matches!(let_statement.value, Expression::Integer(_)));
            if let Expression::Integer(int_lit) = let_statement.value {
                assert_eq!(int_lit.value, 5);
            }
        }

        assert!(matches!(lexer.next(), Token::EOF(_)));
    }

    #[test]
    fn reject_no_ident() {
        let mut lexer = Lexer::from_tokens([
            Token::Let(LetToken::default()),
            Token::Assign(AssignToken::default()),
            Token::Int(IntToken {
                literal: "5".to_string(),
                ..Default::default()
            }),
            Token::Semicolon(SemicolonToken::default()),
        ]);

        let result = LetStatement::parse(&mut lexer);
        assert!(result.is_err());
    }

    #[test]
    fn reject_no_value() {
        let mut lexer = Lexer::from_tokens([
            Token::Let(LetToken::default()),
            Token::Ident(IdentToken {
                literal: "a".to_string(),
                ..Default::default()
            }),
            Token::Assign(AssignToken::default()),
            Token::Semicolon(SemicolonToken::default()),
        ]);

        let result = LetStatement::parse(&mut lexer);
        assert!(result.is_err());
    }

    #[test]
    fn reject_no_assign() {
        let mut lexer = Lexer::from_tokens([
            Token::Let(LetToken::default()),
            Token::Ident(IdentToken {
                literal: "a".to_string(),
                ..Default::default()
            }),
            Token::Int(IntToken {
                literal: "5".to_string(),
                ..Default::default()
            }),
            Token::Semicolon(SemicolonToken::default()),
        ]);

        let result = LetStatement::parse(&mut lexer);
        assert!(result.is_err());
    }

    #[test]
    fn reject_no_semi_colon() {
        let mut tokens = Lexer::from_tokens([
            Token::Let(LetToken::default()),
            Token::Ident(IdentToken {
                literal: "a".to_string(),
                ..Default::default()
            }),
            Token::Assign(AssignToken::default()),
            Token::Int(IntToken {
                literal: "5".to_string(),
                ..Default::default()
            }),
        ]);

        let result = LetStatement::parse(&mut tokens);
        assert!(result.is_err());
    }
}
