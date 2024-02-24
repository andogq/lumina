use std::{
    fmt::{Display, Formatter},
    iter::Peekable,
};

use crate::{
    ast::{AstNode, Expression, ParseNode, Return},
    object::Object,
    token::{ReturnToken, Token},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReturnStatement {
    pub return_token: ReturnToken,
    pub value: Expression,
}

impl AstNode for ReturnStatement {
    fn evaluate(&self) -> Return<Object> {
        match self.value.evaluate() {
            Return::Explicit(value) | Return::Implicit(value) => Return::Explicit(value),
            Return::Error(err) => Return::Error(err),
        }
    }
}

impl ParseNode for ReturnStatement {
    fn parse(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<Self, String> {
        let return_token = tokens
            .next()
            .and_then(|token| {
                if let Token::Return(return_token) = token {
                    Some(return_token)
                } else {
                    None
                }
            })
            .ok_or_else(|| "expected `return` token".to_string())?;

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
    use crate::{
        ast::{IntegerLiteral, Statement},
        object::IntegerObject,
        token::{EOFToken, IdentToken, IntToken, SemicolonToken},
    };

    use super::*;

    #[test]
    fn simple_return() {
        let mut tokens = [
            Token::Return(ReturnToken),
            Token::Ident(IdentToken {
                literal: "a".to_string(),
            }),
            Token::Semicolon(SemicolonToken),
            Token::EOF(EOFToken),
        ]
        .into_iter()
        .peekable();

        let result = ReturnStatement::parse(&mut tokens);

        assert!(matches!(result, Ok(ReturnStatement { .. })));
        if let Ok(return_statement) = result {
            assert!(matches!(return_statement.value, Expression::Identifier(_)));

            if let Expression::Identifier(ident) = return_statement.value {
                assert_eq!(ident.value, "a");
            }
        }

        assert_eq!(tokens.count(), 1);
    }

    #[test]
    fn reject_no_return() {
        let mut tokens = [
            Token::Ident(IdentToken {
                literal: "a".to_string(),
            }),
            Token::Semicolon(SemicolonToken),
        ]
        .into_iter()
        .peekable();

        let result = ReturnStatement::parse(&mut tokens);

        assert!(result.is_err());
    }

    #[test]
    fn reject_no_value() {
        let mut tokens = [Token::Return(ReturnToken), Token::Semicolon(SemicolonToken)]
            .into_iter()
            .peekable();

        let result = ReturnStatement::parse(&mut tokens);

        assert!(result.is_err());
    }

    #[test]
    fn reject_no_semicolon() {
        let mut tokens = [
            Token::Return(ReturnToken),
            Token::Ident(IdentToken {
                literal: "a".to_string(),
            }),
        ]
        .into_iter()
        .peekable();

        let result = ReturnStatement::parse(&mut tokens);

        assert!(result.is_err());
    }

    #[test]
    fn return_explicit_value() {
        let result = Statement::Return(ReturnStatement {
            return_token: ReturnToken,
            value: Expression::Integer(IntegerLiteral {
                token: IntToken {
                    literal: "10".to_string(),
                },
                value: 10,
            }),
        })
        .evaluate();

        assert!(matches!(
            result,
            Return::Explicit(Object::Integer(IntegerObject { value: 10 }))
        ));
    }
}
