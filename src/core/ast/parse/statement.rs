use crate::core::{
    ast::{ExpressionStatement, ReturnStatement, Statement},
    lexer::{token::Token, Lexer},
};

use super::{
    expression::{parse_expression, Precedence},
    ParseError,
};

pub fn parse_statement<S>(lexer: &mut Lexer<S>) -> Result<Statement, ParseError>
where
    S: Iterator<Item = char>,
{
    let expecting_semicolon = true;

    let statement = match lexer.peek() {
        Token::Return(_) => {
            // Parse as return statement
            lexer.next();

            Statement::Return(ReturnStatement {
                value: parse_expression(lexer, Precedence::Lowest)?,
            })
        }
        _ => {
            // Parse expression
            Statement::Expression(ExpressionStatement {
                expression: parse_expression(lexer, Precedence::Lowest)?,
            })
        }
    };

    if expecting_semicolon && !matches!(lexer.next(), Token::Semicolon(_)) {
        return Err(ParseError::ExpectedToken(";".to_string()));
    }

    Ok(statement)
}
