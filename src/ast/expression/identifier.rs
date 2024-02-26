use std::fmt::{Display, Formatter};

use crate::{
    ast::{AstNode, ParseNode},
    interpreter::{environment::Environment, error::Error, return_value::Return},
    lexer::Lexer,
    object::Object,
    token::{IdentToken, Token},
};

#[derive(Clone, Debug, PartialEq)]
pub struct Identifier {
    pub ident_token: IdentToken,
    pub value: String,
}

impl AstNode for Identifier {
    fn evaluate(&self, env: Environment) -> Return<Object> {
        env.get(&self.value)
            .map(|value| Return::Implicit(value))
            .unwrap_or_else(|| Error::throw(format!("identifier not found: \"{}\"", self.value)))
    }

    fn compile(&self, register_constant: impl FnMut(Object) -> u32) -> Result<Vec<u8>, String> {
        todo!()
    }
}

impl<S> ParseNode<S> for Identifier
where
    S: Iterator<Item = char>,
{
    fn parse(lexer: &mut Lexer<S>) -> Result<Self, String> {
        let Token::Ident(ident) = lexer.next() else {
            return Err("expected identifier".to_string());
        };

        Ok(Identifier {
            value: ident.literal.clone(),
            ident_token: ident,
        })
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}

#[cfg(test)]
mod test {
    use crate::token::SemicolonToken;

    use super::*;

    #[test]
    fn parse_ident() {
        let mut lexer = Lexer::from_tokens([
            Token::Ident(IdentToken {
                literal: "my_ident".to_string(),
                ..Default::default()
            }),
            Token::Semicolon(SemicolonToken::default()),
        ]);

        let result = Identifier::parse(&mut lexer);

        assert!(matches!(result, Ok(Identifier { .. })));

        if let Ok(Identifier { value, .. }) = result {
            assert_eq!(value, "my_ident");
        }

        assert!(matches!(lexer.next(), Token::Semicolon(_)));
    }

    #[test]
    fn reject_non_ident() {
        assert!(matches!(
            Identifier::parse(&mut Lexer::from_tokens([Token::Semicolon(
                SemicolonToken::default()
            )])),
            Err(_)
        ));
    }
}
