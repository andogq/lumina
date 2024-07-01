use crate::{
    core::{
        ast,
        lexer::{token::Token, Lexer},
        symbol::SymbolMap,
    },
    util::source::Spanned,
};

use super::{block::parse_block, ParseError};

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
    let mut advance = true;

    let prefix = match lexer.peek_token() {
        Token::Integer(token) => Ok(ast::Expression::Integer(ast::Integer {
            span: token.span,
            value: token
                .literal
                .parse()
                .map_err(|_| ParseError::InvalidLiteral {
                    expected: "integer".to_string(),
                })?,
        })),
        Token::Ident(token) => Ok(ast::Expression::Ident(ast::Ident {
            span: token.span,
            name: symbols.get(token.literal),
        })),
        Token::True(token) => Ok(ast::Expression::Boolean(ast::Boolean {
            span: token.span,
            value: true,
        })),
        Token::False(token) => Ok(ast::Expression::Boolean(ast::Boolean {
            span: token.span,
            value: false,
        })),
        Token::LeftBrace(_) => {
            advance = false;

            Ok(ast::Expression::Block(parse_block(lexer, symbols)?))
        }
        Token::If(token) => {
            advance = false;
            lexer.next_token();

            let mut span = token.span;

            let condition = parse_expression(lexer, Precedence::Lowest, symbols)?;

            let success = parse_block(lexer, symbols)?;
            span = span.to(&success);

            let otherwise = if matches!(lexer.peek_token(), Token::Else(_)) {
                lexer.next_token();

                let otherwise = parse_block(lexer, symbols)?;
                span = span.to(&otherwise);

                Some(otherwise)
            } else {
                None
            };

            Ok(ast::Expression::If(ast::If {
                condition: Box::new(condition),
                success,
                otherwise,
                span,
            }))
        }
        token => Err(ParseError::UnexpectedToken(token)),
    };

    if advance {
        lexer.next_token();
    }

    prefix
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
    use super::*;

    #[test]
    fn simple_addition() {
        let mut lexer = Lexer::with_tokens(vec![
            Token::integer("3"),
            Token::plus(),
            Token::integer("4"),
        ]);
        let expression =
            parse_expression(&mut lexer, Precedence::Lowest, &mut SymbolMap::new()).unwrap();

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
        let mut lexer = Lexer::with_tokens(vec![
            Token::integer("3"),
            Token::plus(),
            Token::integer("4"),
            Token::plus(),
            Token::integer("10"),
        ]);
        let expression =
            parse_expression(&mut lexer, Precedence::Lowest, &mut SymbolMap::new()).unwrap();

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
}
