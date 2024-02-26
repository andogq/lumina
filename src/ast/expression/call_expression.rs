use std::fmt::{Display, Formatter};

use crate::{
    ast::{AstNode, ParseNode},
    interpreter::{environment::Environment, error::Error, object::Object, return_value::Return},
    lexer::Lexer,
    return_value,
    token::{LeftParenToken, Token},
};

use super::{Expression, FunctionLiteral, Identifier};

#[derive(Clone, Debug, PartialEq)]
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

#[derive(Clone, Debug, PartialEq)]
pub struct CallExpression {
    pub function: CallableFunction,
    pub arguments: Vec<Expression>,
}

impl CallExpression {
    pub fn parse_with_left<S>(left: Expression, lexer: &mut Lexer<S>) -> Result<Self, String>
    where
        S: Iterator<Item = char>,
    {
        let function = match left {
            Expression::Identifier(ident) => Some(CallableFunction::Identifier(ident)),
            Expression::Function(function) => Some(CallableFunction::FunctionLiteral(function)),
            _ => None,
        }
        .ok_or_else(|| "expected ident or function literal".to_string())?;

        // Begin function argument list
        lexer.get::<LeftParenToken>("to begin argument list")?;

        // Parse out arguments
        let mut arguments = Vec::new();

        while !matches!(lexer.peek(), Token::RightParen(_)) {
            // Parse out the argument
            arguments.push(Expression::parse(lexer)?);

            if !lexer
                // Attempt to get a comma
                .next_if(|token| matches!(token, Token::Comma(_)))
                .map(|_| true)
                // Otherwise make sure a right parenthesis follows
                .unwrap_or_else(|| matches!(lexer.peek(), Token::RightParen(_)))
            {
                return Err("expected a comma or right parenthesis to follow".to_string());
            }
        }

        lexer
            .next_if(|token| matches!(token, Token::RightParen(_)))
            .ok_or_else(|| "expected right parenthesis to close argument list".to_string())?;

        Ok(Self {
            function,
            arguments,
        })
    }
}

impl AstNode for CallExpression {
    fn evaluate(&self, env: Environment) -> Return<Object> {
        let Object::Function(function) = return_value!(match &self.function {
            CallableFunction::Identifier(ident) => ident.evaluate(env.clone()),
            CallableFunction::FunctionLiteral(lit) => lit.evaluate(env.clone()),
        }) else {
            return Error::throw("value is not of type function");
        };

        let function_env = env.nest();

        // Evaluate all arguments and set them in the environment
        for (arg, param) in self
            .arguments
            .iter()
            .map(|arg| arg.evaluate(env.clone()))
            .zip(function.parameters)
        {
            match arg {
                Return::Implicit(value) | Return::Explicit(value) => {
                    function_env.set(param.value, value)
                }
                Return::Error(err) => return Return::Error(err),
            }
        }

        match function.body.evaluate(function_env) {
            Return::Explicit(value) | Return::Implicit(value) => Return::Implicit(value),
            Return::Error(err) => Return::Error(err),
        }
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
        AsteriskToken, CommaToken, IdentToken, IntToken, LeftParenToken, PlusToken,
        RightParenToken, SemicolonToken,
    };

    use super::*;

    #[test]
    fn call_expression_no_args() {
        let mut lexer = Lexer::from_tokens([
            Token::LeftParen(LeftParenToken::default()),
            Token::RightParen(RightParenToken::default()),
            Token::Semicolon(SemicolonToken::default()),
        ]);

        let result = CallExpression::parse_with_left(
            Expression::Identifier(Identifier {
                value: "add".to_string(),
                ident_token: IdentToken {
                    literal: "add".to_string(),
                    ..Default::default()
                },
            }),
            &mut lexer,
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

        assert!(matches!(lexer.next(), Token::Semicolon(_)));
    }

    #[test]
    fn call_expression_single_arg() {
        let mut lexer = Lexer::from_tokens([
            Token::LeftParen(LeftParenToken::default()),
            Token::Int(IntToken {
                literal: "1".to_string(),
                ..Default::default()
            }),
            Token::RightParen(RightParenToken::default()),
            Token::Semicolon(SemicolonToken::default()),
        ]);

        let result = CallExpression::parse_with_left(
            Expression::Identifier(Identifier {
                value: "add".to_string(),
                ident_token: IdentToken {
                    literal: "add".to_string(),
                    ..Default::default()
                },
            }),
            &mut lexer,
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

        assert!(matches!(lexer.next(), Token::Semicolon(_)));
    }

    #[test]
    fn call_expression_single_arg_trailling_comma() {
        let mut lexer = Lexer::from_tokens([
            Token::LeftParen(LeftParenToken::default()),
            Token::Int(IntToken {
                literal: "1".to_string(),
                ..Default::default()
            }),
            Token::Comma(CommaToken::default()),
            Token::RightParen(RightParenToken::default()),
            Token::Semicolon(SemicolonToken::default()),
        ]);

        let result = CallExpression::parse_with_left(
            Expression::Identifier(Identifier {
                value: "add".to_string(),
                ident_token: IdentToken {
                    literal: "add".to_string(),
                    ..Default::default()
                },
            }),
            &mut lexer,
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

        assert!(matches!(lexer.next(), Token::Semicolon(_)));
    }

    #[test]
    fn call_expression_many_args() {
        let mut lexer = Lexer::from_tokens([
            Token::LeftParen(LeftParenToken::default()),
            Token::Int(IntToken {
                literal: "1".to_string(),
                ..Default::default()
            }),
            Token::Comma(CommaToken::default()),
            Token::Int(IntToken {
                literal: "2".to_string(),
                ..Default::default()
            }),
            Token::Asterisk(AsteriskToken::default()),
            Token::Int(IntToken {
                literal: "3".to_string(),
                ..Default::default()
            }),
            Token::Comma(CommaToken::default()),
            Token::Int(IntToken {
                literal: "4".to_string(),
                ..Default::default()
            }),
            Token::Plus(PlusToken::default()),
            Token::Int(IntToken {
                literal: "5".to_string(),
                ..Default::default()
            }),
            Token::RightParen(RightParenToken::default()),
            Token::Semicolon(SemicolonToken::default()),
        ]);

        let result = CallExpression::parse_with_left(
            Expression::Identifier(Identifier {
                value: "add".to_string(),
                ident_token: IdentToken {
                    literal: "add".to_string(),
                    ..Default::default()
                },
            }),
            &mut lexer,
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

        assert!(matches!(lexer.next(), Token::Semicolon(_)));
    }

    #[test]
    fn call_expression_many_args_trailling_comma() {
        let mut lexer = Lexer::from_tokens([
            Token::LeftParen(LeftParenToken::default()),
            Token::Int(IntToken {
                literal: "1".to_string(),
                ..Default::default()
            }),
            Token::Comma(CommaToken::default()),
            Token::Int(IntToken {
                literal: "2".to_string(),
                ..Default::default()
            }),
            Token::Asterisk(AsteriskToken::default()),
            Token::Int(IntToken {
                literal: "3".to_string(),
                ..Default::default()
            }),
            Token::Comma(CommaToken::default()),
            Token::Int(IntToken {
                literal: "4".to_string(),
                ..Default::default()
            }),
            Token::Plus(PlusToken::default()),
            Token::Int(IntToken {
                literal: "5".to_string(),
                ..Default::default()
            }),
            Token::Comma(CommaToken::default()),
            Token::RightParen(RightParenToken::default()),
            Token::Semicolon(SemicolonToken::default()),
        ]);

        let result = CallExpression::parse_with_left(
            Expression::Identifier(Identifier {
                value: "add".to_string(),
                ident_token: IdentToken {
                    literal: "add".to_string(),
                    ..Default::default()
                },
            }),
            &mut lexer,
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

        assert!(matches!(lexer.next(), Token::Semicolon(_)));
    }
}
