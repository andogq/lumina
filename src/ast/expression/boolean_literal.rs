use std::{
    fmt::{Display, Formatter},
    iter::Peekable,
};

use crate::{
    ast::{AstNode, ParseNode},
    interpreter::{
        object::{BooleanObject, Object},
        return_value::Return,
    },
    token::{FalseToken, Token, TrueToken},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BooleanToken {
    True(TrueToken),
    False(FalseToken),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BooleanLiteral {
    pub token: BooleanToken,
    pub value: bool,
}

impl BooleanLiteral {
    pub fn new(value: bool) -> Self {
        Self {
            token: match value {
                true => BooleanToken::True(TrueToken),
                false => BooleanToken::False(FalseToken),
            },
            value,
        }
    }
}

impl AstNode for BooleanLiteral {
    fn evaluate(&self) -> Return<Object> {
        Return::Implicit(Object::Boolean(BooleanObject { value: self.value }))
    }
}

impl ParseNode for BooleanLiteral {
    fn parse(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<Self, String> {
        tokens
            .next()
            .and_then(|token| match token {
                Token::True(true_token) => Some(BooleanLiteral {
                    token: BooleanToken::True(true_token),
                    value: true,
                }),
                Token::False(false_token) => Some(BooleanLiteral {
                    token: BooleanToken::False(false_token),
                    value: false,
                }),
                _ => None,
            })
            .ok_or_else(|| "expected boolean".to_string())
    }
}

impl Display for BooleanLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.value {
            true => write!(f, "true"),
            false => write!(f, "false"),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::token::{EOFToken, SemicolonToken};

    use super::*;

    #[test]
    fn parse_true() {
        let mut tokens = [Token::True(TrueToken), Token::EOF(EOFToken)]
            .into_iter()
            .peekable();

        assert!(matches!(
            BooleanLiteral::parse(&mut tokens),
            Ok(BooleanLiteral {
                token: BooleanToken::True(_),
                value: true
            })
        ));
        assert_eq!(tokens.count(), 1);
    }

    #[test]
    fn parse_false() {
        let mut tokens = [Token::False(FalseToken), Token::EOF(EOFToken)]
            .into_iter()
            .peekable();

        assert!(matches!(
            BooleanLiteral::parse(&mut tokens),
            Ok(BooleanLiteral {
                token: BooleanToken::False(_),
                value: false,
            })
        ));
        assert_eq!(tokens.count(), 1);
    }

    #[test]
    fn reject_non_bool() {
        assert!(matches!(
            BooleanLiteral::parse(&mut [Token::Semicolon(SemicolonToken)].into_iter().peekable()),
            Err(_)
        ));
    }

    #[test]
    fn evaluate() {
        assert!(matches!(
            BooleanLiteral::new(true).evaluate(),
            Return::Implicit(Object::Boolean(BooleanObject { value: true }))
        ))
    }
}
