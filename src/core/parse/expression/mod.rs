use crate::{
    core::{ast, lexer::token::Token},
    util::source::Spanned,
};

use super::{block::parse_block, ParseCtx, ParseError};

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
}

impl Precedence {
    pub fn of(token: &Token) -> Self {
        match token {
            Token::Plus(_) => Precedence::Sum,
            Token::Eq(_) | Token::NotEq(_) => Precedence::Equality,
            _ => Precedence::Lowest,
        }
    }
}

fn parse_prefix(ctx: &mut ParseCtx) -> Result<ast::Expression, ParseError> {
    match ctx.lexer.peek_token() {
        Token::Integer(_) => Ok(ast::Expression::Integer(parse_integer(ctx)?)),
        Token::Ident(_) => Ok(ast::Expression::Ident(parse_ident(ctx)?)),
        Token::True(_) => Ok(ast::Expression::Boolean(parse_boolean(ctx)?)),
        Token::False(_) => Ok(ast::Expression::Boolean(parse_boolean(ctx)?)),
        Token::LeftBrace(_) => Ok(ast::Expression::Block(parse_block(ctx)?)),
        Token::If(_) => Ok(ast::Expression::If(parse_if(ctx)?)),
        token => Err(ParseError::UnexpectedToken(token)),
    }
}

pub fn parse_expression(
    ctx: &mut ParseCtx,
    precedence: Precedence,
) -> Result<ast::Expression, ParseError> {
    let mut left = parse_prefix(ctx)?;

    while !matches!(ctx.lexer.peek_token(), Token::EOF(_))
        && precedence < Precedence::of(&ctx.lexer.peek_token())
    {
        if let Ok(operation) = ast::InfixOperation::try_from(ctx.lexer.peek_token()) {
            let token = ctx.lexer.next_token();
            let precedence = Precedence::of(&token);

            let right = parse_expression(ctx, precedence)?;

            left = ast::Expression::Infix(ast::Infix {
                span: token.span().to(&right),
                left: Box::new(left),
                operation,
                right: Box::new(right),
            });
        } else {
            // Probably aren't in the expression any more
            break;
        }
    }

    Ok(left)
}

#[cfg(test)]
mod test {
    use crate::core::lexer::Lexer;

    use self::ast::Expression;

    use super::*;

    fn run(tokens: Vec<Token>) -> Result<Expression, ParseError> {
        let lexer = Lexer::with_tokens(tokens);
        parse_expression(&mut ParseCtx::new(lexer), Precedence::Lowest)
    }

    #[test]
    fn simple_addition() {
        let expression = run(vec![
            Token::integer("3"),
            Token::plus(),
            Token::integer("4"),
        ])
        .unwrap();

        insta::assert_debug_snapshot!(expression, @r###"
        Infix(
            Infix {
                span: 1:0 -> 1:0,
                left: Integer(
                    Integer {
                        span: 1:0 -> 1:0,
                        value: 3,
                    },
                ),
                operation: Plus(
                    1:0 -> 1:0,
                ),
                right: Integer(
                    Integer {
                        span: 1:0 -> 1:0,
                        value: 4,
                    },
                ),
            },
        )
        "###);
    }

    #[test]
    fn multi_addition() {
        let expression = run(vec![
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
                left: Infix(
                    Infix {
                        span: 1:0 -> 1:0,
                        left: Integer(
                            Integer {
                                span: 1:0 -> 1:0,
                                value: 3,
                            },
                        ),
                        operation: Plus(
                            1:0 -> 1:0,
                        ),
                        right: Integer(
                            Integer {
                                span: 1:0 -> 1:0,
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
                        value: 10,
                    },
                ),
            },
        )
        "###);
    }

    #[test]
    fn if_statement() {
        let expression = run(vec![
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
                condition: Integer(
                    Integer {
                        span: 1:0 -> 1:0,
                        value: 1,
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
        let expression = run(vec![Token::integer("1")]).unwrap();
        insta::assert_debug_snapshot!(expression, @r###"
        Integer(
            Integer {
                span: 1:0 -> 1:0,
                value: 1,
            },
        )
        "###);
    }

    #[test]
    fn ident() {
        let expression = run(vec![Token::ident("someident")]).unwrap();
        insta::assert_debug_snapshot!(expression, @r###"
        Ident(
            Ident {
                span: 1:0 -> 1:0,
                name: SymbolU32 {
                    value: 1,
                },
            },
        )
        "###);
    }

    #[test]
    fn equality() {
        let expression = run(vec![Token::integer("1"), Token::eq(), Token::integer("1")]).unwrap();

        insta::assert_debug_snapshot!(expression, @r###"
        Infix(
            Infix {
                span: 1:0 -> 1:0,
                left: Integer(
                    Integer {
                        span: 1:0 -> 1:0,
                        value: 1,
                    },
                ),
                operation: Eq(
                    1:0 -> 1:0,
                ),
                right: Integer(
                    Integer {
                        span: 1:0 -> 1:0,
                        value: 1,
                    },
                ),
            },
        )
        "###);
    }

    #[test]
    fn complex_equality() {
        let expression = run(vec![
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
                left: Integer(
                    Integer {
                        span: 1:0 -> 1:0,
                        value: 1,
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
                            },
                        ),
                        operation: Plus(
                            1:0 -> 1:0,
                        ),
                        right: Integer(
                            Integer {
                                span: 1:0 -> 1:0,
                                value: 2,
                            },
                        ),
                    },
                ),
            },
        )
        "###);
    }
}
