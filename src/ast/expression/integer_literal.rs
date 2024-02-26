use std::fmt::{Display, Formatter};

use crate::{
    ast::{AstNode, ParseNode},
    code::{Instruction, OpConstant},
    interpreter::{environment::Environment, return_value::Return},
    lexer::Lexer,
    object::{IntegerObject, Object},
    token::{IntToken, Token},
};

#[derive(Clone, Debug, PartialEq)]
pub struct IntegerLiteral {
    pub token: IntToken,
    pub value: i64,
}

impl IntegerLiteral {
    pub fn new(value: i64) -> Self {
        Self {
            token: IntToken {
                literal: value.to_string(),
                ..Default::default()
            },
            value,
        }
    }

    pub fn as_object(&self) -> Object {
        Object::Integer(IntegerObject { value: self.value })
    }
}

impl AstNode for IntegerLiteral {
    fn evaluate(&self, _env: Environment) -> Return<Object> {
        Return::Implicit(self.as_object())
    }

    fn compile(&self, mut register_constant: impl FnMut(Object) -> u32) -> Result<Vec<u8>, String> {
        let id = register_constant(self.as_object());

        Ok(OpConstant(id).bytes())
    }
}

impl<S> ParseNode<S> for IntegerLiteral
where
    S: Iterator<Item = char>,
{
    fn parse(lexer: &mut Lexer<S>) -> Result<Self, String> {
        let Token::Int(int_token) = lexer.next() else {
            return Err("expected integer".to_string());
        };

        Ok(IntegerLiteral {
            value: int_token
                .literal
                .parse::<i64>()
                .map_err(|e| e.to_string())?,
            token: int_token,
        })
    }
}

impl Display for IntegerLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}

#[cfg(test)]
mod test {
    use crate::{compiler::compile, token::SemicolonToken};

    use super::*;

    #[test]
    fn parse_single_digit() {
        let mut lexer = Lexer::from_tokens([
            Token::Int(IntToken {
                literal: "1".to_string(),
                ..Default::default()
            }),
            Token::Semicolon(SemicolonToken::default()),
        ]);

        assert!(matches!(
            IntegerLiteral::parse(&mut lexer),
            Ok(IntegerLiteral { value: 1, .. })
        ));

        assert!(matches!(lexer.next(), Token::Semicolon(_)));
    }

    #[test]
    fn parse_multiple_digits() {
        let mut lexer = Lexer::from_tokens([
            Token::Int(IntToken {
                literal: "12345".to_string(),
                ..Default::default()
            }),
            Token::Semicolon(SemicolonToken::default()),
        ]);

        assert!(matches!(
            IntegerLiteral::parse(&mut lexer),
            Ok(IntegerLiteral { value: 12345, .. })
        ));

        assert!(matches!(lexer.next(), Token::Semicolon(_)));
    }

    #[test]
    fn reject_large_number() {
        let mut lexer = Lexer::from_tokens([
            Token::Int(IntToken {
                literal: u64::MAX.to_string(),
                ..Default::default()
            }),
            Token::Semicolon(SemicolonToken::default()),
        ]);

        assert!(matches!(IntegerLiteral::parse(&mut lexer), Err(_)));

        assert!(matches!(lexer.next(), Token::Semicolon(_)));
    }

    #[test]
    fn reject_non_number() {
        assert!(matches!(
            IntegerLiteral::parse(&mut Lexer::from_tokens([Token::Semicolon(
                SemicolonToken::default()
            )])),
            Err(_)
        ));
    }

    #[test]
    fn evaluate() {
        assert!(matches!(
            IntegerLiteral::new(5).evaluate(Environment::new()),
            Return::Implicit(Object::Integer(IntegerObject { value: 5 }))
        ));
    }

    #[test]
    fn int_compile() {
        let bytecode = compile(IntegerLiteral::new(5)).unwrap();

        assert_eq!(bytecode.constants.len(), 1);

        // Make sure the stored constants value is correct
        assert!(matches!(bytecode.constants[0], Object::Integer(_)));
        if let Object::Integer(int) = &bytecode.constants[0] {
            assert_eq!(int.value, 5);
        }

        assert_eq!(
            bytecode.instructions,
            [
                0x00, // Opcode
                0x00, 0x00, 0x00, 0x00, // Constant index
            ]
        );
    }
}
