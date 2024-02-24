use std::{
    fmt::{Display, Formatter},
    iter::Peekable,
};

use crate::{
    ast::{AstNode, ParseNode, Return},
    object::{IntegerObject, Object},
    token::{IntToken, Token},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IntegerLiteral {
    pub token: IntToken,
    pub value: i64,
}

impl AstNode for IntegerLiteral {
    fn evaluate(&self) -> Return<Object> {
        Return::Implicit(Object::Integer(IntegerObject { value: self.value }))
    }
}

impl ParseNode for IntegerLiteral {
    fn parse(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<Self, String> {
        let int_token = tokens
            .next()
            .and_then(|token| {
                if let Token::Int(int_token) = token {
                    Some(int_token)
                } else {
                    None
                }
            })
            .ok_or_else(|| "expected integer".to_string())?;

        Ok(IntegerLiteral {
            value: int_token
                .literal
                .parse::<i64>()
                .map_err(|e| e.to_string())?,
            token: int_token,
        })
    }
}

impl Display for IntegerLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}

#[cfg(test)]
mod test {
    use crate::token::{EOFToken, SemicolonToken};

    use super::*;

    #[test]
    fn parse_single_digit() {
        let mut tokens = [
            Token::Int(IntToken {
                literal: "1".to_string(),
            }),
            Token::EOF(EOFToken),
        ]
        .into_iter()
        .peekable();

        assert!(matches!(
            IntegerLiteral::parse(&mut tokens),
            Ok(IntegerLiteral { value: 1, .. })
        ));

        assert_eq!(tokens.count(), 1);
    }

    #[test]
    fn parse_multiple_digits() {
        let mut tokens = [
            Token::Int(IntToken {
                literal: "12345".to_string(),
            }),
            Token::EOF(EOFToken),
        ]
        .into_iter()
        .peekable();

        assert!(matches!(
            IntegerLiteral::parse(&mut tokens),
            Ok(IntegerLiteral { value: 12345, .. })
        ));

        assert_eq!(tokens.count(), 1);
    }

    #[test]
    fn reject_large_number() {
        let mut tokens = [
            Token::Int(IntToken {
                literal: u64::MAX.to_string(),
            }),
            Token::EOF(EOFToken),
        ]
        .into_iter()
        .peekable();

        assert!(matches!(IntegerLiteral::parse(&mut tokens), Err(_)));

        assert_eq!(tokens.count(), 1);
    }

    #[test]
    fn reject_non_number() {
        assert!(matches!(
            IntegerLiteral::parse(&mut [Token::Semicolon(SemicolonToken)].into_iter().peekable()),
            Err(_)
        ));
    }

    #[test]
    fn evaluate() {
        let lit = IntegerLiteral {
            token: IntToken {
                literal: "5".to_string(),
            },
            value: 5,
        };
        let obj = lit.evaluate();

        assert!(matches!(
            obj,
            Return::Implicit(Object::Integer(IntegerObject { value: 5 }))
        ));
    }
}
