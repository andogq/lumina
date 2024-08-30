use std::iter;

use e_assign::{parse_assign, parse_op_assign};
use e_loop::parse_loop;

use super::*;

use self::{
    e_array::parse_array, e_boolean::parse_boolean, e_ident::parse_ident, e_if::parse_if,
    e_integer::parse_integer,
};

mod e_array;
mod e_assign;
mod e_boolean;
mod e_ident;
mod e_if;
mod e_integer;
mod e_loop;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    Lowest,
    Assign,
    Binary,
    Equality,
    Sum,
    Multiply,
    Call,
}

impl Precedence {
    pub fn of(token: &Token) -> Self {
        match token {
            Token::Minus | Token::Plus => Precedence::Sum,
            Token::Asterix | Token::ForwardSlash => Precedence::Multiply,
            Token::And | Token::Or => Precedence::Binary,
            Token::DoubleEq
            | Token::NotEq
            | Token::LeftAngle
            | Token::RightAngle
            | Token::LeftAngleEq
            | Token::RightAngleEq => Precedence::Equality,
            Token::LeftParen | Token::LeftSquare => Precedence::Call,
            Token::Eq
            | Token::AddAssign
            | Token::MinusAssign
            | Token::DivAssign
            | Token::MulAssign => Precedence::Assign,
            _ => Precedence::Lowest,
        }
    }
}

impl From<Token> for Precedence {
    fn from(token: Token) -> Self {
        Self::of(&token)
    }
}

fn parse_prefix(compiler: &mut Compiler, lexer: &mut Lexer<'_>) -> Result<Expression, ParseError> {
    match lexer.peek_token().unwrap().clone() {
        Token::Integer(_) => Ok(Expression::Integer(parse_integer(compiler, lexer)?)),
        Token::Ident(_) => match lexer.double_peek_token() {
            Some(Token::Eq) => Ok(Expression::Assign(parse_assign(compiler, lexer)?)),
            Some(Token::AddAssign | Token::MinusAssign) => {
                Ok(Expression::Assign(parse_op_assign(compiler, lexer)?))
            }
            _ => Ok(Expression::Ident(parse_ident(compiler, lexer)?)),
        },
        Token::True => Ok(Expression::Boolean(parse_boolean(compiler, lexer)?)),
        Token::False => Ok(Expression::Boolean(parse_boolean(compiler, lexer)?)),
        Token::LeftBrace => Ok(Expression::Block(parse_block(compiler, lexer)?)),
        Token::LeftParen => parse_grouped(compiler, lexer),
        Token::LeftSquare => Ok(Expression::Array(parse_array(compiler, lexer)?)),
        Token::If => Ok(Expression::If(parse_if(compiler, lexer)?)),
        Token::Loop => Ok(Expression::Loop(parse_loop(compiler, lexer)?)),
        token => Err(ParseError::UnexpectedToken(token.clone())),
    }
}

pub fn parse_expression(
    compiler: &mut Compiler,
    lexer: &mut Lexer<'_>,
    precedence: Precedence,
) -> Result<Expression, ParseError> {
    let mut left = parse_prefix(compiler, lexer)?;

    while lexer.peek_token().is_some() && precedence < Precedence::of(lexer.peek_token().unwrap()) {
        left = match (left, lexer.peek_token().unwrap()) {
            // Function call
            (Expression::Ident(ident), Token::LeftParen) => {
                parse_function_call(compiler, lexer, ident)?
            }

            // Index operation
            (Expression::Ident(ident), Token::LeftSquare) => parse_index(compiler, lexer, ident)?,

            // Regular infix operation
            (left, _) => parse_infix(compiler, lexer, left)?,
        };
    }

    Ok(left)
}

fn parse_index(
    compiler: &mut Compiler,
    lexer: &mut Lexer,
    ident: Ident,
) -> Result<Expression, ParseError> {
    // Parse out the left square bracket
    match lexer.next_token().unwrap() {
        Token::LeftSquare => (),
        token => {
            return Err(ParseError::ExpectedToken {
                expected: Box::new(Token::LeftSquare),
                found: Box::new(token),
                reason: "index must be performed with square bracket".to_string(),
            });
        }
    }

    // Pull out the index
    let index = parse_expression(compiler, lexer, Precedence::Lowest)?;

    // Pull out closing bracket
    let end_span = match lexer.next_spanned().unwrap() {
        (Token::RightSquare, span) => span.end,
        (token, _) => {
            return Err(ParseError::ExpectedToken {
                expected: Box::new(Token::RightSquare),
                found: Box::new(token),
                reason: "index must be performed with square bracket".to_string(),
            });
        }
    };

    Ok(Expression::Index(Index {
        span: ident.span.start..end_span,
        value: ident.binding,
        index: Box::new(index),
        ty_info: None,
    }))
}

fn parse_function_call(
    compiler: &mut Compiler,
    lexer: &mut Lexer,
    ident: Ident,
) -> Result<Expression, ParseError> {
    // Consume the args
    let args = iter::from_fn(|| {
        match lexer.peek_token().unwrap() {
            Token::RightParen => None,
            Token::LeftParen | Token::Comma => {
                // Consume the opening paren or comma
                lexer.next_token();

                // If the closing parenthesis is encountered, stop parsing arguments
                if matches!(lexer.peek_token().unwrap(), Token::RightParen) {
                    return None;
                }

                // Parse the next argument
                Some(parse_expression(compiler, lexer, Precedence::Lowest))
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
    let end_span = match lexer.next_spanned().unwrap() {
        (Token::RightParen, span) => span,
        (token, _) => {
            return Err(ParseError::ExpectedToken {
                expected: Box::new(Token::RightParen),
                found: Box::new(token),
                reason: "argument list must end with right paren".to_string(),
            })
        }
    };

    let span = ident.span.start..end_span.end;
    Ok(Expression::Call(Call::new(
        ident.binding,
        args,
        span,
        Default::default(),
    )))
}

fn parse_infix(
    compiler: &mut Compiler,
    lexer: &mut Lexer,
    left: Expression,
) -> Result<Expression, ParseError> {
    let token = lexer.next_token().unwrap();

    Ok(
        if let Ok(operation) = InfixOperation::try_from(token.clone()) {
            let precedence = Precedence::of(&token);

            let right = parse_expression(compiler, lexer, precedence)?;

            let span = left.span().start..right.span().end;

            Expression::Infix(Infix::new(
                Box::new(left),
                operation,
                Box::new(right),
                span,
                Default::default(),
            ))
        } else {
            left
        },
    )
}

fn parse_grouped(compiler: &mut Compiler, lexer: &mut Lexer) -> Result<Expression, ParseError> {
    let span_start = match lexer.next_spanned().unwrap() {
        (Token::LeftParen, span) => span.start,
        (token, _) => {
            return Err(ParseError::ExpectedToken {
                expected: Box::new(Token::LeftBrace),
                found: Box::new(token),
                reason: "open paren for grouped expression".to_string(),
            });
        }
    };

    let e = parse_expression(compiler, lexer, Precedence::Lowest)?;

    let span_end = match lexer.next_spanned().unwrap() {
        (Token::RightParen, span) => span.end,
        (token, _) => {
            return Err(ParseError::ExpectedToken {
                expected: Box::new(Token::RightBrace),
                found: Box::new(token),
                reason: "close paren must end grouped expression".to_string(),
            });
        }
    };

    // TODO: Need to somehow attach this span to the expression
    #[allow(unused_variables)]
    let e_span = span_start..span_end;

    Ok(e)
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
                left: Integer(
                    Integer {
                        value: 3,
                        span: 0..1,
                        ty_info: None,
                    },
                ),
                operation: Plus,
                right: Integer(
                    Integer {
                        value: 4,
                        span: 4..5,
                        ty_info: None,
                    },
                ),
                span: 0..5,
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
                left: Infix(
                    Infix {
                        left: Integer(
                            Integer {
                                value: 3,
                                span: 0..1,
                                ty_info: None,
                            },
                        ),
                        operation: Plus,
                        right: Integer(
                            Integer {
                                value: 4,
                                span: 4..5,
                                ty_info: None,
                            },
                        ),
                        span: 0..5,
                        ty_info: None,
                    },
                ),
                operation: Plus,
                right: Integer(
                    Integer {
                        value: 10,
                        span: 8..10,
                        ty_info: None,
                    },
                ),
                span: 0..10,
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
                condition: Integer(
                    Integer {
                        value: 1,
                        span: 3..4,
                        ty_info: None,
                    },
                ),
                success: Block {
                    statements: [
                        ExpressionStatement(
                            ExpressionStatement {
                                expression: Ident(
                                    Ident {
                                        binding: SymbolU32 {
                                            value: 1,
                                        },
                                        span: 7..16,
                                        ty_info: None,
                                    },
                                ),
                                implicit_return: true,
                                span: 7..16,
                                ty_info: None,
                            },
                        ),
                    ],
                    span: 5..18,
                    ty_info: None,
                },
                otherwise: None,
                span: 0..18,
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
                value: 1,
                span: 0..1,
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
                binding: SymbolU32 {
                    value: 1,
                },
                span: 0..9,
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
                left: Integer(
                    Integer {
                        value: 1,
                        span: 0..1,
                        ty_info: None,
                    },
                ),
                operation: Eq,
                right: Integer(
                    Integer {
                        value: 1,
                        span: 5..6,
                        ty_info: None,
                    },
                ),
                span: 0..6,
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
                left: Integer(
                    Integer {
                        value: 1,
                        span: 0..1,
                        ty_info: None,
                    },
                ),
                operation: Eq,
                right: Infix(
                    Infix {
                        left: Integer(
                            Integer {
                                value: 1,
                                span: 5..6,
                                ty_info: None,
                            },
                        ),
                        operation: Plus,
                        right: Integer(
                            Integer {
                                value: 2,
                                span: 9..10,
                                ty_info: None,
                            },
                        ),
                        span: 5..10,
                        ty_info: None,
                    },
                ),
                span: 0..10,
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
                name: SymbolU32 {
                    value: 1,
                },
                args: [],
                span: 0..6,
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
                name: SymbolU32 {
                    value: 1,
                },
                args: [
                    Integer(
                        Integer {
                            value: 1,
                            span: 5..6,
                            ty_info: None,
                        },
                    ),
                ],
                span: 0..7,
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
                name: SymbolU32 {
                    value: 1,
                },
                args: [
                    Integer(
                        Integer {
                            value: 1,
                            span: 5..6,
                            ty_info: None,
                        },
                    ),
                ],
                span: 0..8,
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
                name: SymbolU32 {
                    value: 1,
                },
                args: [
                    Integer(
                        Integer {
                            value: 1,
                            span: 5..6,
                            ty_info: None,
                        },
                    ),
                    Integer(
                        Integer {
                            value: 2,
                            span: 8..9,
                            ty_info: None,
                        },
                    ),
                    Integer(
                        Integer {
                            value: 3,
                            span: 11..12,
                            ty_info: None,
                        },
                    ),
                ],
                span: 0..13,
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
                name: SymbolU32 {
                    value: 1,
                },
                args: [
                    Integer(
                        Integer {
                            value: 1,
                            span: 5..6,
                            ty_info: None,
                        },
                    ),
                    Integer(
                        Integer {
                            value: 2,
                            span: 8..9,
                            ty_info: None,
                        },
                    ),
                    Integer(
                        Integer {
                            value: 3,
                            span: 11..12,
                            ty_info: None,
                        },
                    ),
                ],
                span: 0..14,
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
                name: SymbolU32 {
                    value: 1,
                },
                args: [
                    Infix(
                        Infix {
                            left: Integer(
                                Integer {
                                    value: 1,
                                    span: 5..6,
                                    ty_info: None,
                                },
                            ),
                            operation: Plus,
                            right: Integer(
                                Integer {
                                    value: 2,
                                    span: 9..10,
                                    ty_info: None,
                                },
                            ),
                            span: 5..10,
                            ty_info: None,
                        },
                    ),
                    Integer(
                        Integer {
                            value: 3,
                            span: 12..13,
                            ty_info: None,
                        },
                    ),
                ],
                span: 0..15,
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
                left: Call(
                    Call {
                        name: SymbolU32 {
                            value: 1,
                        },
                        args: [
                            Integer(
                                Integer {
                                    value: 1,
                                    span: 5..6,
                                    ty_info: None,
                                },
                            ),
                        ],
                        span: 0..7,
                        ty_info: None,
                    },
                ),
                operation: Plus,
                right: Integer(
                    Integer {
                        value: 2,
                        span: 10..11,
                        ty_info: None,
                    },
                ),
                span: 0..11,
                ty_info: None,
            },
        )
        "###);
    }

    #[rstest]
    fn grouped_expression(mut mock_compiler: Compiler) {
        let expression = parse_expression(
            &mut mock_compiler,
            &mut "1 * (2 + 3) * 4".into(),
            Precedence::Lowest,
        )
        .unwrap();

        insta::assert_debug_snapshot!(expression, @r###"
        Infix(
            Infix {
                left: Infix(
                    Infix {
                        left: Integer(
                            Integer {
                                value: 1,
                                span: 0..1,
                                ty_info: None,
                            },
                        ),
                        operation: Multiply,
                        right: Infix(
                            Infix {
                                left: Integer(
                                    Integer {
                                        value: 2,
                                        span: 5..6,
                                        ty_info: None,
                                    },
                                ),
                                operation: Plus,
                                right: Integer(
                                    Integer {
                                        value: 3,
                                        span: 9..10,
                                        ty_info: None,
                                    },
                                ),
                                span: 5..10,
                                ty_info: None,
                            },
                        ),
                        span: 0..10,
                        ty_info: None,
                    },
                ),
                operation: Multiply,
                right: Integer(
                    Integer {
                        value: 4,
                        span: 14..15,
                        ty_info: None,
                    },
                ),
                span: 0..15,
                ty_info: None,
            },
        )
        "###);
    }
}
