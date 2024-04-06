use crate::{
    core::{
        ast::{ExpressionStatement, LetStatement, ReturnStatement, Statement},
        lexer::{token::Token, Lexer},
        symbol::SymbolMap,
    },
    util::source::Spanned,
};

use super::{
    expression::{parse_expression, Precedence},
    ParseError,
};

pub fn parse_statement<S>(
    lexer: &mut Lexer<S>,
    symbols: &mut SymbolMap,
) -> Result<Statement, ParseError>
where
    S: Iterator<Item = char>,
{
    let mut expecting_semicolon = true;

    let statement = match lexer.peek() {
        Token::Return(return_token) => {
            // Parse as return statement
            lexer.next();

            Statement::Return(ReturnStatement {
                value: parse_expression(lexer, Precedence::Lowest, symbols)?,
                span: return_token.span,
            })
        }
        Token::Let(let_token) => {
            lexer.next();

            let Token::Ident(name) = lexer.next() else {
                return Err(ParseError::ExpectedToken("ident".to_string()));
            };

            if !matches!(lexer.next(), Token::Equals(_)) {
                return Err(ParseError::ExpectedToken("=".to_string()));
            }

            let value = parse_expression(lexer, Precedence::Lowest, symbols)?;

            Statement::Let(LetStatement {
                span: let_token.span().to(&value),
                name: symbols.get(name.literal),
                value,
            })
        }
        _ => {
            // Parse expression
            let expression = parse_expression(lexer, Precedence::Lowest, symbols)?;
            Statement::Expression(ExpressionStatement {
                span: expression.span().clone(),
                expression,
                implicit_return: if matches!(lexer.peek(), Token::Semicolon(_)) {
                    false
                } else {
                    expecting_semicolon = false;

                    true
                },
            })
        }
    };

    if expecting_semicolon && !matches!(lexer.next(), Token::Semicolon(_)) {
        return Err(ParseError::ExpectedToken(";".to_string()));
    }

    Ok(statement)
}
