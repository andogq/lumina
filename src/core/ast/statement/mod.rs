mod s_let;
mod s_return;

use std::fmt::Display;

pub use s_let::*;
pub use s_return::*;

use crate::core::{
    ast::Expression,
    lexer::{Lexer, Token},
};

use super::ParseNode;

#[derive(Clone, Debug, PartialEq)]
pub enum Statement {
    Let(LetStatement),
    Return(ReturnStatement),
    Expression {
        /// The expression
        expression: Expression,

        /// Whether the expression is terminated with a semicolon
        semicolon: bool,
    },
}

impl<S> ParseNode<S> for Statement
where
    S: Iterator<Item = char>,
{
    fn parse(lexer: &mut Lexer<S>) -> Result<Self, String> {
        match lexer.peek() {
            Token::Let(_) => Ok(Statement::Let(LetStatement::parse(lexer)?)),
            Token::Return(_) => Ok(Statement::Return(ReturnStatement::parse(lexer)?)),
            _ => Ok(Statement::Expression {
                expression: Expression::parse(lexer)?,
                semicolon: lexer
                    .next_if(|token| matches!(token, Token::Semicolon(_)))
                    .is_some(),
            }),
        }
    }
}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::Let(let_statement) => let_statement.fmt(f),
            Statement::Return(return_statement) => return_statement.fmt(f),
            Statement::Expression {
                expression,
                semicolon,
            } => {
                expression.fmt(f)?;

                if *semicolon {
                    write!(f, ";")?;
                }

                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        assert_pattern,
        core::ast::{Identifier, InfixExpression},
        test_parser,
    };

    use super::*;

    #[test]
    fn let_statement() {
        test_parser!(
            Statement,
            "let x = 1;",
            Statement::Let(LetStatement {
                name,
                value: Expression::Integer(i),
                ..
            }),
            {
                assert_eq!(name.value, "x");
                assert_eq!(i.value, 1);
            }
        );
    }

    #[test]
    fn return_statement() {
        test_parser!(
            Statement,
            "return y;",
            Statement::Return(ReturnStatement {
                value: Expression::Identifier(Identifier { value, .. }),
                ..
            }),
            {
                assert_eq!(value, "y");
            }
        );
    }

    #[test]
    fn expression_statement_implicit() {
        test_parser!(
            Statement,
            "a + b",
            Statement::Expression {
                expression: Expression::Infix(InfixExpression {
                    operator,
                    left,
                    right,
                    ..
                }),
                semicolon: false
            },
            {
                assert_eq!(operator, "+");

                assert_pattern!(*left, Expression::Identifier(Identifier { value, .. }), {
                    assert_eq!(value, "a");
                });

                assert_pattern!(*right, Expression::Identifier(Identifier { value, .. }), {
                    assert_eq!(value, "b");
                });
            }
        );
    }

    #[test]
    fn expression_statement_explicit() {
        test_parser!(
            Statement,
            "a + b;",
            Statement::Expression {
                expression: Expression::Infix(InfixExpression {
                    operator,
                    left,
                    right,
                    ..
                }),
                semicolon: true
            },
            {
                assert_eq!(operator, "+");

                assert_pattern!(*left, Expression::Identifier(Identifier { value, .. }), {
                    assert_eq!(value, "a");
                });

                assert_pattern!(*right, Expression::Identifier(Identifier { value, .. }), {
                    assert_eq!(value, "b");
                });
            }
        );
    }
}
