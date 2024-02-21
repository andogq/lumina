use std::iter::Peekable;

use crate::{
    ast::{Expression, Identifier, LetStatement, Program, Statement},
    lexer::Lexer,
    token::Token,
};

pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(mut lexer: Lexer<'a>) -> Self {
        Self {
            lexer: lexer.peekable(),
        }
    }

    pub fn parse_program(&mut self) -> Option<Program> {
        let mut statements = Vec::new();

        while !self
            .lexer
            .peek()
            .map(|token| matches!(token, Token::EOF(_)))
            .unwrap_or(true)
        {
            statements.push(self.parse_statement()?);
        }

        Some(Program { statements })
    }

    pub fn parse_statement(&mut self) -> Option<Statement> {
        match self.lexer.peek()? {
            Token::Let(_) => Some(Statement::Let(self.parse_let_statement()?)),
            _ => None,
        }
    }

    pub fn parse_let_statement(&mut self) -> Option<LetStatement> {
        let let_token = self.lexer.next().and_then(|token| {
            if let Token::Let(let_token) = token {
                Some(let_token)
            } else {
                None
            }
        })?;

        let name = self.lexer.next().and_then(|token| {
            if let Token::Ident(name_ident_token) = token {
                Some(Identifier {
                    value: name_ident_token.literal.clone(),
                    ident_token: name_ident_token,
                })
            } else {
                None
            }
        })?;

        let assign_token = self.lexer.next().and_then(|token| {
            if let Token::Assign(assign_token) = token {
                Some(assign_token)
            } else {
                None
            }
        })?;

        // TODO: Read expression instead of skipping to semicolon
        while self
            .lexer
            .next_if(|token| !matches!(token, Token::Semicolon(_)))
            .is_some()
        {}

        let semicolon_token = self.lexer.next().and_then(|token| {
            if let Token::Semicolon(semicolon_token) = token {
                Some(semicolon_token)
            } else {
                None
            }
        })?;

        Some(LetStatement {
            let_token,
            name,
            value: Expression,
        })
    }
}

#[cfg(test)]
mod test {
    use crate::ast::Statement;

    use super::*;

    #[test]
    fn let_statements() {
        let input = r#"let x = 5;
        let y = 10;
        let foobar = 838383;
        "#;

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        let statements = program
            .into_iter()
            .flat_map(|program| program.statements)
            .filter_map(|statement| {
                if let Statement::Let(let_statement) = statement {
                    Some(let_statement.name.value)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        assert_eq!(
            vec!["x".to_string(), "y".to_string(), "foobar".to_string()],
            statements
        )
    }
}
