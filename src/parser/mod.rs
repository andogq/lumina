use std::iter::Peekable;

use crate::{
    ast::{
        Expression, Identifier, IntegerLiteral, LetStatement, PrefixExpression, PrefixToken,
        Program, ReturnStatement, Statement,
    },
    lexer::Lexer,
    token::Token,
};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
}

pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
    errors: Vec<String>,
}

impl<'a> Parser<'a> {
    pub fn new(mut lexer: Lexer<'a>) -> Self {
        Self {
            lexer: lexer.peekable(),
            errors: Vec::new(),
        }
    }

    pub fn parse_program(&mut self) -> Program {
        let mut statements = Vec::new();

        while !self
            .lexer
            .peek()
            .map(|token| matches!(token, Token::EOF(_)))
            .unwrap_or(true)
        {
            match self.parse_statement() {
                Ok(statment) => {
                    statements.push(statment);
                }
                Err(error) => {
                    self.errors.push(error);
                    self.lexer.next();
                }
            }
        }

        Program { statements }
    }

    pub fn parse_statement(&mut self) -> Result<Statement, String> {
        match self
            .lexer
            .peek()
            .ok_or_else(|| "expected statement to follow".to_string())?
        {
            Token::Let(_) => Ok(Statement::Let(self.parse_let_statement()?)),
            Token::Return(_) => Ok(Statement::Return(self.parse_return_statement()?)),
            _ => Ok(Statement::Expression(self.parse_expression_statement()?)),
        }
    }

    pub fn parse_let_statement(&mut self) -> Result<LetStatement, String> {
        let let_token = self
            .lexer
            .next()
            .and_then(|token| {
                if let Token::Let(let_token) = token {
                    Some(let_token)
                } else {
                    None
                }
            })
            .ok_or_else(|| "expected `let` token".to_string())?;

        let name = self
            .lexer
            .next()
            .and_then(|token| {
                if let Token::Ident(name_ident_token) = token {
                    Some(Identifier {
                        value: name_ident_token.literal.clone(),
                        ident_token: name_ident_token,
                    })
                } else {
                    None
                }
            })
            .ok_or_else(|| "expected `ident` token".to_string())?;

        let assign_token = self
            .lexer
            .next()
            .and_then(|token| {
                if let Token::Assign(assign_token) = token {
                    Some(assign_token)
                } else {
                    None
                }
            })
            .ok_or_else(|| "expected `assign` token".to_string())?;

        // TODO: Read expression instead of skipping to semicolon
        while self
            .lexer
            .next_if(|token| !matches!(token, Token::Semicolon(_)))
            .is_some()
        {}

        let semicolon_token = self
            .lexer
            .next()
            .and_then(|token| {
                if let Token::Semicolon(semicolon_token) = token {
                    Some(semicolon_token)
                } else {
                    None
                }
            })
            .ok_or_else(|| "expected `semicolon` token".to_string())?;

        Ok(LetStatement {
            let_token,
            name,
            value: todo!(),
        })
    }

    pub fn parse_return_statement(&mut self) -> Result<ReturnStatement, String> {
        let return_token = self
            .lexer
            .next()
            .and_then(|token| {
                if let Token::Return(return_token) = token {
                    Some(return_token)
                } else {
                    None
                }
            })
            .ok_or_else(|| "expected `return` token".to_string())?;

        // TODO: Read expression instead of skipping to semicolon
        while self
            .lexer
            .next_if(|token| !matches!(token, Token::Semicolon(_)))
            .is_some()
        {}

        let semicolon_token = self
            .lexer
            .next()
            .and_then(|token| {
                if let Token::Semicolon(semicolon_token) = token {
                    Some(semicolon_token)
                } else {
                    None
                }
            })
            .ok_or_else(|| "expected `semicolon` token".to_string())?;

        Ok(ReturnStatement {
            return_token,
            value: todo!(),
        })
    }

    pub fn parse_expression_statement(&mut self) -> Result<Expression, String> {
        let expression = self.parse_expression(Precedence::Lowest)?;

        // Advance past semicolon, if present
        self.lexer
            .next_if(|token| matches!(token, Token::Semicolon(_)));

        Ok(expression)
    }

    pub fn parse_identifier(&mut self) -> Result<Identifier, String> {
        self.lexer
            .next()
            .and_then(|token| {
                if let Token::Ident(ident_token) = token {
                    Some(Identifier {
                        value: ident_token.literal.clone(),
                        ident_token,
                    })
                } else {
                    None
                }
            })
            .ok_or_else(|| "expected identifier".to_string())
    }

    pub fn parse_integer_literal(&mut self) -> Result<IntegerLiteral, String> {
        let int_token = self
            .lexer
            .next()
            .and_then(|token| {
                if let Token::Int(int_token) = token {
                    Some(int_token)
                } else {
                    None
                }
            })
            .ok_or_else(|| "expected integer".to_string())?;

        Ok(IntegerLiteral {
            value: int_token
                .literal
                .parse::<i64>()
                .map_err(|e| e.to_string())?,
            token: int_token,
        })
    }

    pub fn parse_prefix_expression(&mut self) -> Result<PrefixExpression, String> {
        let (prefix_token, operator) = match self
            .lexer
            .next()
            .ok_or_else(|| "expected prefix operator".to_string())?
        {
            Token::Plus(token) => Ok((PrefixToken::Plus(token), "+".to_string())),
            Token::Minus(token) => Ok((PrefixToken::Minus(token), "-".to_string())),
            Token::Bang(token) => Ok((PrefixToken::Bang(token), "!".to_string())),
            token => Err(format!("unknown prefix operator: {token:?}")),
        }?;

        let right = self.parse_expression(Precedence::Prefix)?;

        Ok(PrefixExpression {
            prefix_token,
            operator,
            right: Box::new(right),
        })
    }

    pub fn parse_prefix(&mut self) -> Result<Expression, String> {
        match self
            .lexer
            .peek()
            .ok_or_else(|| "expected token for prefix expression".to_string())?
        {
            Token::Ident(_) => Ok(Expression::Identifier(self.parse_identifier()?)),
            Token::Int(_) => Ok(Expression::Integer(self.parse_integer_literal()?)),
            Token::Bang(_) | Token::Minus(_) => {
                Ok(Expression::Prefix(self.parse_prefix_expression()?))
            }
            token => Err(format!("no prefix parse function found for {token:?}")),
        }
    }

    pub fn parse_expression(&mut self, precedence: Precedence) -> Result<Expression, String> {
        let left_expression = self.parse_prefix()?;

        Ok(left_expression)
    }
}

#[cfg(test)]
mod test {
    use crate::ast::{PrefixExpression, Statement};

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
            .statements
            .into_iter()
            .filter_map(|statement| {
                if let Statement::Let(let_statement) = statement {
                    Some(let_statement.name.value)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        assert!(parser.errors.is_empty());

        assert_eq!(
            vec!["x".to_string(), "y".to_string(), "foobar".to_string()],
            statements
        );
    }

    #[test]
    fn return_statements() {
        let input = r#"return 5;
        return 10;
        return 993322;
        "#;

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();

        assert!(parser.errors.is_empty());
        assert_eq!(program.statements.len(), 3);
    }

    #[test]
    fn identifier_expression() {
        let input = "foobar;";

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();

        assert!(parser.errors.is_empty());

        assert_eq!(program.statements.len(), 1);

        let statement = &program.statements[0];
        assert!(matches!(
            statement,
            Statement::Expression(Expression::Identifier(_))
        ));

        if let Statement::Expression(Expression::Identifier(identifier)) = statement {
            assert_eq!(identifier.value, "foobar".to_string());
        }
    }

    #[test]
    fn integer_literal_expression() {
        let input = "5;";

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();

        assert!(parser.errors.is_empty());

        assert_eq!(program.statements.len(), 1);

        let statement = &program.statements[0];
        assert!(matches!(
            statement,
            Statement::Expression(Expression::Integer(_))
        ));

        if let Statement::Expression(Expression::Integer(integer)) = statement {
            assert_eq!(integer.value, 5);
        }
    }

    #[test]
    fn parse_prefix_expressions() {
        [("!5;", "!", 5), ("-15;", "-", 15)].into_iter().for_each(
            |(input, operator, integer_value)| {
                let lexer = Lexer::new(input);
                let mut parser = Parser::new(lexer);

                let program = parser.parse_program();

                assert!(dbg!(parser.errors).is_empty());

                assert_eq!(program.statements.len(), 1);

                let statement = &program.statements[0];
                assert!(matches!(
                    statement,
                    Statement::Expression(Expression::Prefix(_))
                ));

                if let Statement::Expression(Expression::Prefix(prefix_expression)) = statement {
                    assert_eq!(prefix_expression.operator, operator);
                    assert!(matches!(*prefix_expression.right, Expression::Integer(_)));

                    if let Expression::Integer(ref integer_literal) = *prefix_expression.right {
                        assert_eq!(integer_value, integer_literal.value);
                    }
                }
            },
        );
    }
}
