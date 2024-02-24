use std::{
    fmt::{Display, Formatter},
    iter::Peekable,
};

use crate::{
    ast::{AstNode, ParseNode},
    interpreter::{object::Object, return_value::Return},
    token::Token,
};

use super::{Expression, FunctionLiteral, Identifier};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CallableFunction {
    Identifier(Identifier),
    FunctionLiteral(FunctionLiteral),
}

impl Display for CallableFunction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CallableFunction::Identifier(identifier) => identifier.fmt(f),
            CallableFunction::FunctionLiteral(function_literal) => function_literal.fmt(f),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CallExpression {
    pub function: CallableFunction,
    pub arguments: Vec<Expression>,
}

impl CallExpression {
    pub fn parse_with_left(
        left: Expression,
        tokens: &mut Peekable<impl Iterator<Item = Token>>,
    ) -> Result<Self, String> {
        let function = match left {
            Expression::Identifier(ident) => Some(CallableFunction::Identifier(ident)),
            Expression::Function(function) => Some(CallableFunction::FunctionLiteral(function)),
            _ => None,
        }
        .ok_or_else(|| "expected ident or function literal".to_string())?;

        // Begin function argument list
        tokens
            .next_if(|token| matches!(token, Token::LeftParen(_)))
            .ok_or_else(|| "expected left parenthesis to begin argument list".to_string())?;

        // Parse out arguments
        let mut arguments = Vec::new();

        while tokens
            .peek()
            .map(|token| !matches!(token, Token::RightParen(_)))
            .ok_or_else(|| "expected token to follow in argument list".to_string())?
        {
            // Parse out the argument
            arguments.push(Expression::parse(tokens)?);

            if !tokens
                // Attempt to get a comma
                .next_if(|token| matches!(token, Token::Comma(_)))
                .map(|_| true)
                // Otherwise make sure a right parenthesis follows
                .or_else(|| {
                    tokens
                        .peek()
                        .map(|token| matches!(token, Token::RightParen(_)))
                })
                .unwrap_or(false)
            {
                return Err("expected a comma or right parenthesis to follow".to_string());
            }
        }

        tokens
            .next_if(|token| matches!(token, Token::RightParen(_)))
            .ok_or_else(|| "expected right parenthesis to close argument list".to_string())?;

        Ok(Self {
            function,
            arguments,
        })
    }
}

impl AstNode for CallExpression {
    fn evaluate(&self) -> Return<Object> {
        todo!()
    }
}

impl Display for CallExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}({})",
            self.function.to_string(),
            self.arguments
                .iter()
                .map(|arg| arg.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

#[cfg(test)]
mod test {
    use crate::token::{
        AsteriskToken, CommaToken, EOFToken, IdentToken, IntToken, LeftParenToken, PlusToken,
        RightParenToken,
    };

    use super::*;

    #[test]
    fn call_expression_no_args() {
        let mut tokens = [
            Token::LeftParen(LeftParenToken),
            Token::RightParen(RightParenToken),
            Token::EOF(EOFToken),
        ]
        .into_iter()
        .peekable();

        let result = CallExpression::parse_with_left(
            Expression::Identifier(Identifier {
                value: "add".to_string(),
                ident_token: IdentToken {
                    literal: "add".to_string(),
                },
            }),
            &mut tokens,
        );
        assert!(matches!(
            result,
            Ok(CallExpression {
                function: CallableFunction::Identifier(_),
                ..
            })
        ));

        if let Ok(CallExpression {
            function: CallableFunction::Identifier(ident),
            arguments,
        }) = result
        {
            assert_eq!(ident.value, "add");
            assert!(arguments.is_empty());
        }

        assert_eq!(tokens.count(), 1);
    }

    #[test]
    fn call_expression_single_arg() {
        let mut tokens = [
            Token::LeftParen(LeftParenToken),
            Token::Int(IntToken {
                literal: "1".to_string(),
            }),
            Token::RightParen(RightParenToken),
            Token::EOF(EOFToken),
        ]
        .into_iter()
        .peekable();

        let result = CallExpression::parse_with_left(
            Expression::Identifier(Identifier {
                value: "add".to_string(),
                ident_token: IdentToken {
                    literal: "add".to_string(),
                },
            }),
            &mut tokens,
        );
        assert!(matches!(
            result,
            Ok(CallExpression {
                function: CallableFunction::Identifier(_),
                ..
            })
        ));

        if let Ok(CallExpression {
            function: CallableFunction::Identifier(ident),
            arguments,
        }) = result
        {
            assert_eq!(ident.value, "add");
            assert_eq!(arguments.len(), 1);
            assert_eq!(arguments[0].to_string(), "1");
        }

        assert_eq!(tokens.count(), 1);
    }

    #[test]
    fn call_expression_single_arg_trailling_comma() {
        let mut tokens = [
            Token::LeftParen(LeftParenToken),
            Token::Int(IntToken {
                literal: "1".to_string(),
            }),
            Token::Comma(CommaToken),
            Token::RightParen(RightParenToken),
            Token::EOF(EOFToken),
        ]
        .into_iter()
        .peekable();

        let result = CallExpression::parse_with_left(
            Expression::Identifier(Identifier {
                value: "add".to_string(),
                ident_token: IdentToken {
                    literal: "add".to_string(),
                },
            }),
            &mut tokens,
        );
        assert!(matches!(
            result,
            Ok(CallExpression {
                function: CallableFunction::Identifier(_),
                ..
            })
        ));

        if let Ok(CallExpression {
            function: CallableFunction::Identifier(ident),
            arguments,
        }) = result
        {
            assert_eq!(ident.value, "add");
            assert_eq!(arguments.len(), 1);
            assert_eq!(arguments[0].to_string(), "1");
        }

        assert_eq!(tokens.count(), 1);
    }

    #[test]
    fn call_expression_many_args() {
        let mut tokens = [
            Token::LeftParen(LeftParenToken),
            Token::Int(IntToken {
                literal: "1".to_string(),
            }),
            Token::Comma(CommaToken),
            Token::Int(IntToken {
                literal: "2".to_string(),
            }),
            Token::Asterisk(AsteriskToken),
            Token::Int(IntToken {
                literal: "3".to_string(),
            }),
            Token::Comma(CommaToken),
            Token::Int(IntToken {
                literal: "4".to_string(),
            }),
            Token::Plus(PlusToken),
            Token::Int(IntToken {
                literal: "5".to_string(),
            }),
            Token::RightParen(RightParenToken),
            Token::EOF(EOFToken),
        ]
        .into_iter()
        .peekable();

        let result = CallExpression::parse_with_left(
            Expression::Identifier(Identifier {
                value: "add".to_string(),
                ident_token: IdentToken {
                    literal: "add".to_string(),
                },
            }),
            &mut tokens,
        );
        assert!(matches!(
            result,
            Ok(CallExpression {
                function: CallableFunction::Identifier(_),
                ..
            })
        ));

        if let Ok(CallExpression {
            function: CallableFunction::Identifier(ident),
            arguments,
        }) = result
        {
            assert_eq!(ident.value, "add");
            assert_eq!(arguments.len(), 3);
            assert_eq!(arguments[0].to_string(), "1");
            assert_eq!(arguments[1].to_string(), "(2 * 3)");
            assert_eq!(arguments[2].to_string(), "(4 + 5)");
        }

        assert_eq!(tokens.count(), 1);
    }

    #[test]
    fn call_expression_many_args_trailling_comma() {
        let mut tokens = [
            Token::LeftParen(LeftParenToken),
            Token::Int(IntToken {
                literal: "1".to_string(),
            }),
            Token::Comma(CommaToken),
            Token::Int(IntToken {
                literal: "2".to_string(),
            }),
            Token::Asterisk(AsteriskToken),
            Token::Int(IntToken {
                literal: "3".to_string(),
            }),
            Token::Comma(CommaToken),
            Token::Int(IntToken {
                literal: "4".to_string(),
            }),
            Token::Plus(PlusToken),
            Token::Int(IntToken {
                literal: "5".to_string(),
            }),
            Token::Comma(CommaToken),
            Token::RightParen(RightParenToken),
            Token::EOF(EOFToken),
        ]
        .into_iter()
        .peekable();

        let result = CallExpression::parse_with_left(
            Expression::Identifier(Identifier {
                value: "add".to_string(),
                ident_token: IdentToken {
                    literal: "add".to_string(),
                },
            }),
            &mut tokens,
        );
        assert!(matches!(
            result,
            Ok(CallExpression {
                function: CallableFunction::Identifier(_),
                ..
            })
        ));

        if let Ok(CallExpression {
            function: CallableFunction::Identifier(ident),
            arguments,
        }) = result
        {
            assert_eq!(ident.value, "add");
            assert_eq!(arguments.len(), 3);
            assert_eq!(arguments[0].to_string(), "1");
            assert_eq!(arguments[1].to_string(), "(2 * 3)");
            assert_eq!(arguments[2].to_string(), "(4 + 5)");
        }

        assert_eq!(tokens.count(), 1);
    }
}
