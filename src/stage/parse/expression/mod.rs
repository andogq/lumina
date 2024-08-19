use std::iter;

use e_loop::parse_loop;

use super::*;

use self::{
    e_boolean::parse_boolean, e_ident::parse_ident, e_if::parse_if, e_integer::parse_integer,
};

mod e_boolean;
mod e_ident;
mod e_if;
mod e_integer;
mod e_loop;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    Lowest,
    Binary,
    Equality,
    Sum,
    Call,
}

impl Precedence {
    pub fn of(token: &Token) -> Self {
        match token {
            Token::Minus | Token::Plus => Precedence::Sum,
            Token::And | Token::Or => Precedence::Binary,
            Token::DoubleEq | Token::NotEq => Precedence::Equality,
            Token::LeftParen => Precedence::Call,
            _ => Precedence::Lowest,
        }
    }
}

fn parse_prefix(compiler: &mut Compiler, tokens: &mut Lexer<'_>) -> Result<Expression, ParseError> {
    match tokens.peek_token().unwrap() {
        Token::Integer(_) => Ok(Expression::Integer(parse_integer(compiler, tokens)?)),
        Token::Ident(_) => Ok(Expression::Ident(parse_ident(compiler, tokens)?)),
        Token::True => Ok(Expression::Boolean(parse_boolean(compiler, tokens)?)),
        Token::False => Ok(Expression::Boolean(parse_boolean(compiler, tokens)?)),
        Token::LeftBrace => Ok(Expression::Block(parse_block(compiler, tokens)?)),
        Token::If => Ok(Expression::If(parse_if(compiler, tokens)?)),
        Token::Loop => Ok(Expression::Loop(parse_loop(compiler, tokens)?)),
        token => Err(ParseError::UnexpectedToken(token.clone())),
    }
}

pub fn parse_expression(
    compiler: &mut Compiler,
    tokens: &mut Lexer<'_>,
    precedence: Precedence,
) -> Result<Expression, ParseError> {
    let mut left = parse_prefix(compiler, tokens)?;

    while tokens.peek_token().is_some() && precedence < Precedence::of(tokens.peek_token().unwrap())
    {
        left = match (left, tokens.peek_token().unwrap()) {
            // Function call
            (Expression::Ident(name), Token::LeftParen) => {
                // Consume the args
                let args = iter::from_fn(|| {
                    match tokens.peek_token().unwrap() {
                        Token::RightParen => None,
                        Token::LeftParen | Token::Comma => {
                            // Consume the opening paren or comma
                            tokens.next_token();

                            // If the closing parenthesis is encountered, stop parsing arguments
                            if matches!(tokens.peek_token().unwrap(), Token::RightParen) {
                                return None;
                            }

                            // Parse the next argument
                            Some(parse_expression(compiler, tokens, Precedence::Lowest))
                        }
                        token => Some(Err(ParseError::ExpectedToken {
                            expected: Box::new(Token::Comma),
                            found: Box::new(token.clone()),
                            reason: "function arguments must be separated by a comma".to_string(),
                        })),
                    }
                })
                .collect::<Result<Vec<_>, _>>()?;

                // Consume the closing paren
                let end_span = match tokens.next_spanned().unwrap() {
                    (Token::RightParen, span) => span,
                    (token, _) => {
                        return Err(ParseError::ExpectedToken {
                            expected: Box::new(Token::RightParen),
                            found: Box::new(token),
                            reason: "argument list must end with right paren".to_string(),
                        })
                    }
                };

                let span = name.span.start..end_span.end;
                Expression::Call(Call::new(name.binding, args, span))
            }
            // Regular infix operation
            (left, token) => {
                if let Ok(operation) = InfixOperation::try_from(token.clone()) {
                    let token = tokens.next_token().unwrap();
                    let precedence = Precedence::of(&token);

                    let right = parse_expression(compiler, tokens, precedence)?;

                    let span = left.span().start..right.span().end;

                    Expression::Infix(Infix::new(Box::new(left), operation, Box::new(right), span))
                } else {
                    // Probably aren't in the expression any more
                    return Ok(left);
                }
            }
        };
    }

    Ok(left)
}

#[cfg(test)]
mod test {

    use super::*;

    use rstest::*;

    #[fixture]
    fn mock_compiler(#[default("func")] ident: &'static str) -> Compiler {
        let mut compiler = Compiler::default();

        compiler.symbols.get_or_intern(ident);

        compiler
    }

    #[rstest]
    fn simple_addition() {
        let expression = parse_expression(
            &mut Compiler::default(),
            &mut "3 + 4".into(),
            Precedence::Lowest,
        )
        .unwrap();

        insta::assert_debug_snapshot!(expression, @r###"
        Infix(
            Infix {
                span: 0..5,
                left: Integer(
                    Integer {
                        span: 0..1,
                        value: 3,
                        ty_info: None,
                    },
                ),
                operation: Plus,
                right: Integer(
                    Integer {
                        span: 4..5,
                        value: 4,
                        ty_info: None,
                    },
                ),
                ty_info: None,
            },
        )
        "###);
    }

    #[rstest]
    fn multi_addition() {
        let expression = parse_expression(
            &mut Compiler::default(),
            &mut "3 + 4 + 10".into(),
            Precedence::Lowest,
        )
        .unwrap();

        insta::assert_debug_snapshot!(expression, @r###"
        Infix(
            Infix {
                span: 0..10,
                left: Infix(
                    Infix {
                        span: 0..5,
                        left: Integer(
                            Integer {
                                span: 0..1,
                                value: 3,
                                ty_info: None,
                            },
                        ),
                        operation: Plus,
                        right: Integer(
                            Integer {
                                span: 4..5,
                                value: 4,
                                ty_info: None,
                            },
                        ),
                        ty_info: None,
                    },
                ),
                operation: Plus,
                right: Integer(
                    Integer {
                        span: 8..10,
                        value: 10,
                        ty_info: None,
                    },
                ),
                ty_info: None,
            },
        )
        "###);
    }

    #[rstest]
    fn if_statement() {
        let mut compiler = Compiler::default();

        let expression = parse_expression(
            &mut compiler,
            &mut "if 1 { someident }".into(),
            Precedence::Lowest,
        )
        .unwrap();

        assert!(compiler.symbols.get("someident").is_some());

        insta::assert_debug_snapshot!(expression, @r###"
        If(
            If {
                span: 0..18,
                condition: Integer(
                    Integer {
                        span: 3..4,
                        value: 1,
                        ty_info: None,
                    },
                ),
                success: Block {
                    span: 5..18,
                    statements: [
                        Expression(
                            ExpressionStatement {
                                span: 7..16,
                                expression: Ident(
                                    Ident {
                                        span: 7..16,
                                        binding: SymbolU32 {
                                            value: 1,
                                        },
                                        ty_info: None,
                                    },
                                ),
                                implicit_return: true,
                                ty_info: None,
                            },
                        ),
                    ],
                    ty_info: None,
                },
                otherwise: None,
                ty_info: None,
            },
        )
        "###);
    }

    #[rstest]
    fn integer() {
        let expression = parse_expression(
            &mut Compiler::default(),
            &mut "1".into(),
            Precedence::Lowest,
        )
        .unwrap();
        insta::assert_debug_snapshot!(expression, @r###"
        Integer(
            Integer {
                span: 0..1,
                value: 1,
                ty_info: None,
            },
        )
        "###);
    }

    #[rstest]
    fn ident(#[with("someident")] mut mock_compiler: Compiler) {
        let expression = parse_expression(
            &mut mock_compiler,
            &mut "someident".into(),
            Precedence::Lowest,
        )
        .unwrap();
        insta::assert_debug_snapshot!(expression, @r###"
        Ident(
            Ident {
                span: 0..9,
                binding: SymbolU32 {
                    value: 1,
                },
                ty_info: None,
            },
        )
        "###);
    }

    #[rstest]
    fn equality() {
        let expression = parse_expression(
            &mut Compiler::default(),
            &mut "1 == 1".into(),
            Precedence::Lowest,
        )
        .unwrap();

        insta::assert_debug_snapshot!(expression, @r###"
        Infix(
            Infix {
                span: 0..6,
                left: Integer(
                    Integer {
                        span: 0..1,
                        value: 1,
                        ty_info: None,
                    },
                ),
                operation: Eq,
                right: Integer(
                    Integer {
                        span: 5..6,
                        value: 1,
                        ty_info: None,
                    },
                ),
                ty_info: None,
            },
        )
        "###);
    }

    #[rstest]
    fn complex_equality() {
        let expression = parse_expression(
            &mut Compiler::default(),
            &mut "1 == 1 + 2".into(),
            Precedence::Lowest,
        )
        .unwrap();

        insta::assert_debug_snapshot!(expression, @r###"
        Infix(
            Infix {
                span: 0..10,
                left: Integer(
                    Integer {
                        span: 0..1,
                        value: 1,
                        ty_info: None,
                    },
                ),
                operation: Eq,
                right: Infix(
                    Infix {
                        span: 5..10,
                        left: Integer(
                            Integer {
                                span: 5..6,
                                value: 1,
                                ty_info: None,
                            },
                        ),
                        operation: Plus,
                        right: Integer(
                            Integer {
                                span: 9..10,
                                value: 2,
                                ty_info: None,
                            },
                        ),
                        ty_info: None,
                    },
                ),
                ty_info: None,
            },
        )
        "###);
    }

    #[rstest]
    fn function_call_no_param(mut mock_compiler: Compiler) {
        let expression =
            parse_expression(&mut mock_compiler, &mut "func()".into(), Precedence::Lowest).unwrap();

        insta::assert_debug_snapshot!(expression, @r###"
        Call(
            Call {
                span: 0..6,
                name: SymbolU32 {
                    value: 1,
                },
                args: [],
                ty_info: None,
            },
        )
        "###);
    }

    #[rstest]
    fn function_call_one_param_no_comma(mut mock_compiler: Compiler) {
        let expression = parse_expression(
            &mut mock_compiler,
            &mut "func(1)".into(),
            Precedence::Lowest,
        )
        .unwrap();

        insta::assert_debug_snapshot!(expression, @r###"
        Call(
            Call {
                span: 0..7,
                name: SymbolU32 {
                    value: 1,
                },
                args: [
                    Integer(
                        Integer {
                            span: 5..6,
                            value: 1,
                            ty_info: None,
                        },
                    ),
                ],
                ty_info: None,
            },
        )
        "###);
    }

    #[rstest]
    fn function_call_one_param_trailing_comma(mut mock_compiler: Compiler) {
        let expression = parse_expression(
            &mut mock_compiler,
            &mut "func(1,)".into(),
            Precedence::Lowest,
        )
        .unwrap();

        insta::assert_debug_snapshot!(expression, @r###"
        Call(
            Call {
                span: 0..8,
                name: SymbolU32 {
                    value: 1,
                },
                args: [
                    Integer(
                        Integer {
                            span: 5..6,
                            value: 1,
                            ty_info: None,
                        },
                    ),
                ],
                ty_info: None,
            },
        )
        "###);
    }

    #[rstest]
    fn function_call_multi_param_no_comma(mut mock_compiler: Compiler) {
        let expression = parse_expression(
            &mut mock_compiler,
            &mut "func(1, 2, 3)".into(),
            Precedence::Lowest,
        )
        .unwrap();

        insta::assert_debug_snapshot!(expression, @r###"
        Call(
            Call {
                span: 0..13,
                name: SymbolU32 {
                    value: 1,
                },
                args: [
                    Integer(
                        Integer {
                            span: 5..6,
                            value: 1,
                            ty_info: None,
                        },
                    ),
                    Integer(
                        Integer {
                            span: 8..9,
                            value: 2,
                            ty_info: None,
                        },
                    ),
                    Integer(
                        Integer {
                            span: 11..12,
                            value: 3,
                            ty_info: None,
                        },
                    ),
                ],
                ty_info: None,
            },
        )
        "###);
    }

    #[rstest]
    fn function_call_multi_param_trailing_comma(mut mock_compiler: Compiler) {
        let expression = parse_expression(
            &mut mock_compiler,
            &mut "func(1, 2, 3,)".into(),
            Precedence::Lowest,
        )
        .unwrap();

        insta::assert_debug_snapshot!(expression, @r###"
        Call(
            Call {
                span: 0..14,
                name: SymbolU32 {
                    value: 1,
                },
                args: [
                    Integer(
                        Integer {
                            span: 5..6,
                            value: 1,
                            ty_info: None,
                        },
                    ),
                    Integer(
                        Integer {
                            span: 8..9,
                            value: 2,
                            ty_info: None,
                        },
                    ),
                    Integer(
                        Integer {
                            span: 11..12,
                            value: 3,
                            ty_info: None,
                        },
                    ),
                ],
                ty_info: None,
            },
        )
        "###);
    }

    #[rstest]
    fn function_call_with_expression_param(mut mock_compiler: Compiler) {
        let expression = parse_expression(
            &mut mock_compiler,
            &mut "func(1 + 2, 3,)".into(),
            Precedence::Lowest,
        )
        .unwrap();

        insta::assert_debug_snapshot!(expression, @r###"
        Call(
            Call {
                span: 0..15,
                name: SymbolU32 {
                    value: 1,
                },
                args: [
                    Infix(
                        Infix {
                            span: 5..10,
                            left: Integer(
                                Integer {
                                    span: 5..6,
                                    value: 1,
                                    ty_info: None,
                                },
                            ),
                            operation: Plus,
                            right: Integer(
                                Integer {
                                    span: 9..10,
                                    value: 2,
                                    ty_info: None,
                                },
                            ),
                            ty_info: None,
                        },
                    ),
                    Integer(
                        Integer {
                            span: 12..13,
                            value: 3,
                            ty_info: None,
                        },
                    ),
                ],
                ty_info: None,
            },
        )
        "###);
    }

    #[rstest]
    fn function_call_in_expression(mut mock_compiler: Compiler) {
        let expression = parse_expression(
            &mut mock_compiler,
            &mut "func(1) + 2".into(),
            Precedence::Lowest,
        )
        .unwrap();

        insta::assert_debug_snapshot!(expression, @r###"
        Infix(
            Infix {
                span: 0..11,
                left: Call(
                    Call {
                        span: 0..7,
                        name: SymbolU32 {
                            value: 1,
                        },
                        args: [
                            Integer(
                                Integer {
                                    span: 5..6,
                                    value: 1,
                                    ty_info: None,
                                },
                            ),
                        ],
                        ty_info: None,
                    },
                ),
                operation: Plus,
                right: Integer(
                    Integer {
                        span: 10..11,
                        value: 2,
                        ty_info: None,
                    },
                ),
                ty_info: None,
            },
        )
        "###);
    }
}
