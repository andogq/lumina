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

pub fn parse_statement(
    lexer: &mut Lexer,
    symbols: &mut SymbolMap,
) -> Result<Statement, ParseError> {
    let mut expecting_semicolon = true;

    let statement = match lexer.peek_token() {
        Token::Return(return_token) => {
            // Parse as return statement
            lexer.next_token();

            Statement::Return(ReturnStatement {
                value: parse_expression(lexer, Precedence::Lowest, symbols)?,
                span: return_token.span,
            })
        }
        Token::Let(let_token) => {
            // let token
            lexer.next_token();

            // variable binding
            let name = match lexer.next_token() {
                Token::Ident(name) => name,
                token => {
                    return Err(ParseError::ExpectedToken {
                        found: Box::new(token),
                        expected: Box::new(Token::Ident(Default::default())),
                        reason: "ident must follow let binding".to_string(),
                    });
                }
            };

            // equals sign
            match lexer.next_token() {
                Token::Equals(_) => (),
                token => {
                    return Err(ParseError::ExpectedToken {
                        found: Box::new(token),
                        expected: Box::new(Token::Equals(Default::default())),
                        reason: "equals sign must follow ident".to_string(),
                    });
                }
            };

            // value
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
                implicit_return: if matches!(lexer.peek_token(), Token::Semicolon(_)) {
                    false
                } else {
                    expecting_semicolon = false;

                    true
                },
            })
        }
    };

    if expecting_semicolon {
        match lexer.next_token() {
            Token::Semicolon(_) => (),
            token => {
                return Err(ParseError::ExpectedToken {
                    found: Box::new(token),
                    expected: Box::new(Token::Equals(Default::default())),
                    reason: "semicolon must follow statement".to_string(),
                });
            }
        };
    }

    Ok(statement)
}
