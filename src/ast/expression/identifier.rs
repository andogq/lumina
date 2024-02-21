use std::iter::Peekable;

use crate::{
    parser::Node,
    token::{IdentToken, Token},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Identifier {
    pub ident_token: IdentToken,
    pub value: String,
}

impl Node for Identifier {
    fn parse(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<Self, String> {
        tokens
            .next()
            .and_then(|token| {
                if let Token::Ident(ident_token) = token {
                    Some(Identifier {
                        value: ident_token.literal.clone(),
                        ident_token,
                    })
                } else {
                    None
                }
            })
            .ok_or_else(|| "expected identifier".to_string())
    }
}

impl ToString for Identifier {
    fn to_string(&self) -> String {
        self.value.clone()
    }
}

#[cfg(test)]
mod test {
    use crate::token::{EOFToken, SemicolonToken};

    use super::*;

    #[test]
    fn parse_ident() {
        let mut tokens = [
            Token::Ident(IdentToken {
                literal: "my_ident".to_string(),
            }),
            Token::EOF(EOFToken),
        ]
        .into_iter()
        .peekable();

        let result = Identifier::parse(&mut tokens);

        assert!(matches!(result, Ok(Identifier { .. })));

        if let Ok(Identifier { value, .. }) = result {
            assert_eq!(value, "my_ident");
        }

        assert_eq!(tokens.count(), 1);
    }

    #[test]
    fn reject_non_ident() {
        assert!(matches!(
            Identifier::parse(&mut [Token::Semicolon(SemicolonToken)].into_iter().peekable()),
            Err(_)
        ));
    }
}
