use crate::core::{
    ast::{source, symbol::SymbolMap},
    lexer::{token::Token, Lexer},
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

fn parse_prefix<S>(
    lexer: &mut Lexer<S>,
    symbols: &mut SymbolMap,
) -> Result<source::Expression, ParseError>
where
    S: Iterator<Item = char>,
{
    let mut advance = true;

    let prefix = match lexer.peek() {
        Token::Integer(token) => Ok(source::Expression::Integer(source::Integer {
            span: token.span,
            value: token
                .literal
                .parse()
                .map_err(|_| ParseError::InvalidLiteral {
                    expected: "integer".to_string(),
                })?,
        })),
        Token::Ident(token) => Ok(source::Expression::Ident(source::Ident {
            span: token.span,
            name: symbols.get(token.literal),
        })),
        Token::True(token) => Ok(source::Expression::Boolean(source::Boolean {
            span: token.span,
            value: true,
        })),
        Token::False(token) => Ok(source::Expression::Boolean(source::Boolean {
            span: token.span,
            value: false,
        })),
        Token::LeftBrace(_) => {
            advance = false;

            Ok(source::Expression::Block(parse_block(lexer, symbols)?))
        }
        token => Err(ParseError::UnexpectedToken(token)),
    };

    if advance {
        lexer.next();
    }

    prefix
}

pub fn parse_expression<S>(
    lexer: &mut Lexer<S>,
    precedence: Precedence,
    symbols: &mut SymbolMap,
) -> Result<source::Expression, ParseError>
where
    S: Iterator<Item = char>,
{
    let mut left = parse_prefix(lexer, symbols)?;

    while !matches!(lexer.peek(), Token::EOF(_)) && precedence < Precedence::of(&lexer.peek()) {
        if let Ok(operation) = source::InfixOperation::try_from(lexer.peek()) {
            let token = lexer.next();
            let precedence = Precedence::of(&token);

            left = source::Expression::Infix(source::Infix {
                left: Box::new(left),
                operation,
                right: Box::new(parse_expression(lexer, precedence, symbols)?),
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
        let expression = parse_expression(&mut lexer, Precedence::Lowest, &mut SymbolMap::new());

        assert!(matches!(expression, Ok(source::Expression::Infix(_))));
        if let Ok(source::Expression::Infix(source::Infix {
            left,
            operation: source::InfixOperation::Plus(_),
            right,
        })) = expression
        {
            assert!(matches!(
                *left,
                source::Expression::Integer(source::Integer { value: 3, .. })
            ));
            assert!(matches!(
                *right,
                source::Expression::Integer(source::Integer { value: 4, .. })
            ));
        }
    }

    #[test]
    fn multi_addition() {
        let mut lexer = Lexer::new(Source::new("test", "3 + 4 + 10".chars()));
        let expression = parse_expression(&mut lexer, Precedence::Lowest, &mut SymbolMap::new());

        assert!(matches!(expression, Ok(source::Expression::Infix(_))));
        if let Ok(source::Expression::Infix(source::Infix {
            left,
            operation: source::InfixOperation::Plus(_),
            right,
        })) = expression
        {
            assert!(matches!(*left, source::Expression::Infix(_)));
            if let source::Expression::Infix(source::Infix {
                left,
                operation: source::InfixOperation::Plus(_),
                right,
            }) = *left
            {
                assert!(matches!(
                    *left,
                    source::Expression::Integer(source::Integer { value: 3, .. })
                ));
                assert!(matches!(
                    *right,
                    source::Expression::Integer(source::Integer { value: 4, .. })
                ));
            }

            assert!(matches!(
                *right,
                source::Expression::Integer(source::Integer { value: 10, .. })
            ));
        }
    }
}
