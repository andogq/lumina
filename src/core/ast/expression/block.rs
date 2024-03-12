use std::fmt::{Display, Formatter};

use crate::core::{
    ast::{ParseNode, Statement},
    lexer::{Lexer, Token},
};

#[derive(Clone, Debug, PartialEq)]
pub struct Block {
    pub statements: Vec<Statement>,
}

impl<S> ParseNode<S> for Block
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

impl Display for Block {
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

#[cfg(test)]
mod test {
    use crate::{
        assert_pattern,
        core::ast::{Expression, IntegerLiteral},
        test_parser,
    };

    use super::*;

    #[test]
    fn parse() {
        test_parser!(Block, "{ 1 }", Block { statements }, {
            assert_pattern!(
                statements[0],
                Statement::Expression {
                    expression: Expression::Integer(IntegerLiteral { value: 1, .. }),
                    semicolon: false
                }
            );
        });
    }
}
