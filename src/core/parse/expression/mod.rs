use crate::{
    core::{
        ast,
        lexer::{token::Token, Lexer},
        symbol::SymbolMap,
    },
    util::source::Spanned,
};

use super::{block::parse_block, ParseError};

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
    Sum,
}

impl Precedence {
    pub fn of(token: &Token) -> Self {
        match token {
            Token::Plus(_) => Precedence::Sum,
            _ => Precedence::Lowest,
        }
    }
}

fn parse_prefix(lexer: &mut Lexer, symbols: &mut SymbolMap) -> Result<ast::Expression, ParseError> {
    match lexer.peek_token() {
        Token::Integer(_) => Ok(ast::Expression::Integer(parse_integer(lexer)?)),
        Token::Ident(_) => Ok(ast::Expression::Ident(parse_ident(lexer, symbols)?)),
        Token::True(_) => Ok(ast::Expression::Boolean(parse_boolean(lexer)?)),
        Token::False(_) => Ok(ast::Expression::Boolean(parse_boolean(lexer)?)),
        Token::LeftBrace(_) => Ok(ast::Expression::Block(parse_block(lexer, symbols)?)),
        Token::If(_) => Ok(ast::Expression::If(parse_if(lexer, symbols)?)),
        token => Err(ParseError::UnexpectedToken(token)),
    }
}

pub fn parse_expression(
    lexer: &mut Lexer,
    precedence: Precedence,
    symbols: &mut SymbolMap,
) -> Result<ast::Expression, ParseError> {
    let mut left = parse_prefix(lexer, symbols)?;

    while !matches!(lexer.peek_token(), Token::EOF(_))
        && precedence < Precedence::of(&lexer.peek_token())
    {
        if let Ok(operation) = ast::InfixOperation::try_from(lexer.peek_token()) {
            let token = lexer.next_token();
            let precedence = Precedence::of(&token);

            let right = parse_expression(lexer, precedence, symbols)?;

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
    use self::ast::Expression;

    use super::*;

    fn run(tokens: Vec<Token>) -> Result<Expression, ParseError> {
        let mut lexer = Lexer::with_tokens(tokens);
        parse_expression(&mut lexer, Precedence::Lowest, &mut SymbolMap::new())
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
                                        name: Symbol(
                                            0,
                                        ),
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
                name: Symbol(
                    0,
                ),
            },
        )
        "###);
    }
}
