use crate::core::{
    ast::node,
    lexer::{token::Token, Lexer},
};

use super::ParseError;

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

fn parse_prefix<S>(lexer: &mut Lexer<S>) -> Result<node::Expression, ParseError>
where
    S: Iterator<Item = char>,
{
    match lexer.next() {
        Token::Integer(token) => Ok(node::Expression::Integer(node::Integer {
            span: token.span,
            literal: token
                .literal
                .parse()
                .map_err(|_| ParseError::InvalidLiteral {
                    expected: "integer".to_string(),
                })?,
        })),
        Token::Ident(token) => Ok(node::Expression::Ident(node::Ident {
            span: token.span,
            name: token.literal,
        })),
        token => Err(ParseError::UnexpectedToken(token)),
    }
}

pub fn parse_expression<S>(
    lexer: &mut Lexer<S>,
    precedence: Precedence,
) -> Result<node::Expression, ParseError>
where
    S: Iterator<Item = char>,
{
    let mut left = parse_prefix(lexer)?;

    while !matches!(lexer.peek(), Token::EOF(_)) && precedence < Precedence::of(&lexer.peek()) {
        if let Ok(operation) = node::InfixOperation::try_from(lexer.peek()) {
            let token = lexer.next();
            let precedence = Precedence::of(&token);

            left = node::Expression::Infix(node::Infix {
                left: Box::new(left),
                operation,
                right: Box::new(parse_expression(lexer, precedence)?),
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
    use crate::util::source::Source;

    use super::*;

    #[test]
    fn simple_addition() {
        let mut lexer = Lexer::new(Source::new("test", "3 + 4".chars()));
        let expression = parse_expression(&mut lexer, Precedence::Lowest);

        assert!(matches!(expression, Ok(node::Expression::Infix(_))));
        if let Ok(node::Expression::Infix(node::Infix {
            left,
            operation: node::InfixOperation::Plus(_),
            right,
        })) = expression
        {
            assert!(matches!(
                *left,
                node::Expression::Integer(node::Integer { literal: 3, .. })
            ));
            assert!(matches!(
                *right,
                node::Expression::Integer(node::Integer { literal: 4, .. })
            ));
        }
    }

    #[test]
    fn multi_addition() {
        let mut lexer = Lexer::new(Source::new("test", "3 + 4 + 10".chars()));
        let expression = parse_expression(&mut lexer, Precedence::Lowest);

        assert!(matches!(expression, Ok(node::Expression::Infix(_))));
        if let Ok(node::Expression::Infix(node::Infix {
            left,
            operation: node::InfixOperation::Plus(_),
            right,
        })) = expression
        {
            assert!(matches!(*left, node::Expression::Infix(_)));
            if let node::Expression::Infix(node::Infix {
                left,
                operation: node::InfixOperation::Plus(_),
                right,
            }) = *left
            {
                assert!(matches!(
                    *left,
                    node::Expression::Integer(node::Integer { literal: 3, .. })
                ));
                assert!(matches!(
                    *right,
                    node::Expression::Integer(node::Integer { literal: 4, .. })
                ));
            }

            assert!(matches!(
                *right,
                node::Expression::Integer(node::Integer { literal: 10, .. })
            ));
        }
    }
}
