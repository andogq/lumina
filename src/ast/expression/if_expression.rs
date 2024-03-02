use std::fmt::Display;

use crate::{
    ast::{BlockStatement, ParseNode},
    lexer::Lexer,
    token::{ElseToken, IfToken, Token},
};

use super::Expression;

#[derive(Clone, Debug, PartialEq)]
pub struct ElseBranch {
    pub else_token: ElseToken,
    pub statement: BlockStatement,
}

#[derive(Clone, Debug, PartialEq)]
pub struct IfExpression {
    pub if_token: IfToken,
    pub condition: Expression,
    pub consequence: BlockStatement,
    pub else_branch: Option<ElseBranch>,
}

impl<S> ParseNode<S> for IfExpression
where
    S: Iterator<Item = char>,
{
    fn parse(lexer: &mut Lexer<S>) -> Result<Self, String> {
        let Token::If(if_token) = lexer.next() else {
            return Err("expected `if` token".to_string());
        };

        let Token::LeftParen(_l_paren) = lexer.next() else {
            return Err("expected opening parenthesis".to_string());
        };

        let condition = Expression::parse(lexer)?;

        let Token::RightParen(_r_paren) = lexer.next() else {
            return Err("expected closing parenthesis".to_string());
        };

        let consequence = BlockStatement::parse(lexer)?;

        let else_branch = lexer
            .next_if(|token| matches!(token, Token::Else(_)))
            .and_then(|token| {
                if let Token::Else(token) = token {
                    Some(token)
                } else {
                    None
                }
            })
            .map(|else_token| {
                Ok::<_, String>(ElseBranch {
                    else_token,
                    statement: BlockStatement::parse(lexer)?,
                })
            })
            .transpose()?;

        Ok(IfExpression {
            if_token,
            condition,
            consequence,
            else_branch,
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
            if let Some(alt) = &self.else_branch {
                format!(" else {}", alt.statement.to_string())
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
        let mut lexer = Lexer::from_tokens([
            Token::If(IfToken::default()),
            Token::LeftParen(LeftParenToken::default()),
            Token::Ident(IdentToken {
                literal: "x".to_string(),
                ..Default::default()
            }),
            Token::LeftAngle(LeftAngleToken::default()),
            Token::Ident(IdentToken {
                literal: "y".to_string(),
                ..Default::default()
            }),
            Token::RightParen(RightParenToken::default()),
            Token::LeftBrace(LeftBraceToken::default()),
            Token::Ident(IdentToken {
                literal: "x".to_string(),
                ..Default::default()
            }),
            Token::RightBrace(RightBraceToken::default()),
        ]);

        let result = IfExpression::parse(&mut lexer);

        assert!(matches!(
            result,
            Ok(IfExpression {
                else_branch: None,
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
        let mut lexer = Lexer::from_tokens([
            Token::If(IfToken::default()),
            Token::LeftParen(LeftParenToken::default()),
            Token::Ident(IdentToken {
                literal: "x".to_string(),
                ..Default::default()
            }),
            Token::LeftAngle(LeftAngleToken::default()),
            Token::Ident(IdentToken {
                literal: "y".to_string(),
                ..Default::default()
            }),
            Token::RightParen(RightParenToken::default()),
            Token::LeftBrace(LeftBraceToken::default()),
            Token::Ident(IdentToken {
                literal: "x".to_string(),
                ..Default::default()
            }),
            Token::RightBrace(RightBraceToken::default()),
            Token::Else(ElseToken::default()),
            Token::LeftBrace(LeftBraceToken::default()),
            Token::Ident(IdentToken {
                literal: "y".to_string(),
                ..Default::default()
            }),
            Token::RightBrace(RightBraceToken::default()),
        ]);

        let result = IfExpression::parse(&mut lexer);

        assert!(matches!(
            result,
            Ok(IfExpression {
                else_branch: Some(_),
                ..
            })
        ));

        if let Ok(expr) = result {
            assert_eq!(expr.condition.to_string(), "(x < y)");
            assert_eq!(expr.consequence.to_string(), "{ x }");

            if let Some(alt) = expr.else_branch {
                assert_eq!(alt.statement.to_string(), "{ y }");
            }
        }
    }
}
