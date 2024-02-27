use std::fmt::{Display, Formatter};

use crate::{
    ast::{AstNode, ParseNode},
    code::Instruction,
    interpreter::{environment::Environment, return_value::Return},
    lexer::Lexer,
    object::{NullObject, Object},
    return_value,
    token::Token,
};

use super::Statement;

#[derive(Clone, Debug, PartialEq)]
pub struct BlockStatement {
    statements: Vec<Statement>,
}

impl AstNode for BlockStatement {
    fn evaluate(&self, env: Environment) -> Return<Object> {
        let mut result = Object::Null(NullObject);

        for statement in &self.statements {
            result = return_value!(statement.evaluate(env.clone()));
        }

        Return::Implicit(result)
    }

    fn compile(
        &self,
        register_constant: &mut impl FnMut(Object) -> u32,
    ) -> Result<Vec<Instruction>, String> {
        todo!()
    }
}

impl<S> ParseNode<S> for BlockStatement
where
    S: Iterator<Item = char>,
{
    fn parse(tokens: &mut Lexer<S>) -> Result<Self, String> {
        let Token::LeftBrace(_l_brace) = tokens.next() else {
            return Err("expected opening brace".to_string());
        };

        let mut statements = Vec::new();

        while !matches!(tokens.peek(), Token::RightBrace(_) | Token::EOF(_)) {
            statements.push(Statement::parse(tokens)?);
        }

        let Token::RightBrace(_r_brace) = tokens.next() else {
            return Err("expected closing brace".to_string());
        };

        Ok(Self { statements })
    }
}

impl Display for BlockStatement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ {} }}",
            self.statements
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}
