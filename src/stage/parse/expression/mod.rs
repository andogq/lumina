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
            Token::Plus(_) => Precedence::Sum,
            Token::Eq(_) | Token::NotEq(_) => Precedence::Equality,
            Token::LeftParen(_) => Precedence::Call,
            _ => Precedence::Lowest,
        }
    }
}

fn parse_prefix(ctx: &mut impl ParseCtxTrait) -> Result<Expression, ParseError> {
    match ctx.peek_token() {
        Token::Integer(_) => Ok(Expression::Integer(parse_integer(ctx)?)),
        Token::Ident(_) => Ok(Expression::Ident(parse_ident(ctx)?)),
        Token::True(_) => Ok(Expression::Boolean(parse_boolean(ctx)?)),
        Token::False(_) => Ok(Expression::Boolean(parse_boolean(ctx)?)),
        Token::LeftBrace(_) => Ok(Expression::Block(parse_block(ctx)?)),
        Token::If(_) => Ok(Expression::If(parse_if(ctx)?)),
        token => Err(ParseError::UnexpectedToken(token)),
    }
}

pub fn parse_expression(
    ctx: &mut impl ParseCtxTrait,
    precedence: Precedence,
) -> Result<Expression, ParseError> {
    let mut left = parse_prefix(ctx)?;

    while !matches!(ctx.peek_token(), Token::EOF(_))
        && precedence < Precedence::of(&ctx.peek_token())
    {
        match (ctx.peek_token(), &left) {
            // Function call
            (Token::LeftParen(_), Expression::Ident(name)) => {
                // Consume the args
                let args = iter::from_fn(|| {
                    match ctx.peek_token() {
                        Token::RightParen(_) => None,
                        Token::LeftParen(_) | Token::Comma(_) => {
                            // Consume the opening paren or comma
                            ctx.next_token();

                            // If the closing parenthesis is encountered, stop parsing arguments
                            if matches!(ctx.peek_token(), Token::RightParen(_)) {
                                return None;
                            }

                            // Parse the next argument
                            Some(parse_expression(ctx, Precedence::Lowest))
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
                let right_paren = match ctx.next_token() {
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
                left = Expression::Call(Call::new(name.name, args, span));
            }
            // Regular infix operation
            (token, _) => {
                if let Ok(operation) = InfixOperation::try_from(token) {
                    let token = ctx.next_token();
                    let precedence = Precedence::of(&token);

                    let right = parse_expression(ctx, precedence)?;

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

    use super::*;

    fn run(tokens: &[Token]) -> Result<Expression, ParseError> {
        parse_expression(&mut SimpleParseCtx::from(tokens), Precedence::Lowest)
    }

    #[test]
    fn simple_addition() {
        let expression = run(&[Token::integer("3"), Token::plus(), Token::integer("4")]).unwrap();

        insta::assert_debug_snapshot!(expression, @r###"
        Infix(
            Infix {
                span: 1:0 -> 1:0,
                ty_info: None,
                left: Integer(
                    Integer {
                        span: 1:0 -> 1:0,
                        ty_info: None,
                        value: 3,
                    },
                ),
                operation: Plus(
                    1:0 -> 1:0,
                ),
                right: Integer(
                    Integer {
                        span: 1:0 -> 1:0,
                        ty_info: None,
                        value: 4,
                    },
                ),
            },
        )
        "###);
    }

    #[test]
    fn multi_addition() {
        let expression = run(&[
            Token::integer("3"),
            Token::plus(),
            Token::integer("4"),
            Token::plus(),
            Token::integer("10"),
        ])
        .unwrap();

        insta::assert_debug_snapshot!(expression, @r###"
        Infix(
            Infix {
                span: 1:0 -> 1:0,
                ty_info: None,
                left: Infix(
                    Infix {
                        span: 1:0 -> 1:0,
                        ty_info: None,
                        left: Integer(
                            Integer {
                                span: 1:0 -> 1:0,
                                ty_info: None,
                                value: 3,
                            },
                        ),
                        operation: Plus(
                            1:0 -> 1:0,
                        ),
                        right: Integer(
                            Integer {
                                span: 1:0 -> 1:0,
                                ty_info: None,
                                value: 4,
                            },
                        ),
                    },
                ),
                operation: Plus(
                    1:0 -> 1:0,
                ),
                right: Integer(
                    Integer {
                        span: 1:0 -> 1:0,
                        ty_info: None,
                        value: 10,
                    },
                ),
            },
        )
        "###);
    }

    #[test]
    fn if_statement() {
        let expression = run(&[
            Token::t_if(),
            Token::integer("1"),
            Token::left_brace(),
            Token::ident("someident"),
            Token::right_brace(),
        ])
        .unwrap();

        insta::assert_debug_snapshot!(expression, @r###"
        If(
            If {
                span: 1:0 -> 1:0,
                ty_info: None,
                condition: Integer(
                    Integer {
                        span: 1:0 -> 1:0,
                        ty_info: None,
                        value: 1,
                    },
                ),
                success: Block {
                    span: 1:0 -> 1:0,
                    ty_info: None,
                    statements: [
                        Expression(
                            ExpressionStatement {
                                span: 1:0 -> 1:0,
                                ty_info: None,
                                expression: Ident(
                                    Ident {
                                        span: 1:0 -> 1:0,
                                        ty_info: None,
                                        name: SymbolU32 {
                                            value: 1,
                                        },
                                    },
                                ),
                                implicit_return: true,
                            },
                        ),
                    ],
                },
                otherwise: None,
            },
        )
        "###);
    }

    #[test]
    fn integer() {
        let expression = run(&[Token::integer("1")]).unwrap();
        insta::assert_debug_snapshot!(expression, @r###"
        Integer(
            Integer {
                span: 1:0 -> 1:0,
                ty_info: None,
                value: 1,
            },
        )
        "###);
    }

    #[test]
    fn ident() {
        let expression = run(&[Token::ident("someident")]).unwrap();
        insta::assert_debug_snapshot!(expression, @r###"
        Ident(
            Ident {
                span: 1:0 -> 1:0,
                ty_info: None,
                name: SymbolU32 {
                    value: 1,
                },
            },
        )
        "###);
    }

    #[test]
    fn equality() {
        let expression = run(&[Token::integer("1"), Token::eq(), Token::integer("1")]).unwrap();

        insta::assert_debug_snapshot!(expression, @r###"
        Infix(
            Infix {
                span: 1:0 -> 1:0,
                ty_info: None,
                left: Integer(
                    Integer {
                        span: 1:0 -> 1:0,
                        ty_info: None,
                        value: 1,
                    },
                ),
                operation: Eq(
                    1:0 -> 1:0,
                ),
                right: Integer(
                    Integer {
                        span: 1:0 -> 1:0,
                        ty_info: None,
                        value: 1,
                    },
                ),
            },
        )
        "###);
    }

    #[test]
    fn complex_equality() {
        let expression = run(&[
            Token::integer("1"),
            Token::eq(),
            Token::integer("1"),
            Token::plus(),
            Token::integer("2"),
        ])
        .unwrap();

        insta::assert_debug_snapshot!(expression, @r###"
        Infix(
            Infix {
                span: 1:0 -> 1:0,
                ty_info: None,
                left: Integer(
                    Integer {
                        span: 1:0 -> 1:0,
                        ty_info: None,
                        value: 1,
                    },
                ),
                operation: Eq(
                    1:0 -> 1:0,
                ),
                right: Infix(
                    Infix {
                        span: 1:0 -> 1:0,
                        ty_info: None,
                        left: Integer(
                            Integer {
                                span: 1:0 -> 1:0,
                                ty_info: None,
                                value: 1,
                            },
                        ),
                        operation: Plus(
                            1:0 -> 1:0,
                        ),
                        right: Integer(
                            Integer {
                                span: 1:0 -> 1:0,
                                ty_info: None,
                                value: 2,
                            },
                        ),
                    },
                ),
            },
        )
        "###);
    }

    #[test]
    fn function_call_no_param() {
        let expression = run(&[
            Token::ident("func"),
            Token::left_paren(),
            Token::right_paren(),
        ])
        .unwrap();

        insta::assert_debug_snapshot!(expression, @r###"
        Call(
            Call {
                span: 1:0 -> 1:0,
                ty_info: None,
                name: SymbolU32 {
                    value: 1,
                },
                args: [],
            },
        )
        "###);
    }

    #[test]
    fn function_call_one_param_no_comma() {
        let expression = run(&[
            Token::ident("func"),
            Token::left_paren(),
            Token::integer("1"),
            Token::right_paren(),
        ])
        .unwrap();

        insta::assert_debug_snapshot!(expression, @r###"
        Call(
            Call {
                span: 1:0 -> 1:0,
                ty_info: None,
                name: SymbolU32 {
                    value: 1,
                },
                args: [
                    Integer(
                        Integer {
                            span: 1:0 -> 1:0,
                            ty_info: None,
                            value: 1,
                        },
                    ),
                ],
            },
        )
        "###);
    }

    #[test]
    fn function_call_one_param_trailing_comma() {
        let expression = run(&[
            Token::ident("func"),
            Token::left_paren(),
            Token::integer("1"),
            Token::comma(),
            Token::right_paren(),
        ])
        .unwrap();

        insta::assert_debug_snapshot!(expression, @r###"
        Call(
            Call {
                span: 1:0 -> 1:0,
                ty_info: None,
                name: SymbolU32 {
                    value: 1,
                },
                args: [
                    Integer(
                        Integer {
                            span: 1:0 -> 1:0,
                            ty_info: None,
                            value: 1,
                        },
                    ),
                ],
            },
        )
        "###);
    }

    #[test]
    fn function_call_multi_param_no_comma() {
        let expression = run(&[
            Token::ident("func"),
            Token::left_paren(),
            Token::integer("1"),
            Token::comma(),
            Token::integer("2"),
            Token::comma(),
            Token::integer("3"),
            Token::right_paren(),
        ])
        .unwrap();

        insta::assert_debug_snapshot!(expression, @r###"
        Call(
            Call {
                span: 1:0 -> 1:0,
                ty_info: None,
                name: SymbolU32 {
                    value: 1,
                },
                args: [
                    Integer(
                        Integer {
                            span: 1:0 -> 1:0,
                            ty_info: None,
                            value: 1,
                        },
                    ),
                    Integer(
                        Integer {
                            span: 1:0 -> 1:0,
                            ty_info: None,
                            value: 2,
                        },
                    ),
                    Integer(
                        Integer {
                            span: 1:0 -> 1:0,
                            ty_info: None,
                            value: 3,
                        },
                    ),
                ],
            },
        )
        "###);
    }

    #[test]
    fn function_call_multi_param_trailing_comma() {
        let expression = run(&[
            Token::ident("func"),
            Token::left_paren(),
            Token::integer("1"),
            Token::comma(),
            Token::integer("2"),
            Token::comma(),
            Token::integer("3"),
            Token::comma(),
            Token::right_paren(),
        ])
        .unwrap();

        insta::assert_debug_snapshot!(expression, @r###"
        Call(
            Call {
                span: 1:0 -> 1:0,
                ty_info: None,
                name: SymbolU32 {
                    value: 1,
                },
                args: [
                    Integer(
                        Integer {
                            span: 1:0 -> 1:0,
                            ty_info: None,
                            value: 1,
                        },
                    ),
                    Integer(
                        Integer {
                            span: 1:0 -> 1:0,
                            ty_info: None,
                            value: 2,
                        },
                    ),
                    Integer(
                        Integer {
                            span: 1:0 -> 1:0,
                            ty_info: None,
                            value: 3,
                        },
                    ),
                ],
            },
        )
        "###);
    }

    #[test]
    fn function_call_with_expression_param() {
        let expression = run(&[
            Token::ident("func"),
            Token::left_paren(),
            Token::integer("1"),
            Token::plus(),
            Token::integer("2"),
            Token::comma(),
            Token::integer("3"),
            Token::comma(),
            Token::right_paren(),
        ])
        .unwrap();

        insta::assert_debug_snapshot!(expression, @r###"
        Call(
            Call {
                span: 1:0 -> 1:0,
                ty_info: None,
                name: SymbolU32 {
                    value: 1,
                },
                args: [
                    Infix(
                        Infix {
                            span: 1:0 -> 1:0,
                            ty_info: None,
                            left: Integer(
                                Integer {
                                    span: 1:0 -> 1:0,
                                    ty_info: None,
                                    value: 1,
                                },
                            ),
                            operation: Plus(
                                1:0 -> 1:0,
                            ),
                            right: Integer(
                                Integer {
                                    span: 1:0 -> 1:0,
                                    ty_info: None,
                                    value: 2,
                                },
                            ),
                        },
                    ),
                    Integer(
                        Integer {
                            span: 1:0 -> 1:0,
                            ty_info: None,
                            value: 3,
                        },
                    ),
                ],
            },
        )
        "###);
    }

    #[test]
    fn function_call_in_expression() {
        let expression = run(&[
            Token::ident("func"),
            Token::left_paren(),
            Token::integer("1"),
            Token::right_paren(),
            Token::plus(),
            Token::integer("2"),
        ])
        .unwrap();

        insta::assert_debug_snapshot!(expression, @r###"
        Infix(
            Infix {
                span: 1:0 -> 1:0,
                ty_info: None,
                left: Call(
                    Call {
                        span: 1:0 -> 1:0,
                        ty_info: None,
                        name: SymbolU32 {
                            value: 1,
                        },
                        args: [
                            Integer(
                                Integer {
                                    span: 1:0 -> 1:0,
                                    ty_info: None,
                                    value: 1,
                                },
                            ),
                        ],
                    },
                ),
                operation: Plus(
                    1:0 -> 1:0,
                ),
                right: Integer(
                    Integer {
                        span: 1:0 -> 1:0,
                        ty_info: None,
                        value: 2,
                    },
                ),
            },
        )
        "###);
    }
}
