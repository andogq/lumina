use std::fmt::{Display, Formatter};

use crate::core::{
    ast::ParseNode,
    lexer::{IdentToken, Lexer, Token},
};

#[derive(Clone, Debug, PartialEq)]
pub struct Identifier {
    pub ident_token: IdentToken,
    pub value: String,
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
    use crate::core::lexer::SemicolonToken;

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
