use std::{fmt::Display, iter::Peekable};

use crate::{
    ast::{AstNode, BlockStatement, ParseNode},
    object::Object,
    token::{IfToken, Token},
};

use super::Expression;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IfExpression {
    if_token: IfToken,
    condition: Expression,
    consequence: BlockStatement,
    alternative: Option<BlockStatement>,
}

impl AstNode for IfExpression {
    fn evaluate(&self) -> Object {
        todo!()
    }
}

impl ParseNode for IfExpression {
    fn parse(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<Self, String> {
        let if_token = tokens
            .next()
            .and_then(|token| {
                if let Token::If(token) = token {
                    Some(token)
                } else {
                    None
                }
            })
            .ok_or_else(|| "expected `if` token".to_string())?;

        let _l_paren = tokens
            .next()
            .and_then(|token| {
                if let Token::LeftParen(token) = token {
                    Some(token)
                } else {
                    None
                }
            })
            .ok_or_else(|| "expected opening parenthesis".to_string())?;

        let condition = Expression::parse(tokens)?;

        let _r_paren = tokens
            .next()
            .and_then(|token| {
                if let Token::RightParen(token) = token {
                    Some(token)
                } else {
                    None
                }
            })
            .ok_or_else(|| "expected closing parenthesis".to_string())?;

        let consequence = BlockStatement::parse(tokens)?;

        let alternative = if let Some(Token::Else(_)) = tokens.peek() {
            tokens.next();

            Some(BlockStatement::parse(tokens)?)
        } else {
            None
        };

        Ok(IfExpression {
            if_token,
            condition,
            consequence,
            alternative,
        })
    }
}

impl Display for IfExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "if {} {}{}",
            self.condition.to_string(),
            self.consequence.to_string(),
            if let Some(alt) = &self.alternative {
                format!(" else {}", alt.to_string())
            } else {
                String::new()
            }
        )
    }
}

#[cfg(test)]
mod test {
    use crate::token::{
        ElseToken, IdentToken, IfToken, LeftAngleToken, LeftBraceToken, LeftParenToken,
        RightBraceToken, RightParenToken,
    };

    use super::*;

    #[test]
    fn simple() {
        let mut tokens = [
            Token::If(IfToken),
            Token::LeftParen(LeftParenToken),
            Token::Ident(IdentToken {
                literal: "x".to_string(),
            }),
            Token::LeftAngle(LeftAngleToken),
            Token::Ident(IdentToken {
                literal: "y".to_string(),
            }),
            Token::RightParen(RightParenToken),
            Token::LeftBrace(LeftBraceToken),
            Token::Ident(IdentToken {
                literal: "x".to_string(),
            }),
            Token::RightBrace(RightBraceToken),
        ]
        .into_iter()
        .peekable();

        let result = IfExpression::parse(&mut tokens);

        assert!(matches!(
            result,
            Ok(IfExpression {
                alternative: None,
                ..
            })
        ));

        if let Ok(expr) = result {
            assert_eq!(expr.condition.to_string(), "(x < y)");
            assert_eq!(expr.consequence.to_string(), "{ x }");
        }
    }

    #[test]
    fn if_else() {
        let mut tokens = [
            Token::If(IfToken),
            Token::LeftParen(LeftParenToken),
            Token::Ident(IdentToken {
                literal: "x".to_string(),
            }),
            Token::LeftAngle(LeftAngleToken),
            Token::Ident(IdentToken {
                literal: "y".to_string(),
            }),
            Token::RightParen(RightParenToken),
            Token::LeftBrace(LeftBraceToken),
            Token::Ident(IdentToken {
                literal: "x".to_string(),
            }),
            Token::RightBrace(RightBraceToken),
            Token::Else(ElseToken),
            Token::LeftBrace(LeftBraceToken),
            Token::Ident(IdentToken {
                literal: "y".to_string(),
            }),
            Token::RightBrace(RightBraceToken),
        ]
        .into_iter()
        .peekable();

        let result = IfExpression::parse(&mut tokens);

        assert!(matches!(
            result,
            Ok(IfExpression {
                alternative: Some(_),
                ..
            })
        ));

        if let Ok(expr) = result {
            assert_eq!(expr.condition.to_string(), "(x < y)");
            assert_eq!(expr.consequence.to_string(), "{ x }");

            if let Some(alt) = expr.alternative {
                assert_eq!(alt.to_string(), "{ y }");
            }
        }
    }
}
