use std::fmt::{Display, Formatter};

use crate::core::{
    ast::{BlockStatement, ParseNode},
    lexer::{FunctionToken, Lexer, Token},
};

use super::Identifier;

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionLiteral {
    pub fn_token: FunctionToken,
    pub parameters: Vec<Identifier>,
    pub body: BlockStatement,
}

impl<S> ParseNode<S> for FunctionLiteral
where
    S: Iterator<Item = char>,
{
    fn parse(lexer: &mut Lexer<S>) -> Result<Self, String> {
        let Token::Function(fn_token) = lexer.next() else {
            return Err("expected function token".to_string());
        };

        let Token::LeftParen(_l_paren) = lexer.next() else {
            return Err("expected opening parenthesis".to_string())?;
        };

        let mut parameters = Vec::new();

        let mut expect_end = false;

        // Loop through all parameters in list
        loop {
            // Check if parameter list is closed
            if lexer
                .next_if(|token| matches!(token, Token::RightParen(_)))
                .is_some()
            {
                break;
            } else if expect_end {
                return Err("expected end of parameter list".to_string());
            }

            let param = Identifier::parse(lexer)?;
            parameters.push(param);

            if lexer
                .next_if(|token| matches!(token, Token::Comma(_)))
                .is_some()
            {
                // Comma encountered, continue parsing parameters
                continue;
            } else {
                // Comma not encountered, so the next thing should be the end of the aprameter list
                expect_end = true;
            }
        }

        let body = BlockStatement::parse(lexer)?;

        Ok(Self {
            fn_token,
            parameters,
            body,
        })
    }
}

impl Display for FunctionLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "fn({}) {}",
            self.parameters
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<_>>()
                .join(", "),
            self.body.to_string()
        )
    }
}

#[cfg(test)]
mod test {
    use crate::core::lexer::{
        CommaToken, IdentToken, IntToken, LeftBraceToken, LeftParenToken, RightBraceToken,
        RightParenToken, SemicolonToken,
    };

    use super::*;

    #[test]
    fn no_parameters() {
        let mut lexer = Lexer::from_tokens([
            Token::Function(FunctionToken::default()),
            Token::LeftParen(LeftParenToken::default()),
            Token::RightParen(RightParenToken::default()),
            Token::LeftBrace(LeftBraceToken::default()),
            Token::Int(IntToken {
                literal: "0".to_string(),
                ..Default::default()
            }),
            Token::RightBrace(RightBraceToken::default()),
            Token::Semicolon(SemicolonToken::default()),
        ]);

        let result = FunctionLiteral::parse(&mut lexer);

        assert!(matches!(result, Ok(FunctionLiteral { .. })));

        if let Ok(func) = result {
            assert!(func.parameters.is_empty());
            assert_eq!(func.body.to_string(), "{ 0 }");
        }

        assert!(matches!(lexer.next(), Token::Semicolon(_)));
    }

    #[test]
    fn one_parameter() {
        let mut lexer = Lexer::from_tokens([
            Token::Function(FunctionToken::default()),
            Token::LeftParen(LeftParenToken::default()),
            Token::Ident(IdentToken {
                literal: "x".to_string(),
                ..Default::default()
            }),
            Token::RightParen(RightParenToken::default()),
            Token::LeftBrace(LeftBraceToken::default()),
            Token::Ident(IdentToken {
                literal: "x".to_string(),
                ..Default::default()
            }),
            Token::RightBrace(RightBraceToken::default()),
            Token::Semicolon(SemicolonToken::default()),
        ]);

        let result = FunctionLiteral::parse(&mut lexer);

        assert!(matches!(result, Ok(FunctionLiteral { .. })));

        if let Ok(func) = result {
            assert_eq!(func.parameters.len(), 1);
            assert_eq!(func.parameters[0].value, "x");
            assert_eq!(func.body.to_string(), "{ x }");
        }

        assert!(matches!(lexer.next(), Token::Semicolon(_)));
    }

    #[test]
    fn one_parameter_trailing_comma() {
        let mut lexer = Lexer::from_tokens([
            Token::Function(FunctionToken::default()),
            Token::LeftParen(LeftParenToken::default()),
            Token::Ident(IdentToken {
                literal: "x".to_string(),
                ..Default::default()
            }),
            Token::Comma(CommaToken::default()),
            Token::RightParen(RightParenToken::default()),
            Token::LeftBrace(LeftBraceToken::default()),
            Token::Ident(IdentToken {
                literal: "x".to_string(),
                ..Default::default()
            }),
            Token::RightBrace(RightBraceToken::default()),
            Token::Semicolon(SemicolonToken::default()),
        ]);

        let result = FunctionLiteral::parse(&mut lexer);

        assert!(matches!(result, Ok(FunctionLiteral { .. })));

        if let Ok(func) = result {
            assert_eq!(func.parameters.len(), 1);
            assert_eq!(func.parameters[0].value, "x");
            assert_eq!(func.body.to_string(), "{ x }");
        }

        assert!(matches!(lexer.next(), Token::Semicolon(_)));
    }

    #[test]
    fn multiple_parameters() {
        let mut lexer = Lexer::from_tokens([
            Token::Function(FunctionToken::default()),
            Token::LeftParen(LeftParenToken::default()),
            Token::Ident(IdentToken {
                literal: "x".to_string(),
                ..Default::default()
            }),
            Token::Comma(CommaToken::default()),
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
            Token::Semicolon(SemicolonToken::default()),
        ]);

        let result = FunctionLiteral::parse(&mut lexer);

        assert!(matches!(result, Ok(FunctionLiteral { .. })));

        if let Ok(func) = result {
            assert_eq!(func.parameters.len(), 2);
            assert_eq!(func.parameters[0].value, "x");
            assert_eq!(func.parameters[1].value, "y");
            assert_eq!(func.body.to_string(), "{ x }");
        }

        assert!(matches!(lexer.next(), Token::Semicolon(_)));
    }

    #[test]
    fn multiple_parameters_with_trailing_comma() {
        let mut lexer = Lexer::from_tokens([
            Token::Function(FunctionToken::default()),
            Token::LeftParen(LeftParenToken::default()),
            Token::Ident(IdentToken {
                literal: "x".to_string(),
                ..Default::default()
            }),
            Token::Comma(CommaToken::default()),
            Token::Ident(IdentToken {
                literal: "y".to_string(),
                ..Default::default()
            }),
            Token::Comma(CommaToken::default()),
            Token::RightParen(RightParenToken::default()),
            Token::LeftBrace(LeftBraceToken::default()),
            Token::Ident(IdentToken {
                literal: "x".to_string(),
                ..Default::default()
            }),
            Token::RightBrace(RightBraceToken::default()),
            Token::Semicolon(SemicolonToken::default()),
        ]);

        let result = FunctionLiteral::parse(&mut lexer);

        assert!(matches!(result, Ok(FunctionLiteral { .. })));

        if let Ok(func) = result {
            assert_eq!(func.parameters.len(), 2);
            assert_eq!(func.parameters[0].value, "x");
            assert_eq!(func.parameters[1].value, "y");
            assert_eq!(func.body.to_string(), "{ x }");
        }

        assert!(matches!(lexer.next(), Token::Semicolon(_)));
    }
}
