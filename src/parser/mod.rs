use std::iter::Peekable;

use crate::{
    ast::{
        BooleanLiteral, BooleanToken, Expression, Identifier, InfixExpression, InfixOperatorToken,
        IntegerLiteral, LetStatement, PrefixExpression, PrefixToken, Program, ReturnStatement,
        Statement,
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

impl Precedence {
    pub fn of(token: &Token) -> Self {
        match token {
            Token::Plus(token) => Precedence::Sum,
            Token::Minus(token) => Precedence::Sum,
            Token::Asterisk(token) => Precedence::Product,
            Token::Slash(token) => Precedence::Product,
            Token::LeftAngle(token) => Precedence::LessGreater,
            Token::RightAngle(token) => Precedence::LessGreater,
            Token::Eq(token) => Precedence::Equals,
            Token::NotEq(token) => Precedence::Equals,
            token => Precedence::Lowest,
        }
    }
}

pub trait Node: Sized {
    fn parse(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<Self, String>;
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

    pub fn parse<N: Node>(&mut self) -> Result<N, String> {
        N::parse(&mut self.lexer)
    }

    pub fn parse_program(&mut self) -> Program {
        let mut statements = Vec::new();

        while !self
            .lexer
            .peek()
            .map(|token| matches!(token, Token::EOF(_)))
            .unwrap_or(true)
        {
            match self.parse::<Statement>() {
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
        [("!5;", "!", 5), ("-15;", "-", 15), ("+10;", "+", 10)]
            .into_iter()
            .for_each(|(input, operator, integer_value)| {
                let lexer = Lexer::new(input);
                let mut parser = Parser::new(lexer);

                let program = parser.parse_program();

                assert!(parser.errors.is_empty());

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
            });
    }

    #[test]
    fn parse_infix_expressions() {
        [
            ("5 + 5", 5, "+", 5),
            ("5 - 5", 5, "-", 5),
            ("5 * 5", 5, "*", 5),
            ("5 / 5", 5, "/", 5),
            ("5 > 5", 5, ">", 5),
            ("5 < 5", 5, "<", 5),
            ("5 == 5", 5, "==", 5),
            ("5 != 5", 5, "!=", 5),
        ]
        .into_iter()
        .for_each(|(input, left, operator, right)| {
            let lexer = Lexer::new(input);
            let mut parser = Parser::new(lexer);

            let program = parser.parse_program();

            assert!(dbg!(parser.errors).is_empty());

            assert_eq!(program.statements.len(), 1);

            let statement = &program.statements[0];
            assert!(matches!(
                statement,
                Statement::Expression(Expression::Infix(_))
            ));

            if let Statement::Expression(Expression::Infix(infix_expression)) = statement {
                assert_eq!(infix_expression.operator, operator);
                assert!(matches!(*infix_expression.left, Expression::Integer(_)));
                assert!(matches!(*infix_expression.right, Expression::Integer(_)));

                if let Expression::Integer(ref integer_literal) = *infix_expression.left {
                    assert_eq!(left, integer_literal.value);
                }
                if let Expression::Integer(ref integer_literal) = *infix_expression.right {
                    assert_eq!(right, integer_literal.value);
                }
            }
        });
    }

    #[test]
    fn parse_boolean_expression() {
        [("true;", true), ("false;", false)]
            .into_iter()
            .for_each(|(input, value)| {
                let lexer = Lexer::new(input);
                let mut parser = Parser::new(lexer);

                let program = parser.parse_program();

                assert!(parser.errors.is_empty());

                assert_eq!(program.statements.len(), 1);

                let statement = &program.statements[0];
                assert!(matches!(
                    statement,
                    Statement::Expression(Expression::Boolean(_))
                ));

                if let Statement::Expression(Expression::Boolean(integer)) = statement {
                    assert_eq!(integer.value, value);
                }
            })
    }
}
