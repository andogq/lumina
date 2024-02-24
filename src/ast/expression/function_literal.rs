use std::{
    fmt::{Display, Formatter},
    iter::Peekable,
};

use crate::{
    ast::{AstNode, BlockStatement, ParseNode},
    interpreter::{
        environment::Environment,
        object::{FunctionObject, Object},
        return_value::Return,
    },
    token::{FunctionToken, Token},
};

use super::Identifier;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FunctionLiteral {
    pub fn_token: FunctionToken,
    pub parameters: Vec<Identifier>,
    pub body: BlockStatement,
}

impl AstNode for FunctionLiteral {
    fn evaluate(&self, env: Environment) -> Return<Object> {
        Return::Implicit(Object::Function(FunctionObject {
            parameters: self.parameters.clone(),
            body: self.body.clone(),
            env: env.clone(),
        }))
    }
}

impl ParseNode for FunctionLiteral {
    fn parse(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<Self, String> {
        let fn_token = tokens
            .next()
            .and_then(|token| {
                if let Token::Function(token) = token {
                    Some(token)
                } else {
                    None
                }
            })
            .ok_or_else(|| "expected function token".to_string())?;

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

        let mut parameters = Vec::new();

        let mut expect_end = false;

        // Loop through all parameters in list
        loop {
            // Check if parameter list is closed
            if tokens
                .next_if(|token| matches!(token, Token::RightParen(_)))
                .is_some()
            {
                break;
            } else if expect_end {
                return Err("expected end of parameter list".to_string());
            }

            let param = Identifier::parse(tokens)?;
            parameters.push(param);

            if tokens
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

        let body = BlockStatement::parse(tokens)?;

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
    use crate::token::{
        CommaToken, EOFToken, IdentToken, IntToken, LeftBraceToken, LeftParenToken,
        RightBraceToken, RightParenToken,
    };

    use super::*;

    #[test]
    fn no_parameters() {
        let mut tokens = [
            Token::Function(FunctionToken),
            Token::LeftParen(LeftParenToken),
            Token::RightParen(RightParenToken),
            Token::LeftBrace(LeftBraceToken),
            Token::Int(IntToken {
                literal: "0".to_string(),
            }),
            Token::RightBrace(RightBraceToken),
            Token::EOF(EOFToken),
        ]
        .into_iter()
        .peekable();

        let result = dbg!(FunctionLiteral::parse(&mut tokens));

        assert!(matches!(result, Ok(FunctionLiteral { .. })));

        if let Ok(func) = result {
            assert!(func.parameters.is_empty());
            assert_eq!(func.body.to_string(), "{ 0 }");
        }

        assert_eq!(tokens.count(), 1);
    }

    #[test]
    fn one_parameter() {
        let mut tokens = [
            Token::Function(FunctionToken),
            Token::LeftParen(LeftParenToken),
            Token::Ident(IdentToken {
                literal: "x".to_string(),
            }),
            Token::RightParen(RightParenToken),
            Token::LeftBrace(LeftBraceToken),
            Token::Ident(IdentToken {
                literal: "x".to_string(),
            }),
            Token::RightBrace(RightBraceToken),
            Token::EOF(EOFToken),
        ]
        .into_iter()
        .peekable();

        let result = FunctionLiteral::parse(&mut tokens);

        assert!(matches!(result, Ok(FunctionLiteral { .. })));

        if let Ok(func) = result {
            assert_eq!(func.parameters.len(), 1);
            assert_eq!(func.parameters[0].value, "x");
            assert_eq!(func.body.to_string(), "{ x }");
        }

        assert_eq!(tokens.count(), 1);
    }

    #[test]
    fn one_parameter_trailing_comma() {
        let mut tokens = [
            Token::Function(FunctionToken),
            Token::LeftParen(LeftParenToken),
            Token::Ident(IdentToken {
                literal: "x".to_string(),
            }),
            Token::Comma(CommaToken),
            Token::RightParen(RightParenToken),
            Token::LeftBrace(LeftBraceToken),
            Token::Ident(IdentToken {
                literal: "x".to_string(),
            }),
            Token::RightBrace(RightBraceToken),
            Token::EOF(EOFToken),
        ]
        .into_iter()
        .peekable();

        let result = FunctionLiteral::parse(&mut tokens);

        assert!(matches!(result, Ok(FunctionLiteral { .. })));

        if let Ok(func) = result {
            assert_eq!(func.parameters.len(), 1);
            assert_eq!(func.parameters[0].value, "x");
            assert_eq!(func.body.to_string(), "{ x }");
        }

        assert_eq!(tokens.count(), 1);
    }

    #[test]
    fn multiple_parameters() {
        let mut tokens = [
            Token::Function(FunctionToken),
            Token::LeftParen(LeftParenToken),
            Token::Ident(IdentToken {
                literal: "x".to_string(),
            }),
            Token::Comma(CommaToken),
            Token::Ident(IdentToken {
                literal: "y".to_string(),
            }),
            Token::RightParen(RightParenToken),
            Token::LeftBrace(LeftBraceToken),
            Token::Ident(IdentToken {
                literal: "x".to_string(),
            }),
            Token::RightBrace(RightBraceToken),
            Token::EOF(EOFToken),
        ]
        .into_iter()
        .peekable();

        let result = FunctionLiteral::parse(&mut tokens);

        assert!(matches!(result, Ok(FunctionLiteral { .. })));

        if let Ok(func) = result {
            assert_eq!(func.parameters.len(), 2);
            assert_eq!(func.parameters[0].value, "x");
            assert_eq!(func.parameters[1].value, "y");
            assert_eq!(func.body.to_string(), "{ x }");
        }

        assert_eq!(tokens.count(), 1);
    }

    #[test]
    fn multiple_parameters_with_trailing_comma() {
        let mut tokens = [
            Token::Function(FunctionToken),
            Token::LeftParen(LeftParenToken),
            Token::Ident(IdentToken {
                literal: "x".to_string(),
            }),
            Token::Comma(CommaToken),
            Token::Ident(IdentToken {
                literal: "y".to_string(),
            }),
            Token::Comma(CommaToken),
            Token::RightParen(RightParenToken),
            Token::LeftBrace(LeftBraceToken),
            Token::Ident(IdentToken {
                literal: "x".to_string(),
            }),
            Token::RightBrace(RightBraceToken),
            Token::EOF(EOFToken),
        ]
        .into_iter()
        .peekable();

        let result = FunctionLiteral::parse(&mut tokens);

        assert!(matches!(result, Ok(FunctionLiteral { .. })));

        if let Ok(func) = result {
            assert_eq!(func.parameters.len(), 2);
            assert_eq!(func.parameters[0].value, "x");
            assert_eq!(func.parameters[1].value, "y");
            assert_eq!(func.body.to_string(), "{ x }");
        }

        assert_eq!(tokens.count(), 1);
    }
}
