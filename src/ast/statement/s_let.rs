use std::{fmt::Display, iter::Peekable};

use crate::{
    ast::{AstNode, Expression, Identifier, ParseNode},
    interpreter::{object::Object, return_value::Return},
    token::{LetToken, Token},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LetStatement {
    pub let_token: LetToken,
    pub name: Identifier,
    pub value: Expression,
}

impl AstNode for LetStatement {
    fn evaluate(&self) -> Return<Object> {
        todo!()
    }
}

impl ParseNode for LetStatement {
    fn parse(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<Self, String> {
        let let_token = tokens
            .next()
            .and_then(|token| {
                if let Token::Let(let_token) = token {
                    Some(let_token)
                } else {
                    None
                }
            })
            .ok_or_else(|| "expected `let` token".to_string())?;

        let name = tokens
            .next()
            .and_then(|token| {
                if let Token::Ident(name_ident_token) = token {
                    Some(Identifier {
                        value: name_ident_token.literal.clone(),
                        ident_token: name_ident_token,
                    })
                } else {
                    None
                }
            })
            .ok_or_else(|| "expected `ident` token".to_string())?;

        let _assign_token = tokens
            .next()
            .and_then(|token| {
                if let Token::Assign(assign_token) = token {
                    Some(assign_token)
                } else {
                    None
                }
            })
            .ok_or_else(|| "expected `assign` token".to_string())?;

        let value = Expression::parse(tokens)?;

        let _semicolon_token = tokens
            .next()
            .and_then(|token| {
                if let Token::Semicolon(semicolon_token) = token {
                    Some(semicolon_token)
                } else {
                    None
                }
            })
            .ok_or_else(|| "expected `semicolon` token".to_string())?;

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
        let mut tokens = [
            Token::Let(LetToken),
            Token::Ident(IdentToken {
                literal: "a".to_string(),
            }),
            Token::Assign(AssignToken),
            Token::Int(IntToken {
                literal: "5".to_string(),
            }),
            Token::Semicolon(SemicolonToken),
            Token::EOF(EOFToken),
        ]
        .into_iter()
        .peekable();

        let result = LetStatement::parse(&mut tokens);
        assert!(matches!(result, Ok(LetStatement { .. })));

        if let Ok(let_statement) = result {
            assert_eq!(let_statement.name.value, "a");
            assert!(matches!(let_statement.value, Expression::Integer(_)));
            if let Expression::Integer(int_lit) = let_statement.value {
                assert_eq!(int_lit.value, 5);
            }
        }

        assert_eq!(tokens.count(), 1);
    }

    #[test]
    fn reject_no_ident() {
        let mut tokens = [
            Token::Let(LetToken),
            Token::Assign(AssignToken),
            Token::Int(IntToken {
                literal: "5".to_string(),
            }),
            Token::Semicolon(SemicolonToken),
        ]
        .into_iter()
        .peekable();

        let result = LetStatement::parse(&mut tokens);
        assert!(result.is_err());
    }

    #[test]
    fn reject_no_value() {
        let mut tokens = [
            Token::Let(LetToken),
            Token::Ident(IdentToken {
                literal: "a".to_string(),
            }),
            Token::Assign(AssignToken),
            Token::Semicolon(SemicolonToken),
        ]
        .into_iter()
        .peekable();

        let result = LetStatement::parse(&mut tokens);
        assert!(result.is_err());
    }

    #[test]
    fn reject_no_assign() {
        let mut tokens = [
            Token::Let(LetToken),
            Token::Ident(IdentToken {
                literal: "a".to_string(),
            }),
            Token::Int(IntToken {
                literal: "5".to_string(),
            }),
            Token::Semicolon(SemicolonToken),
        ]
        .into_iter()
        .peekable();

        let result = LetStatement::parse(&mut tokens);
        assert!(result.is_err());
    }

    #[test]
    fn reject_no_semi_colon() {
        let mut tokens = [
            Token::Let(LetToken),
            Token::Ident(IdentToken {
                literal: "a".to_string(),
            }),
            Token::Assign(AssignToken),
            Token::Int(IntToken {
                literal: "5".to_string(),
            }),
        ]
        .into_iter()
        .peekable();

        let result = LetStatement::parse(&mut tokens);
        assert!(result.is_err());
    }
}
