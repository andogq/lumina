use crate::{
    core::{
        ast::{ExpressionStatement, LetStatement, ReturnStatement, Statement},
        lexer::token::Token,
    },
    util::source::Spanned,
};

use super::{
    expression::{parse_expression, Precedence},
    ParseCtx, ParseError,
};

pub fn parse_statement(ctx: &mut ParseCtx) -> Result<Statement, ParseError> {
    let mut expecting_semicolon = true;

    let statement = match ctx.lexer.peek_token() {
        Token::Return(return_token) => {
            // Parse as return statement
            ctx.lexer.next_token();

            Statement::Return(ReturnStatement {
                value: parse_expression(ctx, Precedence::Lowest)?,
                span: return_token.span,
            })
        }
        Token::Let(let_token) => {
            // let token
            ctx.lexer.next_token();

            // variable binding
            let name = match ctx.lexer.next_token() {
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
            match ctx.lexer.next_token() {
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
            let value = parse_expression(ctx, Precedence::Lowest)?;

            Statement::Let(LetStatement {
                span: let_token.span().to(&value),
                name: ctx.symbols.get(name.literal),
                value,
            })
        }
        _ => {
            // Parse expression
            let expression = parse_expression(ctx, Precedence::Lowest)?;
            Statement::Expression(ExpressionStatement {
                span: expression.span().clone(),
                expression,
                implicit_return: if matches!(ctx.lexer.peek_token(), Token::Semicolon(_)) {
                    false
                } else {
                    expecting_semicolon = false;

                    true
                },
            })
        }
    };

    if expecting_semicolon {
        match ctx.lexer.next_token() {
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
