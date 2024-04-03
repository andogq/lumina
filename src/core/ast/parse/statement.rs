use crate::core::{
    ast::{symbol::SymbolMap, ExpressionStatement, LetStatement, ReturnStatement, Statement},
    lexer::{token::Token, Lexer},
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
    let expecting_semicolon = true;

    let statement = match lexer.peek() {
        Token::Return(_) => {
            // Parse as return statement
            lexer.next();

            Statement::Return(ReturnStatement {
                value: parse_expression(lexer, Precedence::Lowest, symbols)?,
            })
        }
        Token::Let(_) => {
            lexer.next();

            let Token::Ident(name) = lexer.next() else {
                return Err(ParseError::ExpectedToken("ident".to_string()));
            };

            if !matches!(lexer.next(), Token::Equals(_)) {
                return Err(ParseError::ExpectedToken("=".to_string()));
            }

            Statement::Let(LetStatement {
                name: symbols.get(name.literal),
                value: parse_expression(lexer, Precedence::Lowest, symbols)?,
            })
        }
        _ => {
            // Parse expression
            Statement::Expression(ExpressionStatement {
                expression: parse_expression(lexer, Precedence::Lowest, symbols)?,
            })
        }
    };

    if expecting_semicolon && !matches!(lexer.next(), Token::Semicolon(_)) {
        return Err(ParseError::ExpectedToken(";".to_string()));
    }

    Ok(statement)
}
