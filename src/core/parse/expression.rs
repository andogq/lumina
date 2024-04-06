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

fn parse_prefix<S>(
    lexer: &mut Lexer<S>,
    symbols: &mut SymbolMap,
) -> Result<ast::Expression, ParseError>
where
    S: Iterator<Item = char>,
{
    let mut advance = true;

    let prefix = match lexer.peek() {
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
            lexer.next();

            let mut span = token.span;

            let condition = parse_expression(lexer, Precedence::Lowest, symbols)?;

            let success = parse_block(lexer, symbols)?;
            span = span.to(&success);

            let otherwise = if matches!(lexer.peek(), Token::Else(_)) {
                lexer.next();

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
        lexer.next();
    }

    prefix
}

pub fn parse_expression<S>(
    lexer: &mut Lexer<S>,
    precedence: Precedence,
    symbols: &mut SymbolMap,
) -> Result<ast::Expression, ParseError>
where
    S: Iterator<Item = char>,
{
    let mut left = parse_prefix(lexer, symbols)?;

    while !matches!(lexer.peek(), Token::EOF(_)) && precedence < Precedence::of(&lexer.peek()) {
        if let Ok(operation) = ast::InfixOperation::try_from(lexer.peek()) {
            let token = lexer.next();
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
    use crate::util::source::Source;

    use super::*;

    #[test]
    fn simple_addition() {
        let mut lexer = Lexer::new(Source::new("test", "3 + 4".chars()));
        let expression = parse_expression(&mut lexer, Precedence::Lowest, &mut SymbolMap::new());

        assert!(matches!(expression, Ok(ast::Expression::Infix(_))));
        if let Ok(ast::Expression::Infix(ast::Infix {
            left,
            operation: ast::InfixOperation::Plus(_),
            right,
            ..
        })) = expression
        {
            assert!(matches!(
                *left,
                ast::Expression::Integer(ast::Integer { value: 3, .. })
            ));
            assert!(matches!(
                *right,
                ast::Expression::Integer(ast::Integer { value: 4, .. })
            ));
        }
    }

    #[test]
    fn multi_addition() {
        let mut lexer = Lexer::new(Source::new("test", "3 + 4 + 10".chars()));
        let expression = parse_expression(&mut lexer, Precedence::Lowest, &mut SymbolMap::new());

        assert!(matches!(expression, Ok(ast::Expression::Infix(_))));
        if let Ok(ast::Expression::Infix(ast::Infix {
            left,
            operation: ast::InfixOperation::Plus(_),
            right,
            ..
        })) = expression
        {
            assert!(matches!(*left, ast::Expression::Infix(_)));
            if let ast::Expression::Infix(ast::Infix {
                left,
                operation: ast::InfixOperation::Plus(_),
                right,
                ..
            }) = *left
            {
                assert!(matches!(
                    *left,
                    ast::Expression::Integer(ast::Integer { value: 3, .. })
                ));
                assert!(matches!(
                    *right,
                    ast::Expression::Integer(ast::Integer { value: 4, .. })
                ));
            }

            assert!(matches!(
                *right,
                ast::Expression::Integer(ast::Integer { value: 10, .. })
            ));
        }
    }
}
