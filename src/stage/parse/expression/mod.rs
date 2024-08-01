use std::iter;

use super::*;

use self::{
    e_boolean::parse_boolean, e_ident::parse_ident, e_if::parse_if, e_integer::parse_integer,
};

mod e_boolean;
mod e_ident;
mod e_if;
mod e_integer;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    Lowest,
    Equality,
    Sum,
    Call,
}

impl Precedence {
    pub fn of(token: &Token) -> Self {
        match token {
            Token::Minus(_) | Token::Plus(_) => Precedence::Sum,
            Token::Eq(_) | Token::NotEq(_) => Precedence::Equality,
            Token::LeftParen(_) => Precedence::Call,
            _ => Precedence::Lowest,
        }
    }
}

fn parse_prefix(
    ctx: &mut impl ParseCtx,
    tokens: &mut impl TokenGenerator,
) -> Result<Expression, ParseError> {
    match tokens.peek_token() {
        Token::Integer(_) => Ok(Expression::Integer(parse_integer(ctx, tokens)?)),
        Token::Ident(_) => Ok(Expression::Ident(parse_ident(ctx, tokens)?)),
        Token::True(_) => Ok(Expression::Boolean(parse_boolean(ctx, tokens)?)),
        Token::False(_) => Ok(Expression::Boolean(parse_boolean(ctx, tokens)?)),
        Token::LeftBrace(_) => Ok(Expression::Block(parse_block(ctx, tokens)?)),
        Token::If(_) => Ok(Expression::If(parse_if(ctx, tokens)?)),
        token => Err(ParseError::UnexpectedToken(token)),
    }
}

pub fn parse_expression(
    ctx: &mut impl ParseCtx,
    tokens: &mut impl TokenGenerator,
    precedence: Precedence,
) -> Result<Expression, ParseError> {
    let mut left = parse_prefix(ctx, tokens)?;

    while !matches!(tokens.peek_token(), Token::EOF(_))
        && precedence < Precedence::of(&tokens.peek_token())
    {
        match (tokens.peek_token(), &left) {
            // Function call
            (Token::LeftParen(_), Expression::Ident(name)) => {
                // Consume the args
                let args = iter::from_fn(|| {
                    match tokens.peek_token() {
                        Token::RightParen(_) => None,
                        Token::LeftParen(_) | Token::Comma(_) => {
                            // Consume the opening paren or comma
                            tokens.next_token();

                            // If the closing parenthesis is encountered, stop parsing arguments
                            if matches!(tokens.peek_token(), Token::RightParen(_)) {
                                return None;
                            }

                            // Parse the next argument
                            Some(parse_expression(ctx, tokens, Precedence::Lowest))
                        }
                        token => Some(Err(ParseError::ExpectedToken {
                            expected: Box::new(Token::comma()),
                            found: Box::new(token),
                            reason: "function arguments must be separated by a comma".to_string(),
                        })),
                    }
                })
                .collect::<Result<Vec<_>, _>>()?;

                // Consume the closing paren
                let right_paren = match tokens.next_token() {
                    Token::RightParen(token) => token,
                    token => {
                        return Err(ParseError::ExpectedToken {
                            expected: Box::new(Token::right_paren()),
                            found: Box::new(token),
                            reason: "argument list must end with right paren".to_string(),
                        })
                    }
                };

                let span = name.span().to(right_paren.span());
                left = Expression::Call(Call::new(name.binding, args, span));
            }
            // Regular infix operation
            (token, _) => {
                if let Ok(operation) = InfixOperation::try_from(token) {
                    let token = tokens.next_token();
                    let precedence = Precedence::of(&token);

                    let right = parse_expression(ctx, tokens, precedence)?;

                    let span = token.span().to(&right);
                    left = Expression::Infix(Infix::new(
                        Box::new(left),
                        operation,
                        Box::new(right),
                        span,
                    ));
                } else {
                    // Probably aren't in the expression any more
                    break;
                }
            }
        }
    }

    Ok(left)
}

#[cfg(test)]
mod test {
    use ctx::MockParseCtx;
    use string_interner::Symbol as _;

    use crate::util::symbol_map::interner_symbol_map::Symbol;

    use super::*;

    use rstest::*;

    #[fixture]
    fn mock_ctx(#[default("func")] ident: &'static str) -> MockParseCtx {
        let mut ctx = MockParseCtx::new();

        ctx.expect_intern()
            .once()
            .withf(move |s| s.as_ref() == ident)
            .return_const(Symbol::try_from_usize(0).unwrap());

        ctx
    }

    #[rstest]
    fn simple_addition() {
        let expression = parse_expression(
            &mut MockParseCtx::new(),
            &mut [Token::integer("3"), Token::plus(), Token::integer("4")]
                .into_iter()
                .peekable(),
            Precedence::Lowest,
        )
        .unwrap();

        insta::assert_debug_snapshot!(expression, @r###"
        Infix(
            Infix {
                span: 1:0 -> 1:0,
                left: Integer(
                    Integer {
                        span: 1:0 -> 1:0,
                        value: 3,
                        ty_info: None,
                    },
                ),
                operation: Plus(
                    1:0 -> 1:0,
                ),
                right: Integer(
                    Integer {
                        span: 1:0 -> 1:0,
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
            &mut MockParseCtx::new(),
            &mut [
                Token::integer("3"),
                Token::plus(),
                Token::integer("4"),
                Token::plus(),
                Token::integer("10"),
            ]
            .into_iter()
            .peekable(),
            Precedence::Lowest,
        )
        .unwrap();

        insta::assert_debug_snapshot!(expression, @r###"
        Infix(
            Infix {
                span: 1:0 -> 1:0,
                left: Infix(
                    Infix {
                        span: 1:0 -> 1:0,
                        left: Integer(
                            Integer {
                                span: 1:0 -> 1:0,
                                value: 3,
                                ty_info: None,
                            },
                        ),
                        operation: Plus(
                            1:0 -> 1:0,
                        ),
                        right: Integer(
                            Integer {
                                span: 1:0 -> 1:0,
                                value: 4,
                                ty_info: None,
                            },
                        ),
                        ty_info: None,
                    },
                ),
                operation: Plus(
                    1:0 -> 1:0,
                ),
                right: Integer(
                    Integer {
                        span: 1:0 -> 1:0,
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
        let mut ctx = MockParseCtx::new();

        ctx.expect_intern()
            .once()
            .withf(|f| f.as_ref() == "someident")
            .return_const(Symbol::try_from_usize(0).unwrap());

        let expression = parse_expression(
            &mut ctx,
            &mut [
                Token::t_if(),
                Token::integer("1"),
                Token::left_brace(),
                Token::ident("someident"),
                Token::right_brace(),
            ]
            .into_iter()
            .peekable(),
            Precedence::Lowest,
        )
        .unwrap();

        insta::assert_debug_snapshot!(expression, @r###"
        If(
            If {
                span: 1:0 -> 1:0,
                condition: Integer(
                    Integer {
                        span: 1:0 -> 1:0,
                        value: 1,
                        ty_info: None,
                    },
                ),
                success: Block {
                    span: 1:0 -> 1:0,
                    statements: [
                        Expression(
                            ExpressionStatement {
                                span: 1:0 -> 1:0,
                                expression: Ident(
                                    Ident {
                                        span: 1:0 -> 1:0,
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
            &mut MockParseCtx::new(),
            &mut [Token::integer("1")].into_iter().peekable(),
            Precedence::Lowest,
        )
        .unwrap();
        insta::assert_debug_snapshot!(expression, @r###"
        Integer(
            Integer {
                span: 1:0 -> 1:0,
                value: 1,
                ty_info: None,
            },
        )
        "###);
    }

    #[rstest]
    fn ident(#[with("someident")] mut mock_ctx: MockParseCtx) {
        let expression = parse_expression(
            &mut mock_ctx,
            &mut [Token::ident("someident")].into_iter().peekable(),
            Precedence::Lowest,
        )
        .unwrap();
        insta::assert_debug_snapshot!(expression, @r###"
        Ident(
            Ident {
                span: 1:0 -> 1:0,
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
            &mut MockParseCtx::new(),
            &mut [Token::integer("1"), Token::eq(), Token::integer("1")]
                .into_iter()
                .peekable(),
            Precedence::Lowest,
        )
        .unwrap();

        insta::assert_debug_snapshot!(expression, @r###"
        Infix(
            Infix {
                span: 1:0 -> 1:0,
                left: Integer(
                    Integer {
                        span: 1:0 -> 1:0,
                        value: 1,
                        ty_info: None,
                    },
                ),
                operation: Eq(
                    1:0 -> 1:0,
                ),
                right: Integer(
                    Integer {
                        span: 1:0 -> 1:0,
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
            &mut MockParseCtx::new(),
            &mut [
                Token::integer("1"),
                Token::eq(),
                Token::integer("1"),
                Token::plus(),
                Token::integer("2"),
            ]
            .into_iter()
            .peekable(),
            Precedence::Lowest,
        )
        .unwrap();

        insta::assert_debug_snapshot!(expression, @r###"
        Infix(
            Infix {
                span: 1:0 -> 1:0,
                left: Integer(
                    Integer {
                        span: 1:0 -> 1:0,
                        value: 1,
                        ty_info: None,
                    },
                ),
                operation: Eq(
                    1:0 -> 1:0,
                ),
                right: Infix(
                    Infix {
                        span: 1:0 -> 1:0,
                        left: Integer(
                            Integer {
                                span: 1:0 -> 1:0,
                                value: 1,
                                ty_info: None,
                            },
                        ),
                        operation: Plus(
                            1:0 -> 1:0,
                        ),
                        right: Integer(
                            Integer {
                                span: 1:0 -> 1:0,
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
    fn function_call_no_param(mut mock_ctx: MockParseCtx) {
        let expression = parse_expression(
            &mut mock_ctx,
            &mut [
                Token::ident("func"),
                Token::left_paren(),
                Token::right_paren(),
            ]
            .into_iter()
            .peekable(),
            Precedence::Lowest,
        )
        .unwrap();

        insta::assert_debug_snapshot!(expression, @r###"
        Call(
            Call {
                span: 1:0 -> 1:0,
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
    fn function_call_one_param_no_comma(mut mock_ctx: MockParseCtx) {
        let expression = parse_expression(
            &mut mock_ctx,
            &mut [
                Token::ident("func"),
                Token::left_paren(),
                Token::integer("1"),
                Token::right_paren(),
            ]
            .into_iter()
            .peekable(),
            Precedence::Lowest,
        )
        .unwrap();

        insta::assert_debug_snapshot!(expression, @r###"
        Call(
            Call {
                span: 1:0 -> 1:0,
                name: SymbolU32 {
                    value: 1,
                },
                args: [
                    Integer(
                        Integer {
                            span: 1:0 -> 1:0,
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
    fn function_call_one_param_trailing_comma(mut mock_ctx: MockParseCtx) {
        let expression = parse_expression(
            &mut mock_ctx,
            &mut [
                Token::ident("func"),
                Token::left_paren(),
                Token::integer("1"),
                Token::comma(),
                Token::right_paren(),
            ]
            .into_iter()
            .peekable(),
            Precedence::Lowest,
        )
        .unwrap();

        insta::assert_debug_snapshot!(expression, @r###"
        Call(
            Call {
                span: 1:0 -> 1:0,
                name: SymbolU32 {
                    value: 1,
                },
                args: [
                    Integer(
                        Integer {
                            span: 1:0 -> 1:0,
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
    fn function_call_multi_param_no_comma(mut mock_ctx: MockParseCtx) {
        let expression = parse_expression(
            &mut mock_ctx,
            &mut [
                Token::ident("func"),
                Token::left_paren(),
                Token::integer("1"),
                Token::comma(),
                Token::integer("2"),
                Token::comma(),
                Token::integer("3"),
                Token::right_paren(),
            ]
            .into_iter()
            .peekable(),
            Precedence::Lowest,
        )
        .unwrap();

        insta::assert_debug_snapshot!(expression, @r###"
        Call(
            Call {
                span: 1:0 -> 1:0,
                name: SymbolU32 {
                    value: 1,
                },
                args: [
                    Integer(
                        Integer {
                            span: 1:0 -> 1:0,
                            value: 1,
                            ty_info: None,
                        },
                    ),
                    Integer(
                        Integer {
                            span: 1:0 -> 1:0,
                            value: 2,
                            ty_info: None,
                        },
                    ),
                    Integer(
                        Integer {
                            span: 1:0 -> 1:0,
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
    fn function_call_multi_param_trailing_comma(mut mock_ctx: MockParseCtx) {
        let expression = parse_expression(
            &mut mock_ctx,
            &mut [
                Token::ident("func"),
                Token::left_paren(),
                Token::integer("1"),
                Token::comma(),
                Token::integer("2"),
                Token::comma(),
                Token::integer("3"),
                Token::comma(),
                Token::right_paren(),
            ]
            .into_iter()
            .peekable(),
            Precedence::Lowest,
        )
        .unwrap();

        insta::assert_debug_snapshot!(expression, @r###"
        Call(
            Call {
                span: 1:0 -> 1:0,
                name: SymbolU32 {
                    value: 1,
                },
                args: [
                    Integer(
                        Integer {
                            span: 1:0 -> 1:0,
                            value: 1,
                            ty_info: None,
                        },
                    ),
                    Integer(
                        Integer {
                            span: 1:0 -> 1:0,
                            value: 2,
                            ty_info: None,
                        },
                    ),
                    Integer(
                        Integer {
                            span: 1:0 -> 1:0,
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
    fn function_call_with_expression_param(mut mock_ctx: MockParseCtx) {
        let expression = parse_expression(
            &mut mock_ctx,
            &mut [
                Token::ident("func"),
                Token::left_paren(),
                Token::integer("1"),
                Token::plus(),
                Token::integer("2"),
                Token::comma(),
                Token::integer("3"),
                Token::comma(),
                Token::right_paren(),
            ]
            .into_iter()
            .peekable(),
            Precedence::Lowest,
        )
        .unwrap();

        insta::assert_debug_snapshot!(expression, @r###"
        Call(
            Call {
                span: 1:0 -> 1:0,
                name: SymbolU32 {
                    value: 1,
                },
                args: [
                    Infix(
                        Infix {
                            span: 1:0 -> 1:0,
                            left: Integer(
                                Integer {
                                    span: 1:0 -> 1:0,
                                    value: 1,
                                    ty_info: None,
                                },
                            ),
                            operation: Plus(
                                1:0 -> 1:0,
                            ),
                            right: Integer(
                                Integer {
                                    span: 1:0 -> 1:0,
                                    value: 2,
                                    ty_info: None,
                                },
                            ),
                            ty_info: None,
                        },
                    ),
                    Integer(
                        Integer {
                            span: 1:0 -> 1:0,
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
    fn function_call_in_expression(mut mock_ctx: MockParseCtx) {
        let expression = parse_expression(
            &mut mock_ctx,
            &mut [
                Token::ident("func"),
                Token::left_paren(),
                Token::integer("1"),
                Token::right_paren(),
                Token::plus(),
                Token::integer("2"),
            ]
            .into_iter()
            .peekable(),
            Precedence::Lowest,
        )
        .unwrap();

        insta::assert_debug_snapshot!(expression, @r###"
        Infix(
            Infix {
                span: 1:0 -> 1:0,
                left: Call(
                    Call {
                        span: 1:0 -> 1:0,
                        name: SymbolU32 {
                            value: 1,
                        },
                        args: [
                            Integer(
                                Integer {
                                    span: 1:0 -> 1:0,
                                    value: 1,
                                    ty_info: None,
                                },
                            ),
                        ],
                        ty_info: None,
                    },
                ),
                operation: Plus(
                    1:0 -> 1:0,
                ),
                right: Integer(
                    Integer {
                        span: 1:0 -> 1:0,
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
