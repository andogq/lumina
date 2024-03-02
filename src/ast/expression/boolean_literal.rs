use std::fmt::{Display, Formatter};

use crate::{
    ast::ParseNode,
    lexer::Lexer,
    token::{FalseToken, Token, TrueToken},
};

#[derive(Clone, Debug, PartialEq)]
pub enum BooleanToken {
    True(TrueToken),
    False(FalseToken),
}

#[derive(Clone, Debug, PartialEq)]
pub struct BooleanLiteral {
    pub token: BooleanToken,
    pub value: bool,
}

impl BooleanLiteral {
    pub fn new(value: bool) -> Self {
        Self {
            token: match value {
                true => BooleanToken::True(TrueToken::default()),
                false => BooleanToken::False(FalseToken::default()),
            },
            value,
        }
    }
}

impl<S> ParseNode<S> for BooleanLiteral
where
    S: Iterator<Item = char>,
{
    fn parse(lexer: &mut Lexer<S>) -> Result<Self, String> {
        Ok(match lexer.next() {
            Token::True(true_token) => BooleanLiteral {
                token: BooleanToken::True(true_token),
                value: true,
            },
            Token::False(false_token) => BooleanLiteral {
                token: BooleanToken::False(false_token),
                value: false,
            },
            _ => return Err("expected boolean".to_string()),
        })
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
    use crate::token::SemicolonToken;

    use super::*;

    #[test]
    fn parse_true() {
        let mut lexer = Lexer::from_tokens([
            Token::True(TrueToken::default()),
            Token::Semicolon(SemicolonToken::default()),
        ]);

        assert!(matches!(
            BooleanLiteral::parse(&mut lexer),
            Ok(BooleanLiteral {
                token: BooleanToken::True(_),
                value: true
            })
        ));
        assert!(matches!(lexer.next(), Token::Semicolon(_)));
    }

    #[test]
    fn parse_false() {
        let mut lexer = Lexer::from_tokens([
            Token::False(FalseToken::default()),
            Token::Semicolon(SemicolonToken::default()),
        ]);

        assert!(matches!(
            BooleanLiteral::parse(&mut lexer),
            Ok(BooleanLiteral {
                token: BooleanToken::False(_),
                value: false,
            })
        ));
        assert!(matches!(lexer.next(), Token::Semicolon(_)));
    }

    #[test]
    fn reject_non_bool() {
        assert!(matches!(
            BooleanLiteral::parse(&mut Lexer::from_tokens([Token::Semicolon(
                SemicolonToken::default()
            )])),
            Err(_)
        ));
    }
}
